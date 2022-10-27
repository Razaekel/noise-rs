extern crate noise;

use noise::{utils::*, Cylinders, Max, Perlin};

mod utils;

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::default();
    let max = Max::new(cyl, perlin);

    utils::write_example_to_file(&PlaneMapBuilder::<_, 2>::new(max).build(), "max.png");
}
