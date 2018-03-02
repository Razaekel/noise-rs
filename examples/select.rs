extern crate noise;

use noise::*;

mod debug;

fn main() {
    let checkerboard = &Checkerboard::new();
    let cylinders = &Cylinders::new();
    let perlin = &Perlin::new();
    let constant = &Constant::new(0.5);
    let select1 = Select::new(perlin, cylinders, checkerboard)
        .set_bounds(0.0, 1.0)
        .set_falloff(0.5);
    let select2 = Select::new(perlin, constant, checkerboard)
        .set_bounds(0.0, 1.0)
        .set_falloff(0.0);

    debug::render_noise_module3("select1.png", &select1, 1024, 1024, 100);
    debug::render_noise_module3("select2.png", &select2, 1024, 1024, 100);
}
