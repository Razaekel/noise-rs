extern crate noise;

use noise::{
    utils::*,
    Checkerboard,
    TranslatePoint,
};

mod utils;

fn main() {
    let cboard = Checkerboard::default();
    let translate_point = TranslatePoint::new(cboard).set_all_translations(0.5, 0.5, 0.0, 0.0);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(translate_point).build(),
        "translate_point.png",
    );
}
