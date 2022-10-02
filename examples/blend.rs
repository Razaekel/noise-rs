extern crate noise;

use noise::{utils::*, Blend, Fbm, Perlin, RidgedMulti};

fn main() {
    let perlin = Perlin::default();
    let ridged = RidgedMulti::<Perlin>::default();
    let fbm = Fbm::<Perlin>::default();
    let blend = Blend::new(perlin, ridged, fbm);

    PlaneMapBuilder::<_, 2>::new(blend)
        .build()
        .write_to_file("blend.png");
}
