//! An example of using the `BasicMulti` noise function

extern crate noise;

use noise::BasicMulti;

mod debug;

fn main() {
    debug::render_noise_module3("basicmulti.png", &BasicMulti::new(), 1024, 1024, 100);
}
