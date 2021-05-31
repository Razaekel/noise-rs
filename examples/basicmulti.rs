//! An example of using the `BasicMulti` noise function

extern crate noise;

use noise::{utils::*, BasicMulti, Perlin};

fn main() {
    PlaneMapBuilder::new(&BasicMulti::<Perlin>::default())
        .build()
        .write_to_file("basicmulti.png");
}
