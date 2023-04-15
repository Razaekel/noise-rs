#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    core::open_simplex::{open_simplex_2d, open_simplex_3d, open_simplex_4d},
    math::vectors::{Vector2, Vector3, Vector4},
    permutationtable::PermutationTable,
};

criterion_group!(
    open_simplex,
    bench_open_simplex2,
    bench_open_simplex3,
    bench_open_simplex4
);
criterion_group!(
    open_simplex_64x64,
    bench_open_simplex2_64x64,
    bench_open_simplex3_64x64,
    bench_open_simplex4_64x64
);
criterion_main!(open_simplex, open_simplex_64x64);

fn bench_open_simplex2(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("open simplex 2d", |b| {
        b.iter(|| black_box(open_simplex_2d([42.0_f64, 37.0], &hasher)))
    });
}

fn bench_open_simplex3(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("open simplex 3d", |b| {
        b.iter(|| black_box(open_simplex_3d([42.0_f64, 37.0, 26.0], &hasher)))
    });
}

fn bench_open_simplex4(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("open simplex 4d", |b| {
        b.iter(|| black_box(open_simplex_4d([42.0_f64, 37.0, 26.0, 128.0], &hasher)))
    });
}

fn bench_open_simplex2_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("open simplex 2d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(open_simplex_2d([x as f64, y as f64], &hasher));
                }
            }
        })
    });
}

fn bench_open_simplex3_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("open simplex 3d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(open_simplex_3d([x as f64, y as f64, x as f64], &hasher));
                }
            }
        })
    });
}

fn bench_open_simplex4_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("open simplex 4d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(open_simplex_4d(
                        [x as f64, y as f64, x as f64, y as f64],
                        &hasher,
                    ));
                }
            }
        })
    });
}
