extern crate noise;

use noise::{utils::*, Add, Cylinders, Perlin};

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::default();

    let add = Add::new(cyl, perlin);

    PlaneMapBuilder::<_, 2>::new(add)
        .build()
        .write_to_file("add.png");
}
