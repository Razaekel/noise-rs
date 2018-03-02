extern crate noise;

use noise::{Blend, Fbm, Perlin, RidgedMulti};

mod debug;

fn main() {
    let perlin = Perlin::new();
    let ridged = RidgedMulti::new();
    let fbm = Fbm::new();
    let blend = Blend::new(&perlin, &ridged, &fbm);

    debug::render_noise_module3("blend.png", &blend, 1024, 1024, 100);
}
