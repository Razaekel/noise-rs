//! An example of using the Billow noise function

extern crate noise;

use noise::Billow;

mod debug;

fn main() {
    debug::render_noise_module3("billow.png", &Billow::new(), 1024, 1024, 400);
}
