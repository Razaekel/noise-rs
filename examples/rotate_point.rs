extern crate noise;

use noise::{utils::*, Cylinders, RotatePoint};

fn main() {
    let cylinders = Cylinders::new();
    let rotate_point = RotatePoint::new(cylinders).set_x_angle(60.0);

    PlaneMapBuilder::<_, 2>::new(rotate_point)
        .build()
        .write_to_file("rotate_point.png");
}
