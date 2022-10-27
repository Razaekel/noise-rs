extern crate noise;

use noise::{utils::*, Cylinders, RotatePoint};

mod utils;

fn main() {
    let cylinders = Cylinders::new();
    let rotate_point = RotatePoint::new(cylinders).set_x_angle(60.0);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(rotate_point).build(),
        "rotate_point.png",
    );
}
