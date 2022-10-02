extern crate noise;

use noise::{utils::*, Cylinders, Max, Perlin};

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::default();
    let max = Max::new(cyl, perlin);

    PlaneMapBuilder::<_, 2>::new(max)
        .build()
        .write_to_file("max.png");
}
