extern crate noise;

use noise::utils::*;
use noise::{Checkerboard, Max, Perlin};

fn main() {
    let checkerboard = Checkerboard::new();
    let perlin = Perlin::new();
    let max = Max::new(&checkerboard, &perlin);

    PlaneMapBuilder::new(&max).build().write_to_file("max.png");
}
