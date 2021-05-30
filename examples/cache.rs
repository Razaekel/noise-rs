extern crate noise;

use noise::{utils::*, Cache, Perlin};

fn main() {
    let perlin = Perlin::default();
    let cache = Cache::new(&perlin);

    PlaneMapBuilder::new(&cache)
        .build()
        .write_to_file("cache.png");
}
