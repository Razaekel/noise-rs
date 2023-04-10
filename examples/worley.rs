extern crate noise;

use noise::{
    core::worley::{distance_functions::*, worley_2d, worley_3d, worley_4d, ReturnType},
    permutationtable::PermutationTable,
    utils::*,
};

mod utils;

fn output_2d<F>(distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&[f64], &[f64]) -> f64,
{
    let hasher = PermutationTable::new(0);
    let closure = |point: [f64; 2], hasher: &PermutationTable| {
        worley_2d(hasher, distance_function, return_type, point)
    };
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| closure(point, &hasher))
            .set_size(256, 256)
            .build(),
        name,
    );
}

fn output_3d<F>(distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&[f64], &[f64]) -> f64,
{
    let hasher = PermutationTable::new(0);
    let closure = |point: [f64; 3], hasher: &PermutationTable| {
        worley_3d(hasher, distance_function, return_type, point)
    };
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| closure(point, &hasher))
            .set_size(256, 256)
            .build(),
        name,
    );
}

fn output_4d<F>(distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&[f64], &[f64]) -> f64,
{
    let hasher = PermutationTable::new(0);
    let closure = |point: [f64; 4], hasher: &PermutationTable| {
        worley_4d(hasher, distance_function, return_type, point)
    };
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| closure(point, &hasher))
            .set_size(256, 256)
            .build(),
        name,
    );
}

fn main() {
    output_2d(
        &euclidean,
        ReturnType::Value,
        "worley_2d_euclidean_value.png",
    );
    output_3d(
        &euclidean,
        ReturnType::Value,
        "worley_3d_euclidean_value.png",
    );
    output_4d(
        &euclidean,
        ReturnType::Value,
        "worley_4d_euclidean_value.png",
    );
    output_2d(
        &euclidean,
        ReturnType::Distance,
        "worley_2d_euclidean_distance.png",
    );
    output_3d(
        &euclidean,
        ReturnType::Distance,
        "worley_3d_euclidean_distance.png",
    );
    output_4d(
        &euclidean,
        ReturnType::Distance,
        "worley_4d_euclidean_distance.png",
    );
    output_2d(
        &euclidean_squared,
        ReturnType::Value,
        "worley_2d_euclidean_squared_value.png",
    );
    output_3d(
        &euclidean_squared,
        ReturnType::Value,
        "worley_3d_euclidean_squared_value.png",
    );
    output_4d(
        &euclidean_squared,
        ReturnType::Value,
        "worley_4d_euclidean_squared_value.png",
    );
    output_2d(
        &euclidean_squared,
        ReturnType::Distance,
        "worley_2d_euclidean_squared_distance.png",
    );
    output_3d(
        &euclidean_squared,
        ReturnType::Distance,
        "worley_3d_euclidean_squared_distance.png",
    );
    output_4d(
        &euclidean_squared,
        ReturnType::Distance,
        "worley_4d_euclidean_squared_distance.png",
    );
    output_2d(
        &manhattan,
        ReturnType::Value,
        "worley_2d_manhattan_value.png",
    );
    output_3d(
        &manhattan,
        ReturnType::Value,
        "worley_3d_manhattan_value.png",
    );
    output_4d(
        &manhattan,
        ReturnType::Value,
        "worley_4d_manhattan_value.png",
    );
    output_2d(
        &manhattan,
        ReturnType::Distance,
        "worley_2d_manhattan_distance.png",
    );
    output_3d(
        &manhattan,
        ReturnType::Distance,
        "worley_3d_manhattan_distance.png",
    );
    output_4d(
        &manhattan,
        ReturnType::Distance,
        "worley_4d_manhattan_distance.png",
    );
    output_2d(
        &chebyshev,
        ReturnType::Value,
        "worley_2d_chebyshev_value.png",
    );
    output_3d(
        &chebyshev,
        ReturnType::Value,
        "worley_3d_chebyshev_value.png",
    );
    output_4d(
        &chebyshev,
        ReturnType::Value,
        "worley_4d_chebyshev_value.png",
    );
    output_2d(
        &chebyshev,
        ReturnType::Distance,
        "worley_2d_chebyshev_distance.png",
    );
    output_3d(
        &chebyshev,
        ReturnType::Distance,
        "worley_3d_chebyshev_distance.png",
    );
    output_4d(
        &chebyshev,
        ReturnType::Distance,
        "worley_4d_chebyshev_distance.png",
    );
    output_2d(
        &quadratic,
        ReturnType::Value,
        "worley_2d_quadratic_value.png",
    );
    output_3d(
        &quadratic,
        ReturnType::Value,
        "worley_3d_quadratic_value.png",
    );
    output_4d(
        &quadratic,
        ReturnType::Value,
        "worley_4d_quadratic_value.png",
    );
    output_2d(
        &quadratic,
        ReturnType::Distance,
        "worley_2d_quadratic_distance.png",
    );
    output_3d(
        &quadratic,
        ReturnType::Distance,
        "worley_3d_quadratic_distance.png",
    );
    output_4d(
        &quadratic,
        ReturnType::Distance,
        "worley_4d_quadratic_distance.png",
    );
}
