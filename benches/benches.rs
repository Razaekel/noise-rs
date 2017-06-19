// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

#![feature(test)]

extern crate noise;
extern crate test;

use noise::{NoiseModule, OpenSimplex, Perlin, SuperSimplex, Value, Worley};
use test::{Bencher, black_box};

#[bench]
fn bench_open_simplex2(bencher: &mut Bencher) {
    let open_simplex = OpenSimplex::new();
    bencher.iter(|| open_simplex.get(black_box([42.0f64, 37.0])));
}

#[bench]
fn bench_open_simplex3(bencher: &mut Bencher) {
    let open_simplex = OpenSimplex::new();
    bencher.iter(|| open_simplex.get(black_box([42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_open_simplex4(bencher: &mut Bencher) {
    let open_simplex = OpenSimplex::new();
    bencher.iter(|| open_simplex.get(black_box([42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_perlin2(bencher: &mut Bencher) {
    let perlin = Perlin::new();
    bencher.iter(|| perlin.get(black_box([42.0f64, 37.0])));
}

#[bench]
fn bench_perlin3(bencher: &mut Bencher) {
    let perlin = Perlin::new();
    bencher.iter(|| perlin.get(black_box([42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_perlin4(bencher: &mut Bencher) {
    let perlin = Perlin::new();
    bencher.iter(|| perlin.get(black_box([42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_super_simplex2(bencher: &mut Bencher) {
    let super_simplex = SuperSimplex::new();
    bencher.iter(|| super_simplex.get(black_box([42.0f64, 37.0])));
}

#[bench]
fn bench_super_simplex3(bencher: &mut Bencher) {
    let super_simplex = SuperSimplex::new();
    bencher.iter(|| super_simplex.get(black_box([42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_value2(bencher: &mut Bencher) {
    let value = Value::new();
    bencher.iter(|| value.get(black_box([42.0f64, 37.0])));
}

#[bench]
fn bench_value3(bencher: &mut Bencher) {
    let value = Value::new();
    bencher.iter(|| value.get(black_box([42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_value4(bencher: &mut Bencher) {
    let value = Value::new();
    bencher.iter(|| value.get(black_box([42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_worley2_range(bencher: &mut Bencher) {
    let worley = Worley::new().enable_range(true);
    bencher.iter(|| worley.get(black_box([42.0f64, 37.0])));
}

#[bench]
fn bench_worley3_range(bencher: &mut Bencher) {
    let worley = Worley::new().enable_range(true);
    bencher.iter(|| worley.get(black_box([42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_worley4_range(bencher: &mut Bencher) {
    let worley = Worley::new().enable_range(true);
    bencher.iter(|| worley.get(black_box([42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_worley2_value(bencher: &mut Bencher) {
    let worley = Worley::new();
    bencher.iter(|| worley.get(black_box([42.0f64, 37.0])));
}

#[bench]
fn bench_worley3_value(bencher: &mut Bencher) {
    let worley = Worley::new();
    bencher.iter(|| worley.get(black_box([42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_worley4_value(bencher: &mut Bencher) {
    let worley = Worley::new();
    bencher.iter(|| worley.get(black_box([42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_open_simplex2_64x64(bencher: &mut Bencher) {
    let open_simplex = OpenSimplex::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(open_simplex.get([x as f64, y as f64]));
                     }
                 });
}

#[bench]
fn bench_open_simplex3_64x64(bencher: &mut Bencher) {
    let open_simplex = OpenSimplex::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(open_simplex.get([x as f64, y as f64, x as f64]));
                     }
                 });
}

#[bench]
fn bench_open_simplex4_64x64(bencher: &mut Bencher) {
    let open_simplex = OpenSimplex::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(open_simplex.get([x as f64, y as f64, x as f64, y as f64]));
                     }
                 });
}

#[bench]
fn bench_perlin2_64x64(bencher: &mut Bencher) {
    let perlin = Perlin::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(perlin.get([x as f64, y as f64]));
                     }
                 });
}

#[bench]
fn bench_perlin3_64x64(bencher: &mut Bencher) {
    let perlin = Perlin::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(perlin.get([x as f64, y as f64, x as f64]));
                     }
                 });
}

#[bench]
fn bench_perlin4_64x64(bencher: &mut Bencher) {
    let perlin = Perlin::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(perlin.get([x as f64, y as f64, x as f64, y as f64]));
                     }
                 });
}

#[bench]
fn bench_super_simplex2_64x64(bencher: &mut Bencher) {
    let super_simplex = SuperSimplex::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(super_simplex.get([x as f64, y as f64]));
                     }
                 });
}

#[bench]
fn bench_super_simplex3_64x64(bencher: &mut Bencher) {
    let super_simplex = SuperSimplex::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(super_simplex.get([x as f64, y as f64, x as f64]));
                     }
                 });
}

#[bench]
fn bench_value2_64x64(bencher: &mut Bencher) {
    let value = Value::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(value.get([x as f64, y as f64]));
                     }
                 });
}

#[bench]
fn bench_value3_64x64(bencher: &mut Bencher) {
    let value = Value::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(value.get([x as f64, y as f64, x as f64]));
                     }
                 });
}

#[bench]
fn bench_value4_64x64(bencher: &mut Bencher) {
    let value = Value::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(value.get([x as f64, y as f64, x as f64, y as f64]));
                     }
                 });
}

#[bench]
fn bench_worley2_range_64x64(bencher: &mut Bencher) {
    let worley = Worley::new().enable_range(true);
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(worley.get([x as f64, y as f64]));
                     }
                 });
}

#[bench]
fn bench_worley3_range_64x64(bencher: &mut Bencher) {
    let worley = Worley::new().enable_range(true);
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(worley.get([x as f64, y as f64, x as f64]));
                     }
                 });
}

#[bench]
fn bench_worley4_range_64x64(bencher: &mut Bencher) {
    let worley = Worley::new().enable_range(true);
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(worley.get([x as f64, y as f64, x as f64, y as f64]));
                     }
                 });
}

#[bench]
fn bench_worley2_value_64x64(bencher: &mut Bencher) {
    let worley = Worley::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(worley.get([x as f64, y as f64]));
                     }
                 });
}

#[bench]
fn bench_worley3_value_64x64(bencher: &mut Bencher) {
    let worley = Worley::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(worley.get([x as f64, y as f64, x as f64]));
                     }
                 });
}

#[bench]
fn bench_worley4_value_64x64(bencher: &mut Bencher) {
    let worley = Worley::new();
    bencher.iter(|| for y in 0i8..64 {
                     for x in 0i8..64 {
                         black_box(worley.get([x as f64, y as f64, x as f64, y as f64]));
                     }
                 });
}
