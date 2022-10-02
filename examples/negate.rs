extern crate noise;

use noise::{utils::*, Abs, Negate, Perlin};

fn main() {
    let perlin = Perlin::default();
    let abs = Abs::new(perlin);

    PlaneMapBuilder::<_, 2>::new(Negate::new(abs))
        .build()
        .write_to_file("negate.png");
}
