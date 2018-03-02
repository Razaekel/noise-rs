//! An example of generating constant valued noise

extern crate noise;

use noise::Checkerboard;

mod debug;

fn main() {
    debug::render_noise_module3("checkerboard.png", &Checkerboard::new(), 1024, 1024, 100);
    debug::render_noise_module3(
        "checkerboard-2.png",
        &Checkerboard::new().set_size(2),
        1024,
        1024,
        100,
    );
}
