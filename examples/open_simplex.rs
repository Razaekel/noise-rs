//! An example of using simplex noise

extern crate noise;

use noise::{
    utils::*,
    OpenSimplex,
    Seedable,
};

mod utils;

fn main() {
    let open_simplex = OpenSimplex::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(open_simplex).build(),
        "open_simplex.png",
    );

    let open_simplex = open_simplex.set_seed(1);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(open_simplex).build(),
        "open_simplex_seed=1.png",
    );
}
