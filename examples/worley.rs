//! An example of using perlin noise

extern crate noise;

use noise::{RangeFunction, Worley};

mod debug;

fn main() {
    debug::render_noise_module2("worley-linear_2d.png", &Worley::new(), 1024, 1024, 50);
    debug::render_noise_module3("worley-linear_3d.png", &Worley::new(), 1024, 1024, 50);
    debug::render_noise_module4("worley-linear_4d.png", &Worley::new(), 1024, 1024, 50);

    debug::render_noise_module2(
        "worley-linear-range_2d.png",
        &Worley::new().enable_range(true),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3(
        "worley-linear-range_3d.png",
        &Worley::new().enable_range(true),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4(
        "worley-linear-range_4d.png",
        &Worley::new().enable_range(true),
        1024,
        1024,
        50,
    );

    debug::render_noise_module2(
        "worley-linear-squared_2d.png",
        &Worley::new().set_range_function(RangeFunction::EuclideanSquared),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3(
        "worley-linear-squared_3d.png",
        &Worley::new().set_range_function(RangeFunction::EuclideanSquared),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4(
        "worley-linear-squared_4d.png",
        &Worley::new().set_range_function(RangeFunction::EuclideanSquared),
        1024,
        1024,
        50,
    );

    debug::render_noise_module2(
        "worley-manhattan_2d.png",
        &Worley::new().set_range_function(RangeFunction::Manhattan),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3(
        "worley-manhattan_3d.png",
        &Worley::new().set_range_function(RangeFunction::Manhattan),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4(
        "worley-manhattan_4d.png",
        &Worley::new().set_range_function(RangeFunction::Manhattan),
        1024,
        1024,
        50,
    );

    debug::render_noise_module2(
        "worley-manhattan-range_2d.png",
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::Manhattan),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3(
        "worley-manhattan-range_3d.png",
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::Manhattan),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4(
        "worley-manhattan-range_4d.png",
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::Manhattan),
        1024,
        1024,
        50,
    );

    debug::render_noise_module2(
        "worley-chebyshev_2d.png",
        &Worley::new().set_range_function(RangeFunction::Chebyshev),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3(
        "worley-chebyshev_3d.png",
        &Worley::new().set_range_function(RangeFunction::Chebyshev),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4(
        "worley-chebyshev_4d.png",
        &Worley::new().set_range_function(RangeFunction::Chebyshev),
        1024,
        1024,
        50,
    );

    debug::render_noise_module2(
        "worley-chebyshev-range_2d.png",
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::Chebyshev),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3(
        "worley-chebyshev-range_3d.png",
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::Chebyshev),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4(
        "worley-chebyshev-range_4d.png",
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::Chebyshev),
        1024,
        1024,
        50,
    );

    debug::render_noise_module2(
        "worley-quadratic_2d.png",
        &Worley::new().set_range_function(RangeFunction::Quadratic),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3(
        "worley-quadratic_3d.png",
        &Worley::new().set_range_function(RangeFunction::Quadratic),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4(
        "worley-quadratic_4d.png",
        &Worley::new().set_range_function(RangeFunction::Quadratic),
        1024,
        1024,
        50,
    );

    debug::render_noise_module2(
        "worley-quadratic-range_2d.png",
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::Quadratic),
        1024,
        1024,
        50,
    );
    debug::render_noise_module3(
        "worley-quadratic-range_3d.png",
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::Quadratic),
        1024,
        1024,
        50,
    );
    debug::render_noise_module4(
        "worley-quadratic-range_4d.png",
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::Quadratic),
        1024,
        1024,
        50,
    );
}
