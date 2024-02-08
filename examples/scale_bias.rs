extern crate noise;

use noise::{utils::*, NoiseFn, Perlin};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let scale_bias = perlin.scale_bias(0.0625, 0.0);

    utils::write_example_to_file(&PlaneMapBuilder::new(scale_bias).build(), "scale_bias.png");
}
