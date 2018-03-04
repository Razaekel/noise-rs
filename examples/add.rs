extern crate noise;

use noise::{Add, Cylinders, Perlin};
use noise::utils::*;

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::new();

    let add = Add::new(&cyl, &perlin);

    PlaneMapBuilder::new(&add).build().write_to_file("add.png");
}
