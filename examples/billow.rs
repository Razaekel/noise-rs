//! An example of using the Billow noise function

extern crate noise;

use noise::{utils::*, Billow, Perlin, Worley};

fn main() {
    let billow = Billow::<Perlin>::default();

    PlaneMapBuilder::<_, 2>::new(billow)
        .build()
        .write_to_file("billow_perlin.png");

    let billow = Billow::<Worley>::default();

    PlaneMapBuilder::<_, 2>::new(billow)
        .build()
        .write_to_file("billow_worley.png");

    let billow = Billow::<Billow<Perlin>>::default();

    PlaneMapBuilder::<_, 2>::new(billow)
        .build()
        .write_to_file("billow_billow_perlin.png");
}
