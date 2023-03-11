extern crate noise;

use noise::{
    utils::*,
    Perlin,
    Power,
};

mod utils;

fn main() {
    let perlin1 = Perlin::default();
    let perlin2 = Perlin::new(1);
    let power = Power::new(perlin1, perlin2);

    utils::write_example_to_file(&PlaneMapBuilder::<_, 2>::new(power).build(), "power.png");
}
