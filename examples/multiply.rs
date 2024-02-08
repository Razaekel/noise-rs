extern crate noise;

use noise::{utils::*, Cylinders, NoiseFn, Perlin};

mod utils;

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::default();
    let multiply = cyl.multiply(perlin);

    utils::write_example_to_file(&PlaneMapBuilder::new(multiply).build(), "multiply.png");
}
