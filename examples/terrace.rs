extern crate noise;

use noise::{utils::*, Perlin, Terrace};

fn main() {
    let perlin = Perlin::default();
    let terrace = Terrace::new(perlin)
        .add_control_point(-1.0)
        .add_control_point(-0.5)
        .add_control_point(0.1)
        .add_control_point(1.0);

    let terrace_inverted = Terrace::new(perlin)
        .add_control_point(-1.0)
        .add_control_point(-0.5)
        .add_control_point(0.1)
        .add_control_point(1.0)
        .invert_terraces(true);

    PlaneMapBuilder::<_, 2>::new(terrace)
        .build()
        .write_to_file("terrace.png");

    PlaneMapBuilder::<_, 2>::new(terrace_inverted)
        .build()
        .write_to_file("terrace_inverted.png");
}
