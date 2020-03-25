//! An example of using value noise

extern crate noise;

use noise::{utils::*, Value};

fn main() {
    PlaneMapBuilder::new(&Value::new())
        .build()
        .write_to_file("value.png");
}
