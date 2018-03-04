//! An example of using the fBm noise function

extern crate noise;

use noise::Fbm;
use noise::utils::*;

fn main() {
    let fbm = Fbm::new();

    PlaneMapBuilder::new(&fbm).build().write_to_file("fbm.png");
}
