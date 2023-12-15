//! An example of using simplex noise

extern crate noise;

use noise::{
    core::open_simplex::{open_simplex_2d, open_simplex_3d, open_simplex_4d},
    permutationtable::PermutationTable,
    utils::*,
};

mod utils;

fn main() {
    let hasher = PermutationTable::new(0);

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| open_simplex_2d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "open_simplex 2d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| open_simplex_3d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "open_simplex 3d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| open_simplex_4d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "open_simplex 4d.png",
    );
}
