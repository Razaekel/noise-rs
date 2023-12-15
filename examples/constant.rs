//! An example of generating constant valued noise

extern crate noise;

use noise::{utils::*, Constant};

mod utils;

fn main() {
    utils::write_example_to_file(
        &PlaneMapBuilder::new(Constant::new(-1.0)).build(),
        "constant_-1.png",
    );
    utils::write_example_to_file(
        &PlaneMapBuilder::new(Constant::new(0.0)).build(),
        "constant_0.png",
    );
    utils::write_example_to_file(
        &PlaneMapBuilder::new(Constant::new(1.0)).build(),
        "constant_1.png",
    );
}
