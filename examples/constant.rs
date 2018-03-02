//! An example of generating constant valued noise

extern crate noise;

use noise::Constant;

mod debug;

fn main() {
    debug::render_noise_module3("constant1.png", &Constant::new(-1.0), 1024, 1024, 1);
    debug::render_noise_module3("constant2.png", &Constant::new(0.0), 1024, 1024, 1);
    debug::render_noise_module3("constant3.png", &Constant::new(1.0), 1024, 1024, 1);
}
