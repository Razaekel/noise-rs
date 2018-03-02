extern crate noise;

use noise::{Checkerboard, Constant, Cylinders, Displace, Perlin};

mod debug;

fn main() {
    let cboard = Checkerboard::new();
    let constant = Constant::new(0.0);
    let cylinders = Cylinders::new();
    let perlin = Perlin::new();
    let displace = Displace::new(cylinders, cboard, perlin, constant, constant);

    debug::render_noise_module3("displace.png", &displace, 1024, 1024, 50);
}
