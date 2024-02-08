extern crate noise;

use noise::{utils::*, Fbm, NoiseFn, Perlin, RidgedMulti};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let ridged = RidgedMulti::<Perlin>::default();
    let fbm = Fbm::<Perlin>::default();
    let blend = perlin.blend(ridged, fbm);

    utils::write_example_to_file(&PlaneMapBuilder::new(blend).build(), "blend.png");
}
