// Copyright 2015 The noise-rs developers. For a full listing of the authors,
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

//! An example of using cell range noise

extern crate noise;

use noise::{cell2_range_inv, cell3_range_inv, cell4_range_inv, Seed, Point2};

mod debug;

fn main() {
    debug::render_png("cell2_range_inv.png", &Seed::new(0), 256, 256, scaled_cell2_range_inv);
    debug::render_png("cell3_range_inv.png", &Seed::new(0), 256, 256, scaled_cell3_range_inv);
    debug::render_png("cell4_range_inv.png", &Seed::new(0), 256, 256, scaled_cell4_range_inv);
    println!("\nGenerated cell2_range_inv.png, cell3_range_inv.png and cell4_range_inv.png");
}

fn scaled_cell2_range_inv(seed: &Seed, point: &Point2<f32>) -> f32 {
    cell2_range_inv(seed, &[point[0] / 32.0f32, point[1] / 32.00])
}

fn scaled_cell3_range_inv(seed: &Seed, point: &Point2<f32>) -> f32 {
    cell3_range_inv(seed, &[point[0] / 32.0f32, point[1] / 32.00, 0.0])
}

fn scaled_cell4_range_inv(seed: &Seed, point: &Point2<f32>) -> f32 {
    cell4_range_inv(seed, &[point[0] / 32.0f32, point[1] / 32.00, 0.0, 0.0])
}
