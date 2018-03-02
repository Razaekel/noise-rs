//! An example of using simplex noise

extern crate noise;

use noise::{Curve, OpenSimplex, Seedable};

mod debug;

fn main() {
    debug::render_noise_module2(
        "open_simplex_scaled2.png",
        &Curve::new(&OpenSimplex::new())
            .add_control_point(-1.0, -1.0)
            .add_control_point(-0.2, -1.0)
            .add_control_point(0.2, 1.0)
            .add_control_point(1.0, 1.0),
        1024,
        1024,
        128,
    );
    debug::render_noise_module3(
        "open_simplex_scaled3.png",
        &Curve::new(&OpenSimplex::new())
            .add_control_point(-1.0, -1.0)
            .add_control_point(-0.2, -1.0)
            .add_control_point(0.2, 1.0)
            .add_control_point(1.0, 1.0),
        1024,
        1024,
        128,
    );
    debug::render_noise_module4(
        "open_simplex_scaled4.png",
        &Curve::new(&OpenSimplex::new())
            .add_control_point(-1.0, -1.0)
            .add_control_point(-0.2, -1.0)
            .add_control_point(0.2, 1.0)
            .add_control_point(1.0, 1.0),
        1024,
        1024,
        128,
    );
    debug::render_noise_module2("open_simplex2.png", &OpenSimplex::new(), 1024, 1024, 50);
    debug::render_noise_module3("open_simplex3.png", &OpenSimplex::new(), 1024, 1024, 50);
    debug::render_noise_module4("open_simplex4.png", &OpenSimplex::new(), 1024, 1024, 50);
    debug::render_noise_module2(
        "open_simplex_seeded2.png",
        &OpenSimplex::new().set_seed(1),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3(
        "open_simplex_seeded3.png",
        &OpenSimplex::new().set_seed(1),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4(
        "open_simplex_seeded4.png",
        &OpenSimplex::new().set_seed(1),
        1024,
        1024,
        50,
    );
}
