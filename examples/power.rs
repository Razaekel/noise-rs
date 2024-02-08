extern crate noise;

use noise::{utils::*, NoiseFn, Perlin};

mod utils;

fn main() {
    let perlin1 = Perlin::default();
    let perlin2 = Perlin::new(1);
    let power = perlin1.power(perlin2);

    utils::write_example_to_file(&PlaneMapBuilder::new(power).build(), "power.png");
}
