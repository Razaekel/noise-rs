extern crate noise;

use noise::{utils::*, Clamp, Perlin};

mod utils;

fn main() {
    let perlin = Perlin::default();
    let clamp = Clamp::new(perlin).set_lower_bound(0.0).set_upper_bound(0.5);

    utils::write_example_to_file(&PlaneMapBuilder::<_, 2>::new(clamp).build(), "clamp.png");
}
