extern crate noise;

use noise::Cylinders;

mod debug;

fn main() {
    debug::render_noise_module3("cylinders.png", &Cylinders::new(), 1024, 1024, 50);
    debug::render_noise_module3(
        "cylinders-f5.png",
        &Cylinders::new().set_frequency(5.0),
        1024,
        1024,
        50,
    );
}
