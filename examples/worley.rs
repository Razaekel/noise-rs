extern crate noise;

use noise::{distance_functions::*, utils::*, ReturnType, Worley};

fn main() {
    PlaneMapBuilder::new(&Worley::default())
        .build()
        .write_to_file("worley.png");

    PlaneMapBuilder::new(&Worley::default().set_return_type(ReturnType::Distance))
        .build()
        .write_to_file("worley_distance.png");

    PlaneMapBuilder::new(&Worley::default().set_distance_function(euclidean_squared))
        .build()
        .write_to_file("worley_squared.png");

    PlaneMapBuilder::new(
        &Worley::default()
            .set_return_type(ReturnType::Distance)
            .set_distance_function(euclidean_squared),
    )
    .build()
    .write_to_file("worley_squared_distance.png");

    PlaneMapBuilder::new(&Worley::default().set_distance_function(manhattan))
        .build()
        .write_to_file("worley_manhattan.png");

    PlaneMapBuilder::new(&Worley::default().set_return_type(ReturnType::Distance))
        .build()
        .write_to_file("worley_manhattan_distance.png");

    PlaneMapBuilder::new(&Worley::default().set_distance_function(chebyshev))
        .build()
        .write_to_file("worley_chebyshev.png");

    PlaneMapBuilder::new(
        &Worley::default()
            .set_return_type(ReturnType::Distance)
            .set_distance_function(chebyshev),
    )
    .build()
    .write_to_file("worley_chebyshev_distance.png");
}
