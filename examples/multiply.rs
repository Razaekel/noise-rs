extern crate noise;

use noise::{utils::*, Cylinders, Multiply, Perlin};

fn main() {
    let cyl = Cylinders::new();
    let perlin = Perlin::default();
    let multiply = Multiply::new(cyl, perlin);

    PlaneMapBuilder::<_, 2>::new(multiply)
        .build()
        .write_to_file("multiply.png");
}
