//! An example of generating constant valued noise

extern crate noise;

use noise::{utils::*, Checkerboard};

mod utils;

fn main() {
    let checker = Checkerboard::new(0);

    utils::write_example_to_file(
        &PlaneMapBuilder::new(checker)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "checkerboard.png",
    );
}
