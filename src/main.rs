use image::{Rgba, RgbaImage};
use num_complex::Complex;
use rayon::prelude::*;
use std::process::Command;

struct Color {
    r: u32,
    g: u32,
    b: u32,
}

// TODO clean up
const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const MAX_ITER: u32 = 50;
const OUTPUT_VIDEO_FILE: &str = "julia_set_video.mp4";
const FRAME_RATE: u32 = 30;
//
const FRAME_COUNT: u32 = 20;
const COLOR_START: Color = Color { r: 9, g: 1, b: 26 };
const COLOR_STEP: Color = Color { r: 1, g: 1, b: 1 };
const C_START: Complex<f64> = Complex { re: 0.8, im: 1.3 };
const C_STEP: Complex<f64> = Complex {
    re: 0.001,
    im: 0.001,
};

fn julia_set_pixel(x: usize, y: usize, color: &Color, c: &Complex<f64>) -> Rgba<u8> {
    let mut z = Complex {
        re: (x as f64 - WIDTH as f64 / 2.0) / (WIDTH as f64 / 4.0),
        im: (y as f64 - HEIGHT as f64 / 2.0) / (HEIGHT as f64 / 4.0),
    };
    let mut iter = 0;

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

fn generate_julia_set_image(color: &Color, c: &Complex<f64>) -> RgbaImage {
    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let color = julia_set_pixel(x as usize, y as usize, color, c);
            *pixel = color;
        });

    img
}

fn gen_video() {
    // generate image frames
    for i in 0..FRAME_COUNT {
        let color = Color {
            r: (COLOR_START.r as u32 + i % 30 as u32 * COLOR_STEP.r as u32) as u32,
            g: (COLOR_START.g as u32 + i % 3 as u32 * COLOR_STEP.g as u32) as u32,
            b: (COLOR_START.b as u32 + i % 40 as u32 * COLOR_STEP.b as u32) as u32,
        };

        let c = Complex {
            re: C_START.re + i as f64 * C_STEP.re,
            im: C_START.im + i as f64 * C_STEP.im,
        };

        let img = generate_julia_set_image(&color, &c);

        let filename = format!("frame_{:04}.png", i);
        img.save(&filename).unwrap();

        // transform images to svg files
        Command::new("vtracer")
            .arg(format!("--input=frame_{:04}.png", i))
            .arg(format!("--output=frame_{:04}.svg", i))
            .output()
            .ok();
    }
    // Use ffmpeg to create a video from the images
    // TODO use something else
    Command::new("sh")
        .arg("-c")
        .arg("rm -f *.mp4")
        .output()
        .ok();
    let r = Command::new("ffmpeg")
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
    dbg!(r);
    Command::new("sh").arg("-c").arg("rm *.png").output().ok();
    Command::new("sh").arg("-c").arg("rm *.svg").output().ok();

    println!("Video saved as {}", OUTPUT_VIDEO_FILE);
}

fn gen_picture() -> RgbaImage {
    let img = generate_julia_set_image(&COLOR_START, &C_START);
    img.save("1.png").unwrap();
    return img;
}

fn main() {
    // let _ = gen_picture();
    let _ = gen_video();
    return;
}
