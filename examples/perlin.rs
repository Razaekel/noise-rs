//! An example of using perlin noise

extern crate noise;

use noise::{Perlin, utils::*};

mod utils;

fn main() {
    utils::write_example_to_file(&PlaneMapBuilder::new(Perlin::default())
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build(),
        "perlin_seed=0.png");

    utils::write_example_to_file(&PlaneMapBuilder::new(Perlin::new(1))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build(),
        "perlin_seed=1.png");
}
