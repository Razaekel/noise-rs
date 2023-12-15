extern crate noise;

use noise::{utils::*, Checkerboard, Constant, Cylinders, Displace, Perlin};

mod utils;

fn main() {
    let cboard = Checkerboard::default();
    let constant = Constant::new(0.0);
    let cylinders = Cylinders::new();
    let perlin = Perlin::default();
    let displace = Displace::new(cylinders, cboard, perlin, constant, constant);

    utils::write_example_to_file(&PlaneMapBuilder::new(displace).build(), "displace.png");
}
