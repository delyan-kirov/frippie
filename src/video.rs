pub mod params;

use image::{Rgba, RgbaImage};
use num_complex::Complex;
use params::*;
use rayon::prelude::*;
use std::fs::{read_to_string, write};
use std::path as Dir;
use std::process::Command;

const ZOOM_WIDTH: f64 = (WIDTH as f64 / 4.0) / ZOOM;
const ZOOM_HEIGHT: f64 = (WIDTH as f64 / 4.0) / ZOOM;
const BOUNDARY_MAX_ITER: usize = 100;
const ESCAPE_RADIUS: f64 = 2.0;

#[inline]
fn julia_set_pixel(x: usize, y: usize, color: &Color, c: &Complex<f64>) -> Rgba<u8> {
    let mut z = Complex {
        re: (x as f64 - WIDTH as f64 / 2.0) / ZOOM_WIDTH,
        im: (y as f64 - HEIGHT as f64 / 2.0) / ZOOM_HEIGHT,
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
    match iter {
        MAX_ITER => Rgba([0, 0, 0, 255]), // Black for points in the set
        _ => Rgba([
            ((iter * color.r) * 250 / MAX_ITER) as u8,
            ((iter * color.g) * 250 / MAX_ITER) as u8,
            ((iter * color.b) * 250 / MAX_ITER) as u8,
            250 as u8,
        ]), // Custom color mapping
    }
}

fn gen_image(color: &Color, c: &Complex<f64>) -> RgbaImage {
    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            *pixel = julia_set_pixel(x as usize, y as usize, color, c);
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
        .replace(
            &format!(r#"width="{}""#, WIDTH),
            &format!(r#"width="{}""#, width),
        )
        .replace(
            &format!(r#"height="{}""#, HEIGHT),
            &format!(r#"height="{}""#, height),
        )
        .replace(
            r#"<svg"#,
            &format!(r#"<svg viewBox="0 0 {} {}""#, WIDTH, HEIGHT),
        );

    write(svg_path, modified_svg_content)?;

    Ok(())
}

pub fn gen_video() {
    let color = Color {
        r: (COLOR_START.r as u32 as u32 * COLOR_STEP.r as u32) as u32,
        g: (COLOR_START.g as u32 as u32 * COLOR_STEP.g as u32) as u32,
        b: (COLOR_START.b as u32 as u32 * COLOR_STEP.b as u32) as u32,
    };
    // generate image frames
    (0..FRAME_COUNT).into_par_iter().for_each(|i| {
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
    });
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

fn _gen_picture() -> RgbaImage {
    let img = gen_image(&COLOR_START, &C_START);
    img.save("1.png").unwrap();
    return img;
}
