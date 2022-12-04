//! An example of using value noise

extern crate noise;

use noise::{utils::*, Value};

fn main() {
    PlaneMapBuilder::<_, 2>::new(Value::default())
        .build()
        .write_to_file("value.png");
}
