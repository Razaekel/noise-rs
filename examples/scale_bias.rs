extern crate noise;

use noise::{utils::*, Perlin, ScaleBias};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let scale_bias = ScaleBias::new(perlin).set_scale(0.0625).set_bias(0.0);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(scale_bias).build(),
        "scale_bias.png",
    );
}
