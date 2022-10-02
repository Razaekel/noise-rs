extern crate noise;

use noise::{
    core::worley::{distance_functions::*, ReturnType},
    utils::*,
    Worley,
};

fn main() {
    PlaneMapBuilder::<_, 2>::new(Worley::default())
        .build()
        .write_to_file("worley.png");

    PlaneMapBuilder::<_, 2>::new(Worley::default().set_return_type(ReturnType::Distance))
        .build()
        .write_to_file("worley_distance.png");

    PlaneMapBuilder::<_, 2>::new(Worley::default().set_distance_function(euclidean_squared))
        .build()
        .write_to_file("worley_squared.png");

    PlaneMapBuilder::<_, 2>::new(
        Worley::default()
            .set_return_type(ReturnType::Distance)
            .set_distance_function(euclidean_squared),
    )
    .build()
    .write_to_file("worley_squared_distance.png");

    PlaneMapBuilder::<_, 2>::new(Worley::default().set_distance_function(manhattan))
        .build()
        .write_to_file("worley_manhattan.png");

    PlaneMapBuilder::<_, 2>::new(
        Worley::default()
            .set_distance_function(manhattan)
            .set_return_type(ReturnType::Distance),
    )
    .build()
    .write_to_file("worley_manhattan_distance.png");

    PlaneMapBuilder::<_, 2>::new(Worley::default().set_distance_function(chebyshev))
        .build()
        .write_to_file("worley_chebyshev.png");

    PlaneMapBuilder::<_, 2>::new(
        Worley::default()
            .set_return_type(ReturnType::Distance)
            .set_distance_function(chebyshev),
    )
    .build()
    .write_to_file("worley_chebyshev_distance.png");
}
