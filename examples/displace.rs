extern crate noise;

use noise::{utils::*, Checkerboard, Constant, Cylinders, Displace, Perlin};

fn main() {
    let cboard = Checkerboard::default();
    let constant = Constant::new(0.0);
    let cylinders = Cylinders::new();
    let perlin = Perlin::default();
    let displace = Displace::new(cylinders, cboard, perlin, constant, constant);

    PlaneMapBuilder::<_, 2>::new(displace)
        .build()
        .write_to_file("displace.png");
}
