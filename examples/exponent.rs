extern crate noise;

use noise::{utils::*, Exponent, Perlin};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let exponent = Exponent::new(perlin).set_exponent(3.0);

    utils::write_example_to_file(&PlaneMapBuilder::new(exponent).build(), "exponent.png");
}
