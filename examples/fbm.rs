//! An example of using the fBm noise function

extern crate noise;

use noise::{utils::*, Fbm, OpenSimplex};

fn main() {
    let fbm: Fbm<OpenSimplex> = Fbm::default();

    PlaneMapBuilder::new(&fbm)
        .set_size(1000, 1000)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm.png");
}
