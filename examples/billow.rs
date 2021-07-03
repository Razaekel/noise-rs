//! An example of using the Billow noise function

extern crate noise;

use noise::{utils::*, Billow};

fn main() {
    PlaneMapBuilder::new(Billow::default())
        .build()
        .write_to_file("billow.png");
}
