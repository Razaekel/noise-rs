extern crate noise;

use noise::utils::*;
use noise::{Cache, Checkerboard};

fn main() {
    let cboard = Checkerboard::new();
    let cache = Cache::new(&cboard);

    PlaneMapBuilder::new(&cache)
        .build()
        .write_to_file("cache.png");
}
