extern crate noise;

use noise::{utils::*, Checkerboard, Constant, Cylinders, Perlin, Select};

fn main() {
    let checkerboard = &Checkerboard::new();
    let cylinders = &Cylinders::new();
    let perlin = &Perlin::new();
    let constant = &Constant::new(0.5);
    let select1 = Select::new(&perlin, &cylinders, &checkerboard)
        .set_bounds(0.0, 1.0)
        .set_falloff(0.5);
    let select2 = Select::new(&perlin, &constant, &checkerboard)
        .set_bounds(0.0, 1.0)
        .set_falloff(0.0);

    PlaneMapBuilder::new(&select1)
        .build()
        .write_to_file("select1.png");
    PlaneMapBuilder::new(&select2)
        .build()
        .write_to_file("select2.png");
}
