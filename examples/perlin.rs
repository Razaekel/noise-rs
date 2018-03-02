//! An example of using perlin noise

extern crate noise;

use noise::{Perlin, Seedable};

mod debug;

fn main() {
    debug::render_noise_module2("perlin_2d.png", &Perlin::new(), 1024, 1024, 50);
    debug::render_noise_module2(
        "perlin_2d_seeded.png",
        &Perlin::new().set_seed(1),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3("perlin_3d.png", &Perlin::new(), 1024, 1024, 50);
    debug::render_noise_module3(
        "perlin_3d_seeded.png",
        &Perlin::new().set_seed(1),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4("perlin_4d.png", &Perlin::new(), 1024, 1024, 50);
    debug::render_noise_module4(
        "perlin_4d_seeded.png",
        &Perlin::new().set_seed(1),
        1024,
        1024,
        50,
    );
}
