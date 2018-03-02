extern crate noise;

use noise::{Abs, Perlin};

mod debug;

fn main() {
    let perlin = Perlin::new();

    debug::render_noise_module3("abs.png", &Abs::new(&perlin), 1024, 1024, 100);
}
