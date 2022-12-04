extern crate noise;

use noise::{utils::*, Perlin, Power, Seedable};

fn main() {
    let perlin1 = Perlin::default();
    let perlin2 = Perlin::new(1);
    let power = Power::new(perlin1, perlin2);

    PlaneMapBuilder::<_, 2>::new(power)
        .build()
        .write_to_file("power.png");
}
