extern crate noise;

use noise::{Abs, Invert, Perlin};

mod debug;

fn main() {
    let perlin = Perlin::new();
    let abs = Abs::new(&perlin);

    debug::render_noise_module3("invert.png", &Invert::new(&abs), 1024, 1024, 100);
}
