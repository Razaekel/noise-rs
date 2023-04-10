#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    core::worley::{distance_functions::*, ReturnType, worley_2d},
    permutationtable::PermutationTable,
};

criterion_group!(bench_worley_2d,
    bench_worley2d_euclidean_value,
    bench_worley2d_euclidean_range,
    bench_worley2d_squared_value,
    bench_worley2d_squared_range,
    bench_worley2d_manhattan_value,
    bench_worley2d_manhattan_range,
    bench_worley2d_chebyshev_value,
    bench_worley2d_chebyshev_range,
    bench_worley2d_quadratic_value,
    bench_worley2d_quadratic_range,
);
criterion_main!(bench_worley_2d);

fn bench_worley2d_euclidean_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d euclidean value", |b| {
        b.iter(|| worley_2d(&hasher, euclidean, ReturnType::Value, black_box([42.0f64, 37.0])))
    });
}

fn bench_worley2d_euclidean_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d euclidean distance", |b| {
        b.iter(|| worley_2d(&hasher, euclidean, ReturnType::Distance, black_box([42.0f64, 37.0])))
    });
}

fn bench_worley2d_squared_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d squared value", |b| {
        b.iter(|| worley_2d(&hasher, euclidean_squared, ReturnType::Value, black_box([42.0f64, 37.0])))
    });
}

fn bench_worley2d_squared_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d squared distance", |b| {
        b.iter(|| worley_2d(&hasher, euclidean_squared, ReturnType::Distance, black_box([42.0f64, 37.0])))
    });
}

fn bench_worley2d_manhattan_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d manhattan value", |b| {
        b.iter(|| worley_2d(&hasher, manhattan, ReturnType::Value, black_box([42.0f64, 37.0])))
    });
}

fn bench_worley2d_manhattan_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d manhattan distance", |b| {
        b.iter(|| worley_2d(&hasher, manhattan, ReturnType::Distance, black_box([42.0f64, 37.0])))
    });
}

fn bench_worley2d_chebyshev_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d chebyshev value", |b| {
        b.iter(|| worley_2d(&hasher, chebyshev, ReturnType::Value, black_box([42.0f64, 37.0])))
    });
}

fn bench_worley2d_chebyshev_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d chebyshev distance", |b| {
        b.iter(|| worley_2d(&hasher, chebyshev, ReturnType::Distance, black_box([42.0f64, 37.0])))
    });
}

fn bench_worley2d_quadratic_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d quadratic value", |b| {
        b.iter(|| worley_2d(&hasher, quadratic, ReturnType::Value, black_box([42.0f64, 37.0])))
    });
}

fn bench_worley2d_quadratic_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d quadratic distance", |b| {
        b.iter(|| worley_2d(&hasher, quadratic, ReturnType::Distance, black_box([42.0f64, 37.0])))
    });
}
