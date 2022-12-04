extern crate noise;

use noise::{utils::*, Exponent, Perlin};

fn main() {
    let perlin = Perlin::default();
    let exponent = Exponent::new(perlin).set_exponent(3.0);

    PlaneMapBuilder::<_, 2>::new(exponent)
        .build()
        .write_to_file("exponent.png");
}
