extern crate noise;

use noise::{utils::*, Cylinders};

mod utils;

fn main() {
    utils::write_example_to_file(
        &PlaneMapBuilder::new(Cylinders::new()).build(),
        "cylinders.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new(Cylinders::new().set_frequency(5.0)).build(),
        "cylinders-f5.png",
    );
}
