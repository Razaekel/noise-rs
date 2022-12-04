//! An example of using simplex noise

extern crate noise;

use noise::{utils::*, Seedable, Simplex};

fn main() {
    let mut simplex = Simplex::default();

    PlaneMapBuilder::new(simplex)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("simplex.png");

    simplex = simplex.set_seed(1);

    PlaneMapBuilder::new(simplex)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("simplex_seed=1.png");
}
