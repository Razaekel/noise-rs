//! An example of using value noise

extern crate noise;

use noise::{
    core::value::{value_2d, value_3d, value_4d},
    permutationtable::PermutationTable,
    utils::*,
};

mod utils;

fn main() {
    let hasher = PermutationTable::new(0);
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| value_2d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "value 2d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| value_3d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "value 3d.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| value_4d(point.into(), &hasher))
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "value 4d.png",
    );
}
