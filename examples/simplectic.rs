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

extern crate noise;

use noise::{simplectic2, simplectic3, simplectic4, Seed, Point2};

mod debug {
    pub mod image;
}

fn main() {
    debug::image::render_to_png("simplectic2.png", &Seed::new(0), 1024, 1024, scaled_simplectic2);
    debug::image::render_to_png("simplectic3.png", &Seed::new(0), 1024, 1024, scaled_simplectic3);
    debug::image::render_to_png("simplectic4.png", &Seed::new(0), 1024, 1024, scaled_simplectic4);
    println!("\nGenerated simplectic2.png, simplectic3.png and simplectic4.png");
}

fn scaled_simplectic2(seed: &Seed, point: &Point2<f32>) -> f32 {
    simplectic2(seed, &[point[0] / 64.0, point[1] / 64.0])
}

fn scaled_simplectic3(seed: &Seed, point: &Point2<f32>) -> f32 {
    simplectic3(seed, &[point[0] / 64.0, point[1] / 64.0, 0.0])
}

fn scaled_simplectic4(seed: &Seed, point: &Point2<f32>) -> f32 {
    simplectic4(seed, &[point[0] / 64.0, point[1] / 64.0, 0.0, 0.0])
}
