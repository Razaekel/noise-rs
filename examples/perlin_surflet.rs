//! An example of using perlin_surflet noise

extern crate noise;

use noise::{
    core::perlin_surflet::{perlin_surflet_2d, perlin_surflet_3d, perlin_surflet_4d},
    permutationtable::PermutationTable,
    utils::*,
};

mod utils;

fn main() {
    let hasher = PermutationTable::new(0);

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| perlin_surflet_2d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin surflet 2d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| perlin_surflet_3d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin surflet 3d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| perlin_surflet_4d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin surflet 4d.png",
    );
}
