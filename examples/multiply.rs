extern crate noise;

use noise::{Cylinders, Multiply, Perlin};
use noise::utils::*;

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::new();
    let multiply = Multiply::new(&cyl, &perlin);

    PlaneMapBuilder::new(&multiply)
        .build()
        .write_to_file("multiply.png");
}
