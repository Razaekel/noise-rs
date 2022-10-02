//! An example of using the fBm noise function

extern crate noise;

use noise::{utils::*, Fbm, Perlin, Worley};

fn main() {
    let fbm = Fbm::<Perlin>::default();

    PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size(1000, 1000)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm_perlin.png");

    let fbm = Fbm::<Worley>::default();

    PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size(1000, 1000)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm_worley.png");

    let fbm = Fbm::<Fbm<Perlin>>::default();

    PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size(1000, 1000)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm_fbm_perlin.png");
}
