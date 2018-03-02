//! An example of using the fBm noise function

extern crate noise;

use noise::Fbm;

mod debug;

fn main() {
    debug::render_noise_module3("fbm.png", &Fbm::new(), 1024, 1024, 400);
}
