extern crate noise;

use noise::{Blend, Fbm, Perlin, RidgedMulti};
use noise::utils::*;

fn main() {
    let perlin = Perlin::new();
    let ridged = RidgedMulti::new();
    let fbm = Fbm::new();
    let blend = Blend::new(&perlin, &ridged, &fbm);

    PlaneMapBuilder::new(&blend)
        .build()
        .write_to_file("blend.png");
}
