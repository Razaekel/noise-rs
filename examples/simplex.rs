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

//! An example of using simplex noise

#![feature(macro_rules)]

extern crate noise;

use noise::{simplex2, Seed, Point2};

mod debug {
    pub mod image;
}

fn main() {
    debug::image::render_to_png("simplex2.png", &Seed::new(0), 256, 256, scaled_simplex2);
    // debug::image::render_to_png("simplex3.png", &Seed::new(0), 256, 256, scaled_simplex3);
    // debug::image::render_to_png("simplex4.png", &Seed::new(0), 256, 256, scaled_simplex4);
    println!("\nGenerated simplex2.png, simplex3.png and simplex4.png");
}

fn scaled_simplex2(seed: &Seed, point: &Point2<f32>) -> f32 {
    return simplex2(seed, &[point[0] / 32.0, point[1] / 32.00]);
}

// fn scaled_simplex3(seed: &Seed, point: &Point2<f32>) -> f32 {
//     return simplex3_best(seed, &[point[0] / 32.0, point[1] / 32.00, 0.0]);
// }

// fn scaled_simplex4(seed: &Seed, point: &Point2<f32>) -> f32 {
//     return simplex4_best(seed, &[point[0] / 32.0, point[1] / 32.00, 0.0, 0.0]);
// }
