#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    core::value::{value_2d, value_3d, value_4d},
    math::vectors::{Vector2, Vector3, Vector4},
    permutationtable::PermutationTable,
};

criterion_group!(value, bench_value2, bench_value3, bench_value4);
criterion_group!(
    value_64x64,
    bench_value2_64x64,
    bench_value3_64x64,
    bench_value4_64x64
);
criterion_main!(value, value_64x64);

fn bench_value2(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("value 2d", |b| {
        b.iter(|| black_box(value_2d(Vector2::new(42.0_f64, 37.0), &hasher)))
    });
}

fn bench_value3(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("value 3d", |b| {
        b.iter(|| black_box(value_3d(Vector3::new(42.0_f64, 37.0, 26.0), &hasher)))
    });
}

fn bench_value4(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("value 4d", |b| {
        b.iter(|| black_box(value_4d(Vector4::new(42.0_f64, 37.0, 26.0, 128.0), &hasher)))
    });
}

fn bench_value2_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("value 2d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(value_2d(Vector2::new(x as f64, y as f64), &hasher));
                }
            }
        })
    });
}

fn bench_value3_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("value 3d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(value_3d(
                        Vector3::new(x as f64, y as f64, x as f64),
                        &hasher,
                    ));
                }
            }
        })
    });
}

fn bench_value4_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("value 4d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(value_4d(
                        Vector4::new(x as f64, y as f64, x as f64, y as f64),
                        &hasher,
                    ));
                }
            }
        })
    });
}
