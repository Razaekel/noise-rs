extern crate noise;

use noise::{utils::*, Perlin, Turbulence};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let turbulence = Turbulence::<_, Perlin>::new(perlin);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(turbulence).build(),
        "turbulence.png",
    );
}
