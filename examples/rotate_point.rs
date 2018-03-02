extern crate noise;

use noise::{Cylinders, RotatePoint};

mod debug;

fn main() {
    let cylinders = Cylinders::new();
    let rotate_point = RotatePoint::new(cylinders).set_x_angle(60.0);

    debug::render_noise_module3("rotate_point.png", &rotate_point, 1024, 1024, 50);
}
