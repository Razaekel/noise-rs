extern crate noise;

use noise::{
    core::worley::{distance_functions::*, ReturnType},
    utils::*,
    Worley,
};

mod utils;

fn main() {
    utils::write_example_to_file(
        &PlaneMapBuilder::new(Worley::default()).build(),
        "worley.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new(Worley::default().set_return_type(ReturnType::Distance)).build(),
        "worley_distance.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new(Worley::default().set_distance_function(euclidean_squared)).build(),
        "worley_squared.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new(
            Worley::default()
                .set_return_type(ReturnType::Distance)
                .set_distance_function(euclidean_squared),
        )
        .build(),
        "worley_squared_distance.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new(Worley::default().set_distance_function(manhattan)).build(),
        "worley_manhattan.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new(
            Worley::default()
                .set_distance_function(manhattan)
                .set_return_type(ReturnType::Distance),
        )
        .build(),
        "worley_manhattan_distance.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new(Worley::default().set_distance_function(chebyshev)).build(),
        "worley_chebyshev.png",
    );

    utils::write_example_to_file(
        &PlaneMapBuilder::new(
            Worley::default()
                .set_return_type(ReturnType::Distance)
                .set_distance_function(chebyshev),
        )
        .build(),
        "worley_chebyshev_distance.png",
    );
}
