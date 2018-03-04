extern crate noise;

use noise::{Abs, Perlin};
use noise::utils::*;

fn main() {
    let perlin = Perlin::new();
    let abs = Abs::new(&perlin);

    PlaneMapBuilder::new(&abs).build().write_to_file("abs.png");
}
