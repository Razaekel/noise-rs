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

extern crate noise;

use noise::{perlin2, perlin3, perlin4, PermutationTable, Point2};

mod debug;

fn main() {
    debug::render_png("perlin2.png", &PermutationTable::new(0), 1024, 1024, scaled_perlin2);
    debug::render_png("perlin3.png", &PermutationTable::new(0), 1024, 1024, scaled_perlin3);
    debug::render_png("perlin4.png", &PermutationTable::new(0), 1024, 1024, scaled_perlin4);
    println!("\nGenerated perlin2.png, perlin3.png and perlin4.png");
}

fn scaled_perlin2(perm_table: &PermutationTable, point: &Point2<f64>) -> f64 {
    perlin2(perm_table, &[point[0] / 16.0, point[1] / 16.0])
}

fn scaled_perlin3(perm_table: &PermutationTable, point: &Point2<f64>) -> f64 {
    perlin3(perm_table, &[point[0] / 16.0, point[1] / 16.0, point[0] / 32.0])
}

fn scaled_perlin4(perm_table: &PermutationTable, point: &Point2<f64>) -> f64 {
    perlin4(perm_table, &[point[0] / 16.0, point[1] / 16.0, point[0] / 32.0, point[1] / 32.0])
}
