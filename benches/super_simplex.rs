#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{NoiseFn, SuperSimplex};

criterion_group!(super_simplex, bench_super_simplex2, bench_super_simplex3);
criterion_group!(
    super_simplex_64x64,
    bench_super_simplex2_64x64,
    bench_super_simplex3_64x64
);
criterion_main!(super_simplex, super_simplex_64x64);

fn bench_super_simplex2(c: &mut Criterion) {
    let super_simplex = SuperSimplex::new(42);
    c.bench_function("super simplex 2d", |b| {
        b.iter(|| super_simplex.get(black_box([42.0_f64, 37.0])))
    });
}

fn bench_super_simplex3(c: &mut Criterion) {
    let super_simplex = SuperSimplex::new(42);
    c.bench_function("super simplex 3d", |b| {
        b.iter(|| super_simplex.get(black_box([42.0_f64, 37.0, 26.0])))
    });
}

fn bench_super_simplex2_64x64(c: &mut Criterion) {
    let super_simplex = SuperSimplex::new(42);
    c.bench_function("super simplex 2d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(super_simplex.get([x as f64, y as f64]));
                }
            }
        })
    });
}

fn bench_super_simplex3_64x64(c: &mut Criterion) {
    let super_simplex = SuperSimplex::new(42);
    c.bench_function("super simplex 3d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(super_simplex.get([x as f64, y as f64, x as f64]));
                }
            }
        })
    });
}
