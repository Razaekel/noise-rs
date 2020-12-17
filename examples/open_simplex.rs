//! An example of using simplex noise

extern crate noise;

use noise::{utils::*, OpenSimplex, Seedable};

fn main() {
    let open_simplex = OpenSimplex::new();

    PlaneMapBuilder::new(&open_simplex)
        .build()
        .write_to_file("open_simplex.png");

    let open_simplex = open_simplex.set_seed(1);

    PlaneMapBuilder::new(&open_simplex)
        .build()
        .write_to_file("open_simplex_seed=1.png");
}
