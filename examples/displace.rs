extern crate noise;

use noise::{utils::*, Checkerboard, Constant, Cylinders, Displace, Perlin};

fn main() {
    let cboard = Checkerboard::new();
    let constant = Constant::new(0.0);
    let cylinders = Cylinders::new();
    let perlin = Perlin::new();
    let displace = Displace::new(cylinders, cboard, perlin, constant, constant);

    PlaneMapBuilder::new(&displace)
        .build()
        .write_to_file("displace.png");
}
