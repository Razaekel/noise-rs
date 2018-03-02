extern crate noise;

use noise::{Exponent, Perlin};

mod debug;

fn main() {
    let perlin = Perlin::new();

    debug::render_noise_module3(
        "exponent.png",
        &Exponent::new(&perlin).set_exponent(3.0),
        1024,
        1024,
        100,
    );
}
