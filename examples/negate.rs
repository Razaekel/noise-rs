extern crate noise;

use noise::{utils::*, Abs, Negate, Perlin};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let abs = Abs::new(perlin);

    utils::write_example_to_file(
        &PlaneMapBuilder::new(Negate::new(abs)).build(),
        "negate.png",
    );
}
