//! An example of using perlin noise

extern crate noise;

use noise::{utils::*, Perlin, Seedable};

fn main() {
    let perlin = Perlin::new();

    PlaneMapBuilder::new(&perlin)
        .build()
        .write_to_file("perlin.png");

    let perlin = perlin.set_seed(1);

    PlaneMapBuilder::new(&perlin)
        .build()
        .write_to_file("perlin_seed=1.png");
}
