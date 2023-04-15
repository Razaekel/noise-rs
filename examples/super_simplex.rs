//! An example of using Super Simplex noise

extern crate noise;

use noise::{
    core::super_simplex::{super_simplex_2d, super_simplex_3d},
    permutationtable::PermutationTable,
    utils::*,
};

mod utils;

fn main() {
    let hasher = PermutationTable::new(0);

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| super_simplex_2d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "super_simplex 2d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| super_simplex_3d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "super_simplex 3d.png",
    );
}
