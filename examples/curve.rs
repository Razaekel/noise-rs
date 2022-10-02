extern crate noise;

use noise::{utils::*, Curve, Perlin};

fn main() {
    let perlin = Perlin::default();
    let curve = Curve::new(perlin)
        .add_control_point(-2.0, -2.0)
        .add_control_point(-1.0, -1.25)
        .add_control_point(0.0, -0.75)
        .add_control_point(0.5, -0.25)
        .add_control_point(0.625, 0.875)
        .add_control_point(0.75, 1.0)
        .add_control_point(2.0, 1.25);

    PlaneMapBuilder::<_, 2>::new(curve)
        .build()
        .write_to_file("curve.png");
}
