extern crate noise;

use noise::{utils::*, Blend, Fbm, Perlin, RidgedMulti};

fn main() {
    let perlin = Perlin::default();
    let ridged = RidgedMulti::default();
    let fbm :Fbm<Perlin>= Fbm::default();
    let blend = Blend::new(&perlin, &ridged, &fbm);

    PlaneMapBuilder::new(&blend)
        .build()
        .write_to_file("blend.png");
}
