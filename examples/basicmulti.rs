//! An example of using the `BasicMulti` noise function

extern crate noise;

use noise::{utils::*, BasicMulti, Perlin, Worley};

fn main() {
    let basicmulti = BasicMulti::<Perlin>::default();

    PlaneMapBuilder::<_, 2>::new(basicmulti)
        .build()
        .write_to_file("basicmulti_perlin.png");

    let basicmulti = BasicMulti::<Worley>::default();

    PlaneMapBuilder::<_, 2>::new(basicmulti)
        .build()
        .write_to_file("basicmulti_worley.png");

    let basicmulti = BasicMulti::<BasicMulti<Perlin>>::default();

    PlaneMapBuilder::<_, 2>::new(basicmulti)
        .build()
        .write_to_file("basicmulti_basicmulti_perlin.png");
}
