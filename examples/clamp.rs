extern crate noise;

use noise::{Clamp, Perlin};

mod debug;

fn main() {
    let perlin = Perlin::new();
    let clamp = Clamp::new(&perlin)
        .set_lower_bound(0.0)
        .set_upper_bound(0.5);

    debug::render_noise_module3("clamp.png", &clamp, 1024, 1024, 100);
}
