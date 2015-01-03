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

//! An example of using fractal brownian motion on perlin noise

#![feature(macro_rules)]

extern crate noise;

use noise::{brownian2, brownian3, brownian4, perlin2, perlin3, perlin4, Seed, Point2};

mod debug {
    pub mod image;
}

fn main() {
    debug::image::render_to_png("brownian2.png", &Seed::new(0), 256, 256, brownian2_for_image);
    debug::image::render_to_png("brownian3.png", &Seed::new(0), 256, 256, brownian3_for_image);
    debug::image::render_to_png("brownian4.png", &Seed::new(0), 256, 256, brownian4_for_image);
    println!("\nGenerated brownian2.png, brownian3.png and brownian4.png");
}

fn brownian2_for_image(seed: &Seed, point: &Point2<f32>) -> f32 {
    brownian2(seed, point, perlin2, 32.0, 4)
}

fn brownian3_for_image(seed: &Seed, point: &Point2<f32>) -> f32 {
    brownian3(seed, &[point[0], point[1], 0.0], perlin3, 32.0, 4)
}

fn brownian4_for_image(seed: &Seed, point: &Point2<f32>) -> f32 {
    brownian4(seed, &[point[0], point[1], 0.0, 0.0], perlin4, 32.0, 4)
}
