mod video;
// use video::params::*;
use num_complex::Complex;

fn psi(i: f64) -> Complex<f64> {
    let theta = std::f64::consts::PI / i;
    let theta1 = Complex::from_polar(1.0, theta);
    theta
        + (-0.5)
        + (1.0 / 8.0) * Complex::powi(&theta1, -1)
        + (1.0 / 4.0) * Complex::powi(&theta1, -2)
        + (1.0 / 128.0) * Complex::powi(&theta1, -3)
}

fn gen_bound(epsilon: f64) -> Vec<Complex<f64>> {
    vec![psi(epsilon)]
}

fn main() {
    // let _ = gen_picture();
    // video::gen_video();
    println!("{:?}", gen_bound(0.1));
}
