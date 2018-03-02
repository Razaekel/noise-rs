extern crate noise;

use noise::{Checkerboard, TranslatePoint};

mod debug;

fn main() {
    let cboard = Checkerboard::new();
    let translate_point = TranslatePoint::new(cboard).set_all_translations(0.0, 2.0, 3.0, 0.0);

    debug::render_noise_module3("translate_point.png", &translate_point, 1024, 1024, 50);
}
