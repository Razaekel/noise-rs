//! An example of using the `RidgedMulti` noise function

extern crate noise;

use noise::{utils::*, RidgedMulti};

fn main() {
    let ridged_multi = RidgedMulti::new();

    PlaneMapBuilder::new(&ridged_multi)
        .build()
        .write_to_file("ridged_multi.png");
}
