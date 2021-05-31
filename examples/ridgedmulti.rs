//! An example of using the `RidgedMulti` noise function

extern crate noise;

use noise::{utils::*, RidgedMulti, Perlin};

fn main() {
    let ridged_multi :RidgedMulti<Perlin> = RidgedMulti::default();

    PlaneMapBuilder::new(&ridged_multi)
        .build()
        .write_to_file("ridged_multi.png");
}
