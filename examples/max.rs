extern crate noise;

use noise::{utils::*, Cylinders, Max, Perlin};

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::new();
    let max = Max::new(&cyl, &perlin);

    PlaneMapBuilder::new(&max).build().write_to_file("max.png");
}
