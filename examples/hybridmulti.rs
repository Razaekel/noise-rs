//! An example of using the `HybridMulti` noise function

extern crate noise;

use noise::{utils::*, HybridMulti, MultiFractal};

fn main() {
    let hybrid_multi = HybridMulti::new().set_octaves(1);

    PlaneMapBuilder::new(&hybrid_multi)
        .set_size(1024, 1024)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build()
        .write_to_file("hybrid_multi_octave=1.png");

    let hybrid_multi = HybridMulti::new().set_octaves(2);

    PlaneMapBuilder::new(&hybrid_multi)
        .set_size(1024, 1024)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build()
        .write_to_file("hybrid_multi_octave=2.png");

    let hybrid_multi = HybridMulti::new().set_octaves(3);

    PlaneMapBuilder::new(&hybrid_multi)
        .set_size(1024, 1024)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build()
        .write_to_file("hybrid_multi_octave=3.png");

    let hybrid_multi = HybridMulti::new().set_octaves(4);

    PlaneMapBuilder::new(&hybrid_multi)
        .set_size(1024, 1024)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build()
        .write_to_file("hybrid_multi_octave=4.png");

    let hybrid_multi = HybridMulti::new().set_octaves(5);

    PlaneMapBuilder::new(&hybrid_multi)
        .set_size(1024, 1024)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build()
        .write_to_file("hybrid_multi_octave=5.png");

    let hybrid_multi = HybridMulti::new();

    PlaneMapBuilder::new(&hybrid_multi)
        .set_size(1024, 1024)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build()
        .write_to_file("hybrid_multi_octave=6.png");
}
