extern crate noise;

use noise::{Checkerboard, TranslatePoint};
use noise::utils::*;

fn main() {
    let cboard = Checkerboard::new();
    let translate_point = TranslatePoint::new(cboard).set_all_translations(0.5, 0.5, 0.0, 0.0);

    PlaneMapBuilder::new(&translate_point)
        .build()
        .write_to_file("translate_point.png");
}
