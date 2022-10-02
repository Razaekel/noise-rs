//! An example of using the `HybridMulti` noise function

extern crate noise;

use noise::{utils::*, HybridMulti, Perlin, Worley};

fn main() {
    let hybrid_multi = HybridMulti::<Perlin>::default();

    PlaneMapBuilder::<_, 2>::new(hybrid_multi)
        .build()
        .write_to_file("hybrid_multi_perlin.png");

    let hybrid_multi = HybridMulti::<Worley>::default();

    PlaneMapBuilder::<_, 2>::new(hybrid_multi)
        .build()
        .write_to_file("hybrid_multi_worley.png");

    let hybrid_multi = HybridMulti::<HybridMulti<Perlin>>::default();

    PlaneMapBuilder::<_, 2>::new(hybrid_multi)
        .build()
        .write_to_file("hybrid_multi_hybrid_multi_perlin.png");
}
