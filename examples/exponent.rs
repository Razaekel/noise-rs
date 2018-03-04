extern crate noise;

use noise::{Exponent, Perlin};
use noise::utils::*;

fn main() {
    let perlin = Perlin::new();
    let exponent = Exponent::new(&perlin).set_exponent(3.0);

    PlaneMapBuilder::new(&exponent)
        .build()
        .write_to_file("exponent.png");
}
