extern crate noise;

use noise::{utils::*, Add, Cylinders, Perlin};

mod utils;

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::default();

    let add = Add::new(cyl, perlin);

    utils::write_example_to_file(&PlaneMapBuilder::<_, 2>::new(add).build(), "add.png");
}
