//! An example of using perlin noise

extern crate noise;

use noise::{
    core::perlin::{perlin_2d, perlin_3d, perlin_4d},
    permutationtable::PermutationTable,
    utils::*,
};

mod utils;

fn main() {
    let hasher = PermutationTable::new(0);
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| perlin_2d(point, &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_2d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| perlin_3d(point, &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_3d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| perlin_4d(point, &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_4d.png",
    );
}
