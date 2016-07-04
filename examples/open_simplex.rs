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

//! An example of using simplex noise

extern crate noise;

use noise::{open_simplex2, open_simplex3, open_simplex4, PermutationTable, Point2};

mod debug;

fn main() {
    debug::render_png("open_simplex2.png", &PermutationTable::new(0), 1024, 1024, scaled_open_simplex2);
    debug::render_png("open_simplex3.png", &PermutationTable::new(0), 1024, 1024, scaled_open_simplex3);
    debug::render_png("open_simplex4.png", &PermutationTable::new(0), 1024, 1024, scaled_open_simplex4);
    println!("\nGenerated open_simplex2.png, open_simplex3.png and open_simplex4.png");
}

fn scaled_open_simplex2(perm_table: &PermutationTable, point: &Point2<f64>) -> f64 {
    open_simplex2(perm_table, &[point[0] / 16.0, point[1] / 16.0])
}

fn scaled_open_simplex3(perm_table: &PermutationTable, point: &Point2<f64>) -> f64 {
    open_simplex3(perm_table, &[point[0] / 16.0, point[1] / 16.0, point[0] / 32.0])
}

fn scaled_open_simplex4(perm_table: &PermutationTable, point: &Point2<f64>) -> f64 {
    open_simplex4(perm_table, &[point[0] / 16.0, point[1] / 16.0, point[0] / 32.0, point[1] / 32.0])
}
