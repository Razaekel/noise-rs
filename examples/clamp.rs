extern crate noise;

use noise::utils::*;
use noise::{Clamp, Perlin};

fn main() {
    let perlin = Perlin::new();
    let clamp = Clamp::new(&perlin)
        .set_lower_bound(0.0)
        .set_upper_bound(0.5);

    PlaneMapBuilder::new(&clamp)
        .build()
        .write_to_file("clamp.png");
}
