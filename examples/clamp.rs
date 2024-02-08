extern crate noise;

use noise::{utils::*, NoiseFn, Perlin};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let clamp = perlin.clamp(0.0, 0.5);

    utils::write_example_to_file(&PlaneMapBuilder::new(clamp).build(), "clamp.png");
}
