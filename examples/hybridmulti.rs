//! An example of using the `HybridMulti` noise function

extern crate noise;

use noise::{utils::*, HybridMulti, Perlin, Worley};

mod utils;

fn main() {
    let hybrid_multi = HybridMulti::<Perlin>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::new(hybrid_multi).build(),
        "hybrid_multi_perlin.png",
    );

    let hybrid_multi = HybridMulti::<Worley>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::new(hybrid_multi).build(),
        "hybrid_multi_worley.png",
    );

    let hybrid_multi = HybridMulti::<HybridMulti<Perlin>>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::new(hybrid_multi).build(),
        "hybrid_multi_hybrid_multi_perlin.png",
    );
}
