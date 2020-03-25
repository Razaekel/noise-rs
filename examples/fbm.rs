//! An example of using the fBm noise function

extern crate noise;

use noise::{utils::*, Fbm};

fn main() {
    let fbm = Fbm::new();

    PlaneMapBuilder::new(&fbm).build().write_to_file("fbm.png");
}
