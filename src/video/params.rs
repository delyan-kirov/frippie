use num_complex::Complex;

pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

pub const ZOOM: f64 = 0.3;
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 800;
pub const MAX_ITER: u32 = 50;
pub const OUTPUT_VIDEO_FILE: &str = "julia_set_video.mp4";
pub const FRAME_RATE: u32 = 30;
pub const _VIDEO_RESOLUTION: (u16, u16) = (1200, 1200);
//
pub const FRAME_COUNT: u32 = 100;
pub const COLOR_START: Color = Color { r: 4, g: 1, b: 9 };
pub const COLOR_STEP: Color = Color { r: 1, g: 1, b: 1 };
pub const C_START: Complex<f64> = Complex {
    re: -0.787,
    im: 0.1548,
};
pub const C_STEP: Complex<f64> = Complex {
    re: 0.00008,
    im: 0.00008,
};
pub const SVG_WIDTH: u16 = 1800;
pub const SVG_HEIGHT: u16 = 1800;
