//! An example of using the Billow noise function

extern crate noise;

use noise::{utils::*, Billow, Perlin};

fn main() {
    PlaneMapBuilder::new(&Billow::<Perlin>::default())
        .build()
        .write_to_file("billow.png");
}
