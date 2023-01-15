//! An example of using perlin noise

extern crate noise;

use noise::{utils::*, PerlinSurflet, Seedable};

mod utils;

fn main() {
    let perlin = PerlinSurflet::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(perlin)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_surflet.png",
    );

    let perlin = perlin.set_seed(1);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(perlin)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_surflet_seed=1.png",
    );
}
