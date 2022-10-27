extern crate noise;

use noise::{utils::*, Blend, Fbm, Perlin, RidgedMulti};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let ridged = RidgedMulti::<Perlin>::default();
    let fbm = Fbm::<Perlin>::default();
    let blend = Blend::new(perlin, ridged, fbm);

    utils::write_example_to_file(&PlaneMapBuilder::<_, 2>::new(blend).build(), "blend.png");
}
