//! An example of using the `HybridMulti` noise function

extern crate noise;

use noise::{utils::*, HybridMulti, Perlin};

fn main() {
    let hybrid_multi : HybridMulti<Perlin>= HybridMulti::default();

    PlaneMapBuilder::new(&hybrid_multi)
        .build()
        .write_to_file("hybrid_multi.png");
}
