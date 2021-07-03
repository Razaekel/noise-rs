//! An example of generating constant valued noise

extern crate noise;

use noise::{utils::*, Constant};

fn main() {
    PlaneMapBuilder::new(Constant::new(-1.0))
        .build()
        .write_to_file("constant_-1.png");
    PlaneMapBuilder::new(Constant::new(0.0))
        .build()
        .write_to_file("constant_0.png");
    PlaneMapBuilder::new(Constant::new(1.0))
        .build()
        .write_to_file("constant_1.png");
}
