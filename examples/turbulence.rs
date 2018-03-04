extern crate noise;

use noise::{Perlin, Turbulence};
use noise::utils::*;

fn main() {
    let perlin = Perlin::new();
    let turbulence = Turbulence::new(&perlin);

    PlaneMapBuilder::new(&turbulence)
        .build()
        .write_to_file("turbulence.png");
}
