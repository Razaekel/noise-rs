#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    core::perlin::{perlin_2d, perlin_3d, perlin_4d},
    permutationtable::PermutationTable,
    Vector2, Vector3, Vector4,
};

criterion_group!(perlin, bench_perlin2, bench_perlin3, bench_perlin4);
criterion_group!(
    perlin_64x64,
    bench_perlin2_64x64,
    bench_perlin3_64x64,
    bench_perlin4_64x64
);
criterion_main!(perlin, perlin_64x64);

fn bench_perlin2(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin 2d", |b| {
        b.iter(|| perlin_2d(black_box(Vector2::new(42.0_f64, 37.0)), &hasher))
    });
}

fn bench_perlin3(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin 3d", |b| {
        b.iter(|| perlin_3d(black_box(Vector3::new(42.0_f64, 37.0, 26.0)), &hasher))
    });
}

fn bench_perlin4(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin 4d", |b| {
        b.iter(|| {
            perlin_4d(
                black_box(Vector4::new(42.0_f64, 37.0, 26.0, 128.0)),
                &hasher,
            )
        })
    });
}

fn bench_perlin2_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin 2d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    perlin_2d(black_box(Vector2::new(x as f64, y as f64)), &hasher);
                }
            }
        })
    });
}

fn bench_perlin3_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin 3d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    perlin_3d(
                        black_box(Vector3::new(x as f64, y as f64, x as f64)),
                        &hasher,
                    );
                }
            }
        })
    });
}

fn bench_perlin4_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin 4d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    perlin_4d(
                        black_box(Vector4::new(x as f64, y as f64, x as f64, y as f64)),
                        &hasher,
                    );
                }
            }
        })
    });
}
