extern crate noise;

use noise::{Perlin, Turbulence};

mod debug;

fn main() {
    let perlin = Perlin::new();

    debug::render_noise_module3("turbulence.png", &Turbulence::new(perlin), 1024, 1024, 50);
}
