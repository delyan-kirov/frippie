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
const WIDTH: u32 = 5000;
const HEIGHT: u32 = 5000;
const MAX_ITER: u32 = 300;
const OUTPUT_VIDEO_FILE: &str = "julia_set_video.mp4";
const FRAME_RATE: u32 = 60;

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
            ((iter * color.r) * 255 / MAX_ITER) as u8,
            ((iter * color.g) * 255 / MAX_ITER) as u8,
            ((iter * color.b) * 255 / MAX_ITER) as u8,
            255 as u8,
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
    let frame_count = 1000; // Adjust as needed
    let color_start = Color { r: 9, g: 1, b: 26 };
    let color_step = Color { r: 1, g: 1, b: 1 };

    let c_start = Complex {
        re: -0.8,
        im: -0.156,
    };
    let c_step = Complex {
        re: 0.00001,
        im: 0.00001,
    };

    for i in 0..frame_count {
        let color = Color {
            r: (color_start.r as u32 + i % 30 as u32 * color_step.r as u32) as u32,
            g: (color_start.g as u32 + i % 3 as u32 * color_step.g as u32) as u32,
            b: (color_start.b as u32 + i % 40 as u32 * color_step.b as u32) as u32,
        };

        let c = Complex {
            re: c_start.re + i as f64 * c_step.re,
            im: c_start.im + i as f64 * c_step.im,
        };

        let img = generate_julia_set_image(&color, &c);

        let filename = format!("frame_{:04}.png", i);
        img.save(&filename).unwrap();
    }

    // Use ffmpeg to create a video from the images
    // TODO use something else
    Command::new("sh")
        .arg("-c")
        .arg("rm -f *.mp4")
        .output()
        .ok();
    Command::new("ffmpeg")
        .args(&[
            "-framerate",
            &format!("{}", FRAME_RATE),
            "-i",
            "./frame_%04d.png", // Use the full or relative path to your image files
            "-c:v",
            "libx264",
            "-r",
            &format!("{}", FRAME_RATE),
            OUTPUT_VIDEO_FILE,
        ])
        .output()
        .expect("Failed to create video");
    Command::new("sh").arg("-c").arg("rm *.png").output().ok();

    println!("Video saved as {}", OUTPUT_VIDEO_FILE);
}

fn gen_picture() -> RgbaImage {
    let c = Complex {
        re: -0.7,
        im: -0.27015,
    };
    let color = Color { r: 9, g: 3, b: 13 };
    let img = generate_julia_set_image(&color, &c);
    img.save("1.png").unwrap();
    return img;
}

fn main() {
    let _ = gen_picture();
    return;
}
