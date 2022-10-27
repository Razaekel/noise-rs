//! An example of using the `BasicMulti` noise function

extern crate noise;

use noise::{utils::*, BasicMulti, Perlin, Worley};

mod utils;

fn main() {
    let basicmulti = BasicMulti::<Perlin>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(basicmulti).build(),
        "basicmulti_perlin.png",
    );

    let basicmulti = BasicMulti::<Worley>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(basicmulti).build(),
        "basicmulti_worley.png",
    );

    let basicmulti = BasicMulti::<BasicMulti<Perlin>>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(basicmulti).build(),
        "basicmulti_basicmulti_perlin.png",
    );
}
