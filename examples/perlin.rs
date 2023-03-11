//! An example of using perlin noise

extern crate noise;

use noise::{
    core::perlin::{
        perlin_2d,
        perlin_3d,
        perlin_4d,
    },
    permutationtable::PermutationTable,
    utils::*,
};

mod utils;

fn main() {
    let hasher = PermutationTable::new(0);
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(perlin_2d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_2d_seed=0.png",
    );
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(perlin_3d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_3d_seed=0.png",
    );
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(perlin_4d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_4d_seed=0.png",
    );

    let hasher = PermutationTable::new(1);
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(perlin_2d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_2d_seed=1.png",
    );
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(perlin_3d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_3d_seed=1.png",
    );
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(perlin_4d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_4d_seed=1.png",
    );
}
