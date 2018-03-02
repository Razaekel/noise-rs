extern crate noise;

use noise::{Checkerboard, ScalePoint};

mod debug;

fn main() {
    let cboard = Checkerboard::new();
    let scale_point = ScalePoint::new(cboard).set_all_scales(1.0, 2.0, 3.0, 1.0);

    debug::render_noise_module3("scale_point.png", &scale_point, 1024, 1024, 50);
}
