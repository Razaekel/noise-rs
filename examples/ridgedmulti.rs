//! An example of using the `RidgedMulti` noise function

extern crate noise;

use noise::{utils::*, RidgedMulti, Perlin, Worley};

fn main() {
    let ridged_multi = RidgedMulti::<Perlin>::default();

    PlaneMapBuilder::new(ridged_multi)
        .build()
        .write_to_file("ridged_multi_perlin.png");

    let ridged_multi = RidgedMulti::<Worley>::default();

    PlaneMapBuilder::new(ridged_multi)
        .build()
        .write_to_file("ridged_multi_worley.png");

    let ridged_multi = RidgedMulti::<RidgedMulti<Perlin>>::default();

    PlaneMapBuilder::new(ridged_multi)
        .build()
        .write_to_file("ridged_multi_ridged_multi_perlin.png");
}
