//! An example of using the `HybridMulti` noise function

extern crate noise;

use noise::{utils::*, HybridMulti};

fn main() {
    let hybrid_multi = HybridMulti::default();

    PlaneMapBuilder::new(&hybrid_multi)
        .build()
        .write_to_file("hybrid_multi.png");
}
