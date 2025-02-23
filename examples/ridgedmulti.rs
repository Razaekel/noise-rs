//! An example of using the `RidgedMulti` noise function

extern crate noise;

use noise::{utils::*, Perlin, RidgedMulti, Worley, DEFAULT_SEED};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

mod utils;

fn main() {
    let mut rng = XorShiftRng::from_seed(DEFAULT_SEED);

    let ridged_multi = RidgedMulti::<Perlin>::new(rng.gen());

    utils::write_example_to_file(
        &PlaneMapBuilder::new(ridged_multi).build(),
        "ridged_multi_perlin.png",
    );

    let ridged_multi = RidgedMulti::<Worley>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::new(ridged_multi).build(),
        "ridged_multi_worley.png",
    );

    let ridged_multi = RidgedMulti::<RidgedMulti<Perlin>>::new(rng.gen());

    utils::write_example_to_file(
        &PlaneMapBuilder::new(ridged_multi).build(),
        "ridged_multi_ridged_multi_perlin.png",
    );
}
