#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    core::worley::{distance_functions::*, ReturnType, worley_3d},
    permutationtable::PermutationTable,
};

criterion_group!(bench_worley_3d,
    bench_worley3d_euclidean_value,
    bench_worley3d_euclidean_range,
    bench_worley3d_squared_value,
    bench_worley3d_squared_range,
    bench_worley3d_manhattan_value,
    bench_worley3d_manhattan_range,
    bench_worley3d_chebyshev_value,
    bench_worley3d_chebyshev_range,
    bench_worley3d_quadratic_value,
    bench_worley3d_quadratic_range,
);
criterion_main!(bench_worley_3d);

fn bench_worley3d_euclidean_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d euclidean value", |b| {
        b.iter(|| worley_3d(&hasher, euclidean, ReturnType::Value, black_box([42.0f64, 37.0, 26.0])))
    });
}

fn bench_worley3d_euclidean_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d euclidean distance", |b| {
        b.iter(|| worley_3d(&hasher, euclidean, ReturnType::Distance, black_box([42.0f64, 37.0, 26.0])))
    });
}

fn bench_worley3d_squared_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d squared value", |b| {
        b.iter(|| worley_3d(&hasher, euclidean_squared, ReturnType::Value, black_box([42.0f64, 37.0, 26.0])))
    });
}

fn bench_worley3d_squared_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d squared distance", |b| {
        b.iter(|| worley_3d(&hasher, euclidean_squared, ReturnType::Distance, black_box([42.0f64, 37.0, 26.0])))
    });
}

fn bench_worley3d_manhattan_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d manhattan value", |b| {
        b.iter(|| worley_3d(&hasher, manhattan, ReturnType::Value, black_box([42.0f64, 37.0, 26.0])))
    });
}

fn bench_worley3d_manhattan_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d manhattan distance", |b| {
        b.iter(|| worley_3d(&hasher, manhattan, ReturnType::Distance, black_box([42.0f64, 37.0, 26.0])))
    });
}

fn bench_worley3d_chebyshev_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d chebyshev value", |b| {
        b.iter(|| worley_3d(&hasher, chebyshev, ReturnType::Value, black_box([42.0f64, 37.0, 26.0])))
    });
}

fn bench_worley3d_chebyshev_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d chebyshev distance", |b| {
        b.iter(|| worley_3d(&hasher, chebyshev, ReturnType::Distance, black_box([42.0f64, 37.0, 26.0])))
    });
}

fn bench_worley3d_quadratic_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d quadratic value", |b| {
        b.iter(|| worley_3d(&hasher, quadratic, ReturnType::Value, black_box([42.0f64, 37.0, 26.0])))
    });
}

fn bench_worley3d_quadratic_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d quadratic distance", |b| {
        b.iter(|| worley_3d(&hasher, quadratic, ReturnType::Distance, black_box([42.0f64, 37.0, 26.0])))
    });
}
