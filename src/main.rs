use image::{Rgba, RgbaImage};
use num_complex::Complex;
use rayon::prelude::*;
use std::fs::{read_to_string, write};
use std::path as Dir;
use std::process::Command;

struct Color {
    r: u32,
    g: u32,
    b: u32,
}

// TODO clean up
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const MAX_ITER: u32 = 150;
const OUTPUT_VIDEO_FILE: &str = "julia_set_video.mp4";
const FRAME_RATE: u32 = 30;
const _VIDEO_RESOLUTION: (u16, u16) = (1080, 1200);
//
const FRAME_COUNT: u32 = 1000;
const COLOR_START: Color = Color { r: 4, g: 1, b: 9 };
const COLOR_STEP: Color = Color { r: 1, g: 1, b: 1 };
const C_START: Complex<f64> = Complex {
    re: -0.79,
    im: 0.155,
};
const C_STEP: Complex<f64> = Complex {
    re: 0.00008,
    im: 0.00008,
};
const SVG_WIDTH: u16 = 3600;
const SVG_HEIGHT: u16 = 3600;

fn julia_set_pixel(x: usize, y: usize, color: &Color, c: &Complex<f64>) -> Rgba<u8> {
    let mut z = Complex {
        re: (x as f64 - WIDTH as f64 / 2.0) / (WIDTH as f64 / 4.0),
        im: (y as f64 - HEIGHT as f64 / 2.0) / (HEIGHT as f64 / 4.0),
    };
    let mut iter = 0;

    // iteration step
    while iter < MAX_ITER {
        z = z * z + c;
        if z.norm() > 2.0 {
            break;
        }
        iter += 1;
    }

    // Map the iteration count to a custom color
    let color = match iter {
        MAX_ITER => Rgba([0, 0, 0, 255]), // Black for points in the set
        _ => Rgba([
            ((iter * color.r) * 250 / MAX_ITER) as u8,
            ((iter * color.g) * 250 / MAX_ITER) as u8,
            ((iter * color.b) * 250 / MAX_ITER) as u8,
            250 as u8,
        ]), // Custom color mapping
    };

    color
}

fn gen_image(color: &Color, c: &Complex<f64>) -> RgbaImage {
    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let color = julia_set_pixel(x as usize, y as usize, color, c);
            *pixel = color;
        });

    img
}

fn svg_resize(
    svg_path: &Dir::Path,
    width: u16,
    height: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let svg_content = read_to_string(svg_path)?;

    // Modify the width and height attributes
    let modified_svg_content = svg_content
        .replace(r#"width="800""#, &format!(r#"width="{}""#, width))
        .replace(r#"height="800""#, &format!(r#"height="{}""#, height))
        .replace(
            r#"<svg"#,
            &format!(r#"<svg viewBox="0 0 {} {}""#, WIDTH, HEIGHT),
        );

    write(svg_path, modified_svg_content)?;

    Ok(())
}

fn gen_video() {
    // generate image frames
    for i in 0..FRAME_COUNT {
        let color = Color {
            r: (COLOR_START.r as u32 + i % 32 as u32 * COLOR_STEP.r as u32) as u32,
            g: (COLOR_START.g as u32 + i % 32 as u32 * COLOR_STEP.g as u32) as u32,
            b: (COLOR_START.b as u32 + i % 32 as u32 * COLOR_STEP.b as u32) as u32,
        };

        let c = Complex {
            re: C_START.re + i as f64 * C_STEP.re,
            im: C_START.im + i as f64 * C_STEP.im,
        };

        let img = gen_image(&color, &c);

        let filename = format!("frame_{:04}.png", i);
        img.save(&filename).unwrap();

        // transform images to svg files
        Command::new("vtracer")
            .arg(format!("--input=frame_{:04}.png", i))
            .arg(format!("--output=frame_{:04}.svg", i))
            .output()
            .ok();

        let _ = svg_resize(
            &std::path::Path::new(&format!("frame_{:04}.svg", i)),
            SVG_WIDTH,
            SVG_HEIGHT,
        );
    }
    // remove mp4 files
    Command::new("sh")
        .arg("-c")
        .arg("rm -f *.mp4")
        .output()
        .ok();
    // Use ffmpeg to create a video from the images
    Command::new("ffmpeg")
        .args(&[
            "-framerate",
            &format!("{}", FRAME_RATE),
            "-i",
            "frame_%04d.svg", // Use the correct pattern for your resized SVG files
            "-c:v",
            "libx264",
            "-r",
            &format!("{}", FRAME_RATE),
            OUTPUT_VIDEO_FILE,
        ])
        .output()
        .expect("Failed to create video");
    // remove pngd and svgs
    Command::new("sh").arg("-c").arg("rm *.png").output().ok();
    Command::new("sh").arg("-c").arg("rm *.svg").output().ok();

    println!("Video saved as {}", OUTPUT_VIDEO_FILE);
}

fn gen_picture() -> RgbaImage {
    let img = gen_image(&COLOR_START, &C_START);
    img.save("1.png").unwrap();
    return img;
}

fn main() {
    // let _ = gen_picture();
    let _ = gen_video();
    return;
}
