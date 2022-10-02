extern crate noise;

use noise::{utils::*, Cylinders, Min, Perlin};

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::default();
    let min = Min::new(cyl, perlin);

    PlaneMapBuilder::<_, 2>::new(min)
        .build()
        .write_to_file("min.png");
}
