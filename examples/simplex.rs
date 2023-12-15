//! An example of using simplex noise

extern crate noise;

use noise::{
    core::simplex::{simplex_2d, simplex_3d, simplex_4d},
    permutationtable::PermutationTable,
    utils::*,
};

mod utils;

fn main() {
    let hasher = PermutationTable::new(0);

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| simplex_2d(point.into(), &hasher).0)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "simplex 2d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| simplex_3d(point.into(), &hasher).0)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "simplex 3d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| simplex_4d(point.into(), &hasher).0)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "simplex 4d.png",
    );
}
