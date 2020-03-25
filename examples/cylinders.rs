extern crate noise;

use noise::{utils::*, Cylinders};

fn main() {
    PlaneMapBuilder::new(&Cylinders::new())
        .build()
        .write_to_file("cylinders.png");
    PlaneMapBuilder::new(&Cylinders::new().set_frequency(5.0))
        .build()
        .write_to_file("cylinders-f5.png");
}
