extern crate noise;

use noise::{Add, Cylinders, Perlin};

mod debug;

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::new();

    debug::render_noise_module3("add.png", &Add::new(&cyl, &perlin), 1024, 1024, 100);
}
