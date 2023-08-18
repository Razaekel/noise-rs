extern crate noise;

use noise::{
    core::worley::{distance_functions::*, worley_2d, worley_3d, worley_4d, ReturnType},
    permutationtable::PermutationTable,
    utils::*,
    Vector2, Vector3, Vector4,
};

mod utils;

fn output_2d<F>(distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&[f64], &[f64]) -> f64,
{
    let hasher = PermutationTable::new(0);
    let closure = |point: Vector2<f64>, hasher: &PermutationTable| {
        worley_2d(hasher, distance_function, return_type, point)
    };
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| closure(point.into(), &hasher))
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
    let closure = |point: Vector3<f64>, hasher: &PermutationTable| {
        worley_3d(hasher, distance_function, return_type, point)
    };
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| closure(point.into(), &hasher))
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
    let closure = |point: Vector4<f64>, hasher: &PermutationTable| {
        worley_4d(hasher, distance_function, return_type, point)
    };
    utils::write_example_to_file(
        &PlaneMapBuilder::new_fn(|point| closure(point.into(), &hasher))
            .set_size(256, 256)
            .build(),
        name,
    );
}

fn main() {
    output_2d(
        &euclidean,
        ReturnType::Value,
        "worley/2d_euclidean_value.png",
    );
    output_3d(
        &euclidean,
        ReturnType::Value,
        "worley/3d_euclidean_value.png",
    );
    output_4d(
        &euclidean,
        ReturnType::Value,
        "worley/4d_euclidean_value.png",
    );
    output_2d(
        &euclidean,
        ReturnType::Distance,
        "worley/2d_euclidean_distance.png",
    );
    output_3d(
        &euclidean,
        ReturnType::Distance,
        "worley/3d_euclidean_distance.png",
    );
    output_4d(
        &euclidean,
        ReturnType::Distance,
        "worley/4d_euclidean_distance.png",
    );
    output_2d(
        &euclidean_squared,
        ReturnType::Value,
        "worley/2d_euclidean_squared_value.png",
    );
    output_3d(
        &euclidean_squared,
        ReturnType::Value,
        "worley/3d_euclidean_squared_value.png",
    );
    output_4d(
        &euclidean_squared,
        ReturnType::Value,
        "worley/4d_euclidean_squared_value.png",
    );
    output_2d(
        &euclidean_squared,
        ReturnType::Distance,
        "worley/2d_euclidean_squared_distance.png",
    );
    output_3d(
        &euclidean_squared,
        ReturnType::Distance,
        "worley/3d_euclidean_squared_distance.png",
    );
    output_4d(
        &euclidean_squared,
        ReturnType::Distance,
        "worley/4d_euclidean_squared_distance.png",
    );
    output_2d(
        &manhattan,
        ReturnType::Value,
        "worley/2d_manhattan_value.png",
    );
    output_3d(
        &manhattan,
        ReturnType::Value,
        "worley/3d_manhattan_value.png",
    );
    output_4d(
        &manhattan,
        ReturnType::Value,
        "worley/4d_manhattan_value.png",
    );
    output_2d(
        &manhattan,
        ReturnType::Distance,
        "worley/2d_manhattan_distance.png",
    );
    output_3d(
        &manhattan,
        ReturnType::Distance,
        "worley/3d_manhattan_distance.png",
    );
    output_4d(
        &manhattan,
        ReturnType::Distance,
        "worley/4d_manhattan_distance.png",
    );
    output_2d(
        &chebyshev,
        ReturnType::Value,
        "worley/2d_chebyshev_value.png",
    );
    output_3d(
        &chebyshev,
        ReturnType::Value,
        "worley/3d_chebyshev_value.png",
    );
    output_4d(
        &chebyshev,
        ReturnType::Value,
        "worley/4d_chebyshev_value.png",
    );
    output_2d(
        &chebyshev,
        ReturnType::Distance,
        "worley/2d_chebyshev_distance.png",
    );
    output_3d(
        &chebyshev,
        ReturnType::Distance,
        "worley/3d_chebyshev_distance.png",
    );
    output_4d(
        &chebyshev,
        ReturnType::Distance,
        "worley/4d_chebyshev_distance.png",
    );
}
