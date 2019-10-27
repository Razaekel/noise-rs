#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{NoiseFn, Worley};

criterion_group!(
    worley_range,
    bench_worley2_range,
    bench_worley3_range,
    bench_worley4_range
);
criterion_group!(
    worley_value,
    bench_worley2_value,
    bench_worley3_value,
    bench_worley4_value
);
criterion_group!(
    worley_range_64x64,
    bench_worley2_range_64x64,
    bench_worley3_range_64x64,
    bench_worley4_range_64x64
);
criterion_group!(
    worley_value_64x64,
    bench_worley2_value_64x64,
    bench_worley3_value_64x64,
    bench_worley4_value_64x64
);
criterion_main!(
    worley_range,
    worley_value,
    worley_range_64x64,
    worley_value_64x64
);

fn bench_worley2_range(c: &mut Criterion) {
    let worley = Worley::new().enable_range(true);
    c.bench_function("worley 2d - range", |b| {
        b.iter(|| worley.get(black_box([42.0_f64, 37.0])))
    });
}

fn bench_worley3_range(c: &mut Criterion) {
    let worley = Worley::new().enable_range(true);
    c.bench_function("worley 3d - range", |b| {
        b.iter(|| worley.get(black_box([42.0_f64, 37.0, 26.0])))
    });
}

fn bench_worley4_range(c: &mut Criterion) {
    let worley = Worley::new().enable_range(true);
    c.bench_function("worley 4d - range", |b| {
        b.iter(|| worley.get(black_box([42.0_f64, 37.0, 26.0, 128.0])))
    });
}

fn bench_worley2_value(c: &mut Criterion) {
    let worley = Worley::new();
    c.bench_function("worley 2d - value", |b| {
        b.iter(|| worley.get(black_box([42.0_f64, 37.0])))
    });
}

fn bench_worley3_value(c: &mut Criterion) {
    let worley = Worley::new();
    c.bench_function("worley 3d - value", |b| {
        b.iter(|| worley.get(black_box([42.0_f64, 37.0, 26.0])))
    });
}

fn bench_worley4_value(c: &mut Criterion) {
    let worley = Worley::new();
    c.bench_function("worley 4d - value", |b| {
        b.iter(|| worley.get(black_box([42.0_f64, 37.0, 26.0, 128.0])))
    });
}

fn bench_worley2_range_64x64(c: &mut Criterion) {
    let worley = Worley::new().enable_range(true);
    c.bench_function("worley 2d - range (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley.get([x as f64, y as f64]));
                }
            }
        })
    });
}

fn bench_worley3_range_64x64(c: &mut Criterion) {
    let worley = Worley::new().enable_range(true);
    c.bench_function("worley 3d - range (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley.get([x as f64, y as f64, x as f64]));
                }
            }
        })
    });
}

fn bench_worley4_range_64x64(c: &mut Criterion) {
    let worley = Worley::new().enable_range(true);
    c.bench_function("worley 4d - range (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley.get([x as f64, y as f64, x as f64, y as f64]));
                }
            }
        })
    });
}

fn bench_worley2_value_64x64(c: &mut Criterion) {
    let worley = Worley::new();
    c.bench_function("worley 2d - value (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley.get([x as f64, y as f64]));
                }
            }
        })
    });
}

fn bench_worley3_value_64x64(c: &mut Criterion) {
    let worley = Worley::new();
    c.bench_function("worley 3d - value (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley.get([x as f64, y as f64, x as f64]));
                }
            }
        })
    });
}

fn bench_worley4_value_64x64(c: &mut Criterion) {
    let worley = Worley::new();
    c.bench_function("worley 4d - value (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley.get([x as f64, y as f64, x as f64, y as f64]));
                }
            }
        })
    });
}
