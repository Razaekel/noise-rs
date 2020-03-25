extern crate noise;

use noise::{utils::*, Blend, Fbm, Perlin, RidgedMulti};

fn main() {
    let perlin = Perlin::new();
    let ridged = RidgedMulti::new();
    let fbm = Fbm::new();
    let blend = Blend::new(&perlin, &ridged, &fbm);

    PlaneMapBuilder::new(&blend)
        .build()
        .write_to_file("blend.png");
}
