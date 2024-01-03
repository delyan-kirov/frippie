pub mod params;

use image::{Rgba, RgbaImage};
use num_complex::Complex;
use params::*;
use plotters::prelude::{self as Plt, IntoDrawingArea};
use rayon::prelude::*;
use std::fs::{read_to_string, write};
use std::path as Dir;
use std::process::Command;
use std::sync::{Arc, Mutex};

const ZOOM_WIDTH: f64 = (WIDTH as f64 / 4.0) / ZOOM;
const ZOOM_HEIGHT: f64 = (WIDTH as f64 / 4.0) / ZOOM;

#[inline]
fn gen_image(color: &Color, c: &Complex<f64>) -> RgbaImage {
    fn gen_pixel(x: usize, y: usize, color: &Color, c: &Complex<f64>) -> Rgba<u8> {
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
    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            *pixel = gen_pixel(x as usize, y as usize, color, c);
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
    let frames: Arc<Mutex<Vec<RgbaImage>>> = Arc::new(Mutex::new(Vec::new()));
    let color = Color {
        r: (COLOR_START.r as u32 as u32 * COLOR_STEP.r as u32) as u32,
        g: (COLOR_START.g as u32 as u32 * COLOR_STEP.g as u32) as u32,
        b: (COLOR_START.b as u32 as u32 * COLOR_STEP.b as u32) as u32,
    };
    // generate image frames
    // TODO: refactor the code such that the Rgba images are stored in an array
    (0..FRAME_COUNT).into_par_iter().for_each(|i| {
        let c = Complex {
            re: C_START.re + i as f64 * C_STEP.re,
            im: C_START.im + i as f64 * C_STEP.im,
        };

        let img = gen_image(&color, &c);

        let filename = format!("frame_{:04}.png", i);
        img.save(&filename).unwrap();

        // collect frames
        frames.lock().unwrap().push(img);

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

fn gen_picture() {
    let img = gen_image(&COLOR_START, &C_START);
    img.save("1.png").unwrap();
}

// function that adds additional frames to improve frame rate
fn pad_frames(frames: Vec<Vec<Complex<f64>>>, padding: usize) {
    fn frame_diff(frame1: Vec<f64>, frame2: Vec<f64>) {
        todo!()
    }
    todo!()
}

pub fn boundary(epsilon: usize) -> Vec<Complex<f64>> {
    #[inline]
    fn gen_circ(num_points: usize) -> Vec<Complex<f64>> {
        let mut circle = Vec::with_capacity(num_points);

        for i in 0..num_points {
            let theta = 2.0 * std::f64::consts::PI * (i as f64) / (num_points as f64);
            let x = theta.cos();
            let y = theta.sin();
            circle.push(Complex::new(x, y));
        }

        circle
    }
    #[inline]
    fn within_epsilon(c: &Complex<f64>, epsilon: &usize) -> bool {
        let mut z = Complex::new(0.0, 0.0);
        let c1 = c.clone().scale((*epsilon as f64 - 1.0) / (*epsilon as f64));
        dbg!(c1);
        dbg!(c);
        for _ in 0..100 {
            z = z * z + c1;
            dbg!(z.norm());
            if z.norm() > 2.0 {
                return false;
            }
        }

        println!("Got false");
        true
    }
    //
    let mut circ = gen_circ(1000);
    let mut count = 0 as usize;

    plot_points(&circ.clone(), &format!("plot_{:04}.png", &count + 100));
    loop {
        circ.iter_mut().for_each(|z| {
            if !within_epsilon(&z, &epsilon) {
                *z *= (epsilon as f64 - 1.0) / epsilon as f64;
            } else {
                count += 1;
            }
        });
        plot_points(&circ.clone(), &format!("plot_{:04}.png", &count));
        if count == circ.len() {
            dbg!(&count);
            break;
        } else {
            count = 0;
        }
    }
    return circ;
}

pub fn plot_points(points: &Vec<Complex<f64>>, name: &String) {
    // Define the chart area
    let root = Plt::BitMapBackend::new(name, (800, 800)).into_drawing_area();
    root.fill(&Plt::WHITE).unwrap();

    // Create a chart
    let mut chart = Plt::ChartBuilder::on(&root)
        .margin(5)
        .set_all_label_area_size(30)
        .build_cartesian_2d(-2.0..2.0, -2.0..2.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // Plot the complex points
    let _ = chart
        .draw_series(Plt::LineSeries::new(
            points.iter().map(|&c| (c.re, c.im)),
            Plt::BLACK,
        ))
        .unwrap();

    chart
        .configure_series_labels()
        .background_style(&Plt::WHITE)
        .border_style(&Plt::BLACK)
        .draw()
        .unwrap();
}
