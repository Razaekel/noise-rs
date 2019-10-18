#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{NoiseFn, Perlin};

criterion_group!(perlin, bench_perlin2, bench_perlin3, bench_perlin4);
criterion_group!(
    perlin_64x64,
    bench_perlin2_64x64,
    //    bench_perlin3_64x64,
    //    bench_perlin4_64x64,
    bench_perlin2_basic_64x64,
    bench_perlin2_point_64x64,
    bench_perlin2_iterator_64x64,
    bench_perlin2_alt_iter_64x64,
    bench_perlin2_par_iter_64x64
);
criterion_main!(perlin, perlin_64x64);

fn bench_perlin2(c: &mut Criterion) {
    let perlin = Perlin::new();
    c.bench_function("perlin 2d", |b| {
        b.iter(|| perlin.get(black_box([42.5f64, 37.5])))
    });
}

fn bench_perlin3(c: &mut Criterion) {
    let perlin = Perlin::new();
    c.bench_function("perlin 3d", |b| {
        b.iter(|| perlin.get(black_box([42.5f64, 37.5, 26.5])))
    });
}

fn bench_perlin4(c: &mut Criterion) {
    let perlin = Perlin::new();
    c.bench_function("perlin 4d", |b| {
        b.iter(|| perlin.get(black_box([42.5f64, 37.5, 26.5, 128.5])))
    });
}

fn bench_perlin2_64x64(c: &mut Criterion) {
    let perlin = Perlin::new();

    //set up the array to process
    let mut points = Vec::new();

    for y in 0i8..64 {
        let y0 = y as f64 * 0.1;

        for x in 0i8..64 {
            let x0 = x as f64 * 0.1;

            points.push([x0, y0]);
        }
    }

    c.bench_function("perlin 2d (64x64)", |b| {
        b.iter(|| {
            for i in &points {
                black_box(perlin.get(*i));
            }
        })
    });
}

fn bench_perlin3_64x64(c: &mut Criterion) {
    let perlin = Perlin::new();

    //set up the array to process
    let mut points = Vec::new();

    for z in 0i8..64 {
        let z0 = z as f64 * 0.1;

        for y in 0i8..64 {
            let y0 = y as f64 * 0.1;

            for x in 0i8..64 {
                let x0 = x as f64 * 0.1;

                points.push([x0, y0, z0]);
            }
        }
    }

    c.bench_function("perlin 3d (64x64)", |b| {
        b.iter(|| {
            for i in &points {
                black_box(perlin.get(*i));
            }
        })
    });
}

fn bench_perlin4_64x64(c: &mut Criterion) {
    let perlin = Perlin::new();

    //set up the array to process
    let mut points = Vec::new();

    for u in 0i8..64 {
        let u0 = u as f64 * 0.1;

        for z in 0i8..64 {
            let z0 = z as f64 * 0.1;

            for y in 0i8..64 {
                let y0 = y as f64 * 0.1;

                for x in 0i8..64 {
                    let x0 = x as f64 * 0.1;

                    points.push([x0, y0, z0, u0]);
                }
            }
        }
    }

    c.bench_function("perlin 4d (64x64)", |b| {
        b.iter(|| {
            for i in &points {
                black_box(perlin.get(*i));
            }
        })
    });
}

fn bench_perlin2_basic_64x64(c: &mut Criterion) {
    let perlin = Perlin::new();

    //set up the array to process
    let mut points = Vec::new();

    for y in 0i8..64 {
        let y0 = y as f64 * 0.1;

        for x in 0i8..64 {
            let x0 = x as f64 * 0.1;

            points.push([x0, y0]);
        }
    }

    c.bench_function("perlin basic 2d (64x64)", |b| {
        b.iter(|| {
            for i in &points {
                black_box(noise::perlin_2d_basic(i[0], i[1], &perlin.perm_table));
            }
        })
    });
}

fn bench_perlin2_point_64x64(c: &mut Criterion) {
    let perlin = Perlin::new();

    //set up the array to process
    let mut points = Vec::new();

    for y in 0i8..64 {
        let y0 = y as f64 * 0.1;

        for x in 0i8..64 {
            let x0 = x as f64 * 0.1;

            points.push([x0, y0]);
        }
    }

    c.bench_function("perlin point 2d (64x64)", |b| {
        b.iter(|| {
            for i in &points {
                black_box(noise::perlin_2d_point(*i, &perlin.perm_table));
            }
        })
    });
}

fn bench_perlin2_iterator_64x64(c: &mut Criterion) {
    let perlin = Perlin::new();

    //set up the array to process
    let mut points = Vec::new();

    for y in 0i8..64 {
        let y0 = y as f64 * 0.1;

        for x in 0i8..64 {
            let x0 = x as f64 * 0.1;

            points.push([x0, y0]);
        }
    }

    c.bench_function("perlin iter 2d (64x64)", |b| {
        b.iter(|| black_box(noise::perlin_2d_iter(&points, &perlin.perm_table)))
    });
}

fn bench_perlin2_alt_iter_64x64(c: &mut Criterion) {
    let perlin = Perlin::new();

    //set up the array to process
    let mut points = Vec::new();

    for y in 0i8..64 {
        let y0 = y as f64 * 0.1;

        for x in 0i8..64 {
            let x0 = x as f64 * 0.1;

            points.push([x0, y0]);
        }
    }

    c.bench_function("perlin alt_iter 2d (64x64)", |b| {
        b.iter(|| black_box(noise::perlin_2d_alt_iter(&points, &perlin.perm_table)))
    });
}

fn bench_perlin2_par_iter_64x64(c: &mut Criterion) {
    let perlin = Perlin::new();

    //set up the array to process
    let mut points = Vec::new();

    for y in 0i8..64 {
        let y0 = y as f64 * 0.1;

        for x in 0i8..64 {
            let x0 = x as f64 * 0.1;

            points.push([x0, y0]);
        }
    }

    c.bench_function("perlin par_iter 2d (64x64)", |b| {
        b.iter(|| black_box(noise::perlin_2d_par_iter(&points, &perlin.perm_table)))
    });
}
