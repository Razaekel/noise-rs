extern crate noise;

use noise::{utils::*, Abs, Perlin};

fn main() {
    let perlin = Perlin::new();
    let abs = Abs::new(&perlin);

    PlaneMapBuilder::new(&abs).build().write_to_file("abs.png");
}
