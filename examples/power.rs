extern crate noise;

use noise::{utils::*, Perlin, Power, DEFAULT_SEED};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

mod utils;

fn main() {
    let perlin1 = Perlin::default();
    let perlin2 = Perlin::new(XorShiftRng::from_seed(DEFAULT_SEED).gen());
    let power = Power::new(perlin1, perlin2);

    utils::write_example_to_file(&PlaneMapBuilder::new(power).build(), "power.png");
}
