#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    core::super_simplex::{super_simplex_2d, super_simplex_3d},
    math::vectors::{Vector2, Vector3},
    permutationtable::PermutationTable,
};

criterion_group!(super_simplex, bench_super_simplex2, bench_super_simplex3,);
criterion_group!(
    super_simplex_64x64,
    bench_super_simplex2_64x64,
    bench_super_simplex3_64x64,
);
criterion_main!(super_simplex, super_simplex_64x64);

fn bench_super_simplex2(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("super simplex 2d", |b| {
        b.iter(|| super_simplex_2d(black_box(Vector2::new(42.0_f64, 37.0)), &hasher))
    });
}

fn bench_super_simplex3(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("super simplex 3d", |b| {
        b.iter(|| super_simplex_3d(black_box(Vector3::new(42.0_f64, 37.0, 26.0)), &hasher))
    });
}

fn bench_super_simplex2_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("super simplex 2d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    super_simplex_2d(black_box(Vector2::new(x as f64, y as f64)), &hasher);
                }
            }
        })
    });
}

fn bench_super_simplex3_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("super simplex 3d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    super_simplex_3d(
                        black_box(Vector3::new(x as f64, y as f64, x as f64)),
                        &hasher,
                    );
                }
            }
        })
    });
}
