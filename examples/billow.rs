//! An example of using the Billow noise function

extern crate noise;

use noise::{utils::*, Billow, Perlin, Worley};

mod utils;

fn main() {
    let billow = Billow::<Perlin>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(billow).build(),
        "billow_perlin.png",
    );

    let billow = Billow::<Worley>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(billow).build(),
        "billow_worley.png",
    );

    let billow = Billow::<Billow<Perlin>>::default();

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(billow).build(),
        "billow_billow_perlin.png",
    );
}
