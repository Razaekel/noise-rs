//! An example of using the fBm noise function

extern crate noise;

use noise::{
    utils::*,
    Fbm,
    Perlin,
    Worley,
};

mod utils;

fn main() {
    let fbm = Fbm::<Perlin>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(fbm)
            .set_size(1000, 1000)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "fbm_perlin.png",
    );

    let fbm = Fbm::<Worley>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(fbm)
            .set_size(1000, 1000)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "fbm_worley.png",
    );

    let fbm = Fbm::<Fbm<Perlin>>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(fbm)
            .set_size(1000, 1000)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "fbm_fbm_perlin.png",
    );
}
