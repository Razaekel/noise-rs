//! An example of using simplex noise

extern crate noise;

use noise::{
    utils::*,
    Seedable,
    Simplex,
};

mod utils;

fn main() {
    let mut simplex = Simplex::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(simplex)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "simplex.png",
    );

    simplex = simplex.set_seed(1);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(simplex)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "simplex_seed=1.png",
    );
}
