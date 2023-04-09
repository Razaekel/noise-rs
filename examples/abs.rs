extern crate noise;

use noise::{utils::*, Abs, Perlin};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let abs = Abs::new(perlin);

    utils::write_example_to_file(&PlaneMapBuilder::new(abs).build(), "abs.png");
}
