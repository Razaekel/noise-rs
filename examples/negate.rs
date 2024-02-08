extern crate noise;

use noise::{utils::*, Abs, NoiseFn, Perlin};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let negate = Abs::new(perlin).negate();

    utils::write_example_to_file(&PlaneMapBuilder::new(negate).build(), "negate.png");
}
