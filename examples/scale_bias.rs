extern crate noise;

use noise::{utils::*, Perlin, ScaleBias};

fn main() {
    let perlin = Perlin::new();
    let scale_bias = ScaleBias::new(&perlin).set_scale(0.0625).set_bias(0.0);

    PlaneMapBuilder::new(&scale_bias)
        .build()
        .write_to_file("scale_bias.png");
}
