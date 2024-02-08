extern crate noise;

use noise::{utils::*, Cylinders, NoiseFn, Perlin};

mod utils;

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::default();
    let min = cyl.min(perlin);

    utils::write_example_to_file(&PlaneMapBuilder::new(min).build(), "min.png");
}
