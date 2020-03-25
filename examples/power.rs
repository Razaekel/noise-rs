extern crate noise;

use noise::{utils::*, Perlin, Power, Seedable};

fn main() {
    let perlin1 = Perlin::new();
    let perlin2 = Perlin::new().set_seed(1);
    let power = Power::new(&perlin1, &perlin2);

    PlaneMapBuilder::new(&power)
        .build()
        .write_to_file("power.png");
}
