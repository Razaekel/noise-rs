extern crate noise;

use noise::{utils::*, RangeFunction, Worley};

fn main() {
    PlaneMapBuilder::new(&Worley::new())
        .build()
        .write_to_file("worley.png");

    PlaneMapBuilder::new(&Worley::new().enable_range(true))
        .build()
        .write_to_file("worley_range.png");

    PlaneMapBuilder::new(&Worley::new().set_range_function(RangeFunction::EuclideanSquared))
        .build()
        .write_to_file("worley_squared.png");

    PlaneMapBuilder::new(
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::EuclideanSquared),
    )
    .build()
    .write_to_file("worley_squared_range.png");

    PlaneMapBuilder::new(&Worley::new().set_range_function(RangeFunction::Manhattan))
        .build()
        .write_to_file("worley_manhattan.png");

    PlaneMapBuilder::new(&Worley::new().enable_range(true))
        .build()
        .write_to_file("worley_manhattan_range.png");

    PlaneMapBuilder::new(&Worley::new().set_range_function(RangeFunction::Chebyshev))
        .build()
        .write_to_file("worley_chebyshev.png");

    PlaneMapBuilder::new(
        &Worley::new()
            .enable_range(true)
            .set_range_function(RangeFunction::Chebyshev),
    )
    .build()
    .write_to_file("worley_chebyshev_range.png");
}
