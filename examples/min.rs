extern crate noise;

use noise::utils::*;
use noise::{Checkerboard, Min, Perlin};

fn main() {
    let checkerboard = Checkerboard::new();
    let perlin = Perlin::new();
    let min = Min::new(&checkerboard, &perlin);

    PlaneMapBuilder::new(&min).build().write_to_file("min.png");
}
