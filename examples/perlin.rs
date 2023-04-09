//! An example of using perlin noise

extern crate noise;

use noise::{utils::*, core::perlin::perlin_3d, permutationtable::PermutationTable};

mod utils;

fn main() {
    let hasher = PermutationTable::new(0);
    
    PlaneMapBuilder::new_fn(perlin_3d, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin_seed=0.png");
    
    let hasher = PermutationTable::new(1);
    PlaneMapBuilder::new_fn(perlin_3d, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin_seed=1.png");
}
