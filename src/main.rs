mod video;
// use video::params::*;

fn main() {
    // let _ = gen_picture();
    let points = video::boundary(1000);
    video::plot_points(points);
    // video::gen_video();
}
