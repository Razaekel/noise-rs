extern crate noise;

use noise::{utils::*, Checkerboard, Constant, Cylinders, NoiseFn, Perlin};

mod utils;

fn main() {
    let checkerboard = Checkerboard::default();
    let cylinders = Cylinders::new();
    let perlin = Perlin::default();
    let constant = Constant::new(0.5);
    let select1 = perlin.select(cylinders, checkerboard, 0.0, 1.0, 0.5);
    let select2 = perlin.select(constant, checkerboard, 0.0, 1.0, 0.0);

    utils::write_example_to_file(
        &PlaneMapBuilder::new(select1)
            .set_x_bounds(-1.0, 1.0)
            .set_y_bounds(-1.0, 1.0)
            .build(),
        "select1.png",
    );
    utils::write_example_to_file(&PlaneMapBuilder::new(select2).build(), "select2.png");
}
