extern crate noise;

use noise::{
    utils::*,
    Checkerboard,
    Constant,
    Cylinders,
    Perlin,
    Select,
};

mod utils;

fn main() {
    let checkerboard = Checkerboard::default();
    let cylinders = Cylinders::new();
    let perlin = Perlin::default();
    let constant = Constant::new(0.5);
    let select1 = Select::new(perlin, cylinders, checkerboard)
        .set_bounds(0.0, 1.0)
        .set_falloff(0.5);
    let select2 = Select::new(perlin, constant, checkerboard)
        .set_bounds(0.0, 1.0)
        .set_falloff(0.0);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(select1)
            .set_x_bounds(-1.0, 1.0)
            .set_y_bounds(-1.0, 1.0)
            .build(),
        "select1.png",
    );
    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(select2).build(),
        "select2.png",
    );
}
