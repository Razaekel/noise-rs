#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{NoiseFn, Perlin};

criterion_group!(perlin, bench_perlin2, bench_perlin3, bench_perlin4);
criterion_group!(
    perlin_64x64,
    bench_perlin2_64x64,
    bench_perlin3_64x64,
    bench_perlin4_64x64
);
criterion_main!(perlin, perlin_64x64);

fn bench_perlin2(c: &mut Criterion) {
    let perlin = Perlin::default();
    c.bench_function("perlin 2d", |b| {
        b.iter(|| perlin.get(black_box([42.0_f64, 37.0])))
    });
}

fn bench_perlin3(c: &mut Criterion) {
    let perlin = Perlin::default();
    c.bench_function("perlin 3d", |b| {
        b.iter(|| perlin.get(black_box([42.0_f64, 37.0, 26.0])))
    });
}

fn bench_perlin4(c: &mut Criterion) {
    let perlin = Perlin::default();
    c.bench_function("perlin 4d", |b| {
        b.iter(|| perlin.get(black_box([42.0_f64, 37.0, 26.0, 128.0])))
    });
}

fn bench_perlin2_64x64(c: &mut Criterion) {
    let perlin = Perlin::default();
    c.bench_function("perlin 2d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(perlin.get([x as f64, y as f64]));
                }
            }
        })
    });
}

fn bench_perlin3_64x64(c: &mut Criterion) {
    let perlin = Perlin::default();
    c.bench_function("perlin 3d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(perlin.get([x as f64, y as f64, x as f64]));
                }
            }
        })
    });
}

fn bench_perlin4_64x64(c: &mut Criterion) {
    let perlin = Perlin::default();
    c.bench_function("perlin 4d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(perlin.get([x as f64, y as f64, x as f64, y as f64]));
                }
            }
        })
    });
}
