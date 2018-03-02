extern crate noise;

use noise::{Perlin, ScaleBias};

mod debug;

fn main() {
    let perlin = Perlin::new();
    let scale_bias = ScaleBias::new(&perlin).set_scale(0.0625).set_bias(0.0);

    debug::render_noise_module3("scale_bias.png", &scale_bias, 1024, 1024, 100);
}
