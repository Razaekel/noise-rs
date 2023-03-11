extern crate noise;

use noise::{
    utils::*,
    Checkerboard,
    ScalePoint,
};

mod utils;

fn main() {
    let cboard = Checkerboard::default();
    let scale_point = ScalePoint::new(cboard).set_all_scales(1.0, 2.0, 3.0, 1.0);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(scale_point)
            .set_size(500, 500)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "scale_point.png",
    );
}
