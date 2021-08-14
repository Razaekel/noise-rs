#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{NoiseFn, Value};

criterion_group!(value, bench_value2, bench_value3, bench_value4);
criterion_group!(
    value_64x64,
    bench_value2_64x64,
    bench_value3_64x64,
    bench_value4_64x64
);
criterion_main!(value, value_64x64);

fn bench_value2(c: &mut Criterion) {
    let value = Value::new(42);
    c.bench_function("super simplex 4d", |b| {
        b.iter(|| value.get(black_box([42.0_f64, 37.0])))
    });
}

fn bench_value3(c: &mut Criterion) {
    let value = Value::new(42);
    c.bench_function("value 3d", |b| {
        b.iter(|| value.get(black_box([42.0_f64, 37.0, 26.0])))
    });
}

fn bench_value4(c: &mut Criterion) {
    let value = Value::new(42);
    c.bench_function("value 4d", |b| {
        b.iter(|| value.get(black_box([42.0_f64, 37.0, 26.0, 128.0])))
    });
}

fn bench_value2_64x64(c: &mut Criterion) {
    let value = Value::new(42);
    c.bench_function("value 2d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(value.get([x as f64, y as f64]));
                }
            }
        })
    });
}

fn bench_value3_64x64(c: &mut Criterion) {
    let value = Value::new(42);
    c.bench_function("value 3d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(value.get([x as f64, y as f64, x as f64]));
                }
            }
        })
    });
}

fn bench_value4_64x64(c: &mut Criterion) {
    let value = Value::new(42);
    c.bench_function("value 4d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(value.get([x as f64, y as f64, x as f64, y as f64]));
                }
            }
        })
    });
}
