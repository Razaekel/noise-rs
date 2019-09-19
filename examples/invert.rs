extern crate noise;

use noise::utils::*;
use noise::{Abs, Invert, Perlin};

fn main() {
    let perlin = Perlin::new();
    let abs = Abs::new(&perlin);

    PlaneMapBuilder::new(&Invert::new(&abs))
        .build()
        .write_to_file("invert.png");
}
