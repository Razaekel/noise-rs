extern crate noise;

use noise::{utils::*, Cache, Checkerboard};

mod utils;

fn main() {
    let cboard = Checkerboard::default();
    let cache = Cache::new(cboard);

    utils::write_example_to_file(&PlaneMapBuilder::<_, 2>::new(cache).build(), "cache.png");
}
