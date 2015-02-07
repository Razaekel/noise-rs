// Copyright 2015 The Noise-rs Developers.
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

//! An example of using perlin noise

#![feature(core)]
#![feature(path)]

extern crate noise;

use noise::{perlin2, perlin3, perlin4, Seed, Point2};

mod debug;

fn main() {
    debug::render_png("perlin2.png", &Seed::new(0), 1024, 1024, scaled_perlin2);
    debug::render_png("perlin3.png", &Seed::new(0), 1024, 1024, scaled_perlin3);
    debug::render_png("perlin4.png", &Seed::new(0), 1024, 1024, scaled_perlin4);
    println!("\nGenerated perlin2.png, perlin3.png and perlin4.png");
}

fn scaled_perlin2(seed: &Seed, point: &Point2<f32>) -> f32 {
    perlin2(seed, &[point[0] / 16.0, point[1] / 16.0])
}

fn scaled_perlin3(seed: &Seed, point: &Point2<f32>) -> f32 {
    perlin3(seed, &[point[0] / 16.0, point[1] / 16.0, 0.0])
}

fn scaled_perlin4(seed: &Seed, point: &Point2<f32>) -> f32 {
    perlin4(seed, &[point[0] / 16.0, point[1] / 16.0, 0.0, 0.0])
}
