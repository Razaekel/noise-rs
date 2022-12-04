//! An example of generating constant valued noise

extern crate noise;

use noise::{utils::*, Checkerboard};

fn main() {
    let checker = Checkerboard::new(0);

    PlaneMapBuilder::<_, 2>::new(checker)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("checkerboard.png");
}
