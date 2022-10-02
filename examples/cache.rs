extern crate noise;

use noise::{utils::*, Cache, Checkerboard};

fn main() {
    let cboard = Checkerboard::default();
    let cache = Cache::new(cboard);

    PlaneMapBuilder::<_, 2>::new(cache)
        .build()
        .write_to_file("cache.png");
}
