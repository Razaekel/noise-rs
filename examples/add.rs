extern crate noise;

use noise::{utils::*, Cylinders, NoiseFn, Perlin};

mod utils;

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::default();

    // let add = Add::new(cyl, perlin);
    let add = cyl.add(perlin);

    utils::write_example_to_file(&PlaneMapBuilder::new(add).build(), "add.png");
}
