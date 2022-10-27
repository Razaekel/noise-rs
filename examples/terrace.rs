extern crate noise;

use noise::{utils::*, Perlin, Terrace};

mod utils;

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

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(terrace).build(),
        "terrace.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(terrace_inverted).build(),
        "terrace_inverted.png",
    );
}
