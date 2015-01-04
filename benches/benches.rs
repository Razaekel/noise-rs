// Copyright 2013 The noise-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of using simplectic noise

#![feature(macro_rules)]
#![feature(asm)]

extern crate noise;
extern crate test;

use noise::{perlin2, perlin3, perlin4, simplex2, simplex3, simplectic2, simplectic3, simplectic4, Seed};
use test::Bencher;

fn black_box<T>(dummy: T) -> T {
    // we need to "use" the argument in some way LLVM can't
    // introspect.
    unsafe { asm!("" : : "r"(&dummy)) }
    dummy
}

#[bench]
fn bench_perlin2(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| perlin2(black_box(&seed), black_box(&[42.0f32, 37.0])));
}

#[bench]
fn bench_perlin3(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| perlin3(black_box(&seed), black_box(&[42.0f32, 37.0, 26.0])));
}

#[bench]
fn bench_perlin4(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| perlin4(black_box(&seed), black_box(&[42.0f32, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_simplex2(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| simplex2(black_box(&seed), black_box(&[42.0f32, 37.0])));
}

#[bench]
fn bench_simplex3(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| simplex3(black_box(&seed), black_box(&[42.0f32, 37.0, 26.0])));
}

#[bench]
fn bench_simplectic2(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| simplectic2(black_box(&seed), black_box(&[42.0f32, 37.0])));
}

#[bench]
fn bench_simplectic3(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| simplectic3(black_box(&seed), black_box(&[42.0f32, 37.0, 26.0])));
}

#[bench]
fn bench_simplectic4(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| simplectic4(black_box(&seed), black_box(&[42.0f32, 37.0, 26.0, 128.0])));
}
