//! An example of using value noise

extern crate noise;

use noise::{
    utils::*,
    Value,
};

mod utils;

fn main() {
    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(Value::default()).build(),
        "value.png",
    );
}
