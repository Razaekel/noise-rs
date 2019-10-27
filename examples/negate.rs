extern crate noise;

use noise::{utils::*, Abs, Negate, Perlin};

fn main() {
    let perlin = Perlin::new();
    let abs = Abs::new(&perlin);

    PlaneMapBuilder::new(&Negate::new(&abs))
        .build()
        .write_to_file("negate.png");
}
