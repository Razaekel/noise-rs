//! An example of using the `HybridMulti` noise function

extern crate noise;

use noise::HybridMulti;

mod debug;

fn main() {
    debug::render_noise_module3("hybridmulti.png", &HybridMulti::new(), 1024, 1024, 200);
}
