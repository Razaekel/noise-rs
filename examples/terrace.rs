extern crate noise;

use noise::{Perlin, Terrace};

mod debug;

fn main() {
    let perlin = Perlin::new();
    let terrace = Terrace::new(&perlin)
        .add_control_point(-1.0)
        .add_control_point(-0.5)
        .add_control_point(0.1)
        .add_control_point(1.0);

    debug::render_noise_module3("terrace.png", &terrace, 1024, 1024, 100);

    let terrace_inverted = Terrace::new(&perlin)
        .add_control_point(-1.0)
        .add_control_point(-0.5)
        .add_control_point(0.1)
        .add_control_point(1.0)
        .invert_terraces(true);

    debug::render_noise_module3("terrace_inverted.png", &terrace_inverted, 1024, 1024, 100);
}
