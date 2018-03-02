//! An example of using value noise

extern crate noise;

use noise::{Seedable, Value};

mod debug;

fn main() {
    debug::render_noise_module2("value2.png", &Value::new(), 1024, 1024, 50);
    debug::render_noise_module2(
        "value2_seeded.png",
        &Value::new().set_seed(1),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3("value3.png", &Value::new(), 1024, 1024, 50);
    debug::render_noise_module3(
        "value3_seeded.png",
        &Value::new().set_seed(1),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4("value4.png", &Value::new(), 1024, 1024, 50);
    debug::render_noise_module4(
        "value4_seeded.png",
        &Value::new().set_seed(1),
        1024,
        1024,
        50,
    );
}
