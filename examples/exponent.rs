extern crate noise;

use noise::{utils::*, NoiseFn, Perlin};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let exponent = perlin.exponent(3.0);

    utils::write_example_to_file(&PlaneMapBuilder::new(exponent).build(), "exponent.png");
}
