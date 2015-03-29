// Copyright 2013 The Noise-rs Developers.
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

//! An example of using fractal brownian motion on perlin noise

#![feature(core)]
#![feature(unboxed_closures)]

extern crate noise;

use noise::{Brownian2, Brownian3, Brownian4, perlin2, perlin3, perlin4, Seed, Point2};

mod debug;

fn main() {
    debug::render_png("brownian2.png", &Seed::new(0), 1024, 1024, brownian2_for_image);
    debug::render_png("brownian3.png", &Seed::new(0), 1024, 1024, brownian3_for_image);
    debug::render_png("brownian4.png", &Seed::new(0), 1024, 1024, brownian4_for_image);
    println!("\nGenerated brownian2.png, brownian3.png and brownian4.png");
}

fn brownian2_for_image(seed: &Seed, point: &Point2<f32>) -> f32 {
    Brownian2::new(perlin2, 4).wavelength(16.0)(seed, point)
}

fn brownian3_for_image(seed: &Seed, point: &Point2<f32>) -> f32 {
    Brownian3::new(perlin3, 4).wavelength(16.0)(seed, &[point[0], point[1], 0.0])
}

fn brownian4_for_image(seed: &Seed, point: &Point2<f32>) -> f32 {
    Brownian4::new(perlin4, 4).wavelength(16.0)(seed, &[point[0], point[1], 0.0, 0.0])
}
