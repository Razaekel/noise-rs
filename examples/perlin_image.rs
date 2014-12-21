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

//! An example of using perlin noise

#![feature(macro_rules)]

extern crate noise;

use noise::{perlin2d_best, perlin3d_best, perlin4d_best, Seed, Point2d};

mod debug {
    pub mod image;
}

fn main() {
    debug::image::render_to_png("perlin2d.png", &Seed::new(0), 256, 256, scaled_perlin2d);
    debug::image::render_to_png("perlin3d.png", &Seed::new(0), 256, 256, scaled_perlin3d);
    debug::image::render_to_png("perlin4d.png", &Seed::new(0), 256, 256, scaled_perlin4d);
    println!("\nGenerated perlin2d.png, perlin3d.png and perlin4d.png");
}

fn scaled_perlin2d(seed: &Seed, point: &Point2d<f32>) -> f32 {
    return perlin2d_best(seed, &[point[0] / 32.0, point[1] / 32.00]);
}

fn scaled_perlin3d(seed: &Seed, point: &Point2d<f32>) -> f32 {
    return perlin3d_best(seed, &[point[0] / 32.0, point[1] / 32.00, 0.0]);
}

fn scaled_perlin4d(seed: &Seed, point: &Point2d<f32>) -> f32 {
    return perlin4d_best(seed, &[point[0] / 32.0, point[1] / 32.00, 0.0, 0.0]);
}
