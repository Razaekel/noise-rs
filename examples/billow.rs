//! An example of using the Billow noise function

extern crate noise;

use noise::utils::*;
use noise::Billow;

fn main() {
    PlaneMapBuilder::new(&Billow::new())
        .build()
        .write_to_file("billow.png");
}
