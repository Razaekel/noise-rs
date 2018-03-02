extern crate noise;

use noise::{Cache, Checkerboard};

mod debug;

fn main() {
    let cboard = Checkerboard::new();

    debug::render_noise_module3("cache.png", &Cache::new(cboard), 1024, 1024, 100);
}
