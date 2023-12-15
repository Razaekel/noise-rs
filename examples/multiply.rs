extern crate noise;

use noise::{utils::*, Cylinders, Multiply, Perlin};

mod utils;

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::default();
    let multiply = Multiply::new(cyl, perlin);

    utils::write_example_to_file(&PlaneMapBuilder::new(multiply).build(), "multiply.png");
}
