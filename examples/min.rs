extern crate noise;

use noise::{Cylinders, Min, Perlin};

mod debug;

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::new();

    debug::render_noise_module3("min.png", &Min::new(&cyl, &perlin), 1024, 1024, 100);
}
