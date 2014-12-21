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

use noise::{brownian2d, brownian3d, brownian4d, perlin2d_best, perlin3d_best, perlin4d_best, Seed, Point2d};

mod debug {
    pub mod image;
}

fn main() {
    debug::image::render_to_png("brownian2d.png", &Seed::new(0), 256, 256, brownian2d_for_image);
    debug::image::render_to_png("brownian3d.png", &Seed::new(0), 256, 256, brownian3d_for_image);
    debug::image::render_to_png("brownian4d.png", &Seed::new(0), 256, 256, brownian4d_for_image);
    println!("\nGenerated brownian2d.png, brownian3d.png and brownian4d.png");
}

fn brownian2d_for_image(seed: &Seed, point: &Point2d<f32>) -> f32 {
    return brownian2d(seed, point, perlin2d_best, 32.0, 4);
}

fn brownian3d_for_image(seed: &Seed, point: &Point2d<f32>) -> f32 {
    return brownian3d(seed, &[point[0], point[1], 0.0], perlin3d_best, 32.0, 4);
}

fn brownian4d_for_image(seed: &Seed, point: &Point2d<f32>) -> f32 {
    return brownian4d(seed, &[point[0], point[1], 0.0, 0.0], perlin4d_best, 32.0, 4);
}
