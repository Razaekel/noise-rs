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

//! An example of using cell range noise

extern crate noise;

use noise::{cell2_range_inv, cell3_range_inv, cell4_range_inv, Seed, Point2};

mod debug;

fn main() {
    debug::render_png("cell2_range_inv.png", &Seed::new(0), 1024, 1024, scaled_cell2_range_inv);
    debug::render_png("cell3_range_inv.png", &Seed::new(0), 1024, 1024, scaled_cell3_range_inv);
    debug::render_png("cell4_range_inv.png", &Seed::new(0), 1024, 1024, scaled_cell4_range_inv);
    println!("\nGenerated cell2_range_inv.png, cell3_range_inv.png and cell4_range_inv.png");
}

fn scaled_cell2_range_inv(seed: &Seed, point: &Point2<f64>) -> f64 {
    cell2_range_inv(seed, &[point[0] / 16.0, point[1] / 16.0]) * 2.0 - 1.0
}

fn scaled_cell3_range_inv(seed: &Seed, point: &Point2<f64>) -> f64 {
    cell3_range_inv(seed, &[point[0] / 16.0, point[1] / 16.0, point[0] / 32.0]) * 2.0 - 1.0
}

fn scaled_cell4_range_inv(seed: &Seed, point: &Point2<f64>) -> f64 {
    cell4_range_inv(seed, &[point[0] / 16.0, point[1] / 16.0, point[0] / 32.0, point[1] / 32.0]) * 2.0 - 1.0
}
