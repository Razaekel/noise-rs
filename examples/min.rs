extern crate noise;

use noise::{utils::*, Cylinders, Min, Perlin};

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::new();
    let min = Min::new(&cyl, &perlin);

    PlaneMapBuilder::new(&min).build().write_to_file("min.png");
}
