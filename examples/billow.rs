//! An example of using the Billow noise function

extern crate noise;

use noise::Billow;
use noise::utils::*;

fn main() {
    PlaneMapBuilder::new(&Billow::new())
        .build()
        .write_to_file("billow.png");
}
