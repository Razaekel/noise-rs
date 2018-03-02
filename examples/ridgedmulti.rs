//! An example of using the `RidgedMulti` noise function

extern crate noise;

use noise::{MultiFractal, RidgedMulti};

mod debug;

fn main() {
    debug::render_noise_module3(
        "ridgedmulti.png",
        &RidgedMulti::new().set_octaves(2),
        1024,
        1024,
        100,
    );
}
