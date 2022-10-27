//! An example of using the `RidgedMulti` noise function

extern crate noise;

use noise::{utils::*, Perlin, RidgedMulti, Worley};

mod utils;

fn main() {
    let ridged_multi = RidgedMulti::<Perlin>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(ridged_multi).build(),
        "ridged_multi_perlin.png",
    );

    let ridged_multi = RidgedMulti::<Worley>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(ridged_multi).build(),
        "ridged_multi_worley.png",
    );

    let ridged_multi = RidgedMulti::<RidgedMulti<Perlin>>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(ridged_multi).build(),
        "ridged_multi_ridged_multi_perlin.png",
    );
}
