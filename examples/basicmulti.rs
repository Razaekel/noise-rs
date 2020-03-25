//! An example of using the `BasicMulti` noise function

extern crate noise;

use noise::{utils::*, BasicMulti};

fn main() {
    PlaneMapBuilder::new(&BasicMulti::new())
        .build()
        .write_to_file("basicmulti.png");
}
