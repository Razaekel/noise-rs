//! An example of using the `BasicMulti` noise function

extern crate noise;

use noise::{utils::*, BasicMulti};

fn main() {
    PlaneMapBuilder::new(BasicMulti::default())
        .build()
        .write_to_file("basicmulti.png");
}
