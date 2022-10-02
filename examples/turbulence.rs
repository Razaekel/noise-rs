extern crate noise;

use noise::{utils::*, Perlin, Turbulence};

fn main() {
    let perlin = Perlin::default();
    let turbulence = Turbulence::<_, Perlin>::new(perlin);

    PlaneMapBuilder::<_, 2>::new(turbulence)
        .build()
        .write_to_file("turbulence.png");
}
