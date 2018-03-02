extern crate noise;

use noise::{Perlin, Power, Seedable};

mod debug;

fn main() {
    let perlin1 = Perlin::new();
    let perlin2 = Perlin::new().set_seed(1);

    debug::render_noise_module3(
        "power.png",
        &Power::new(&perlin1, &perlin2),
        1024,
        1024,
        100,
    );
}
