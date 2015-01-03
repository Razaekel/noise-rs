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

extern crate noise;

use std::num::Float;

use noise::source::Source;

#[allow(dead_code)]
#[path = "../../src/math.rs"]
mod math;

const GRADIENT: [&'static str; 6] = [" ", "░", "▒", "▓", "█", "█"];

pub fn render<S: Source>(width: uint, height: uint, source: &S) {
    for y in range(0, height / 2) {
        for x in range(0, width) {
            let val = source.get(x as f32, y as f32 * 2.0, Float::zero());
            let val = math::clamp(val, -1.0, 1.0) * 0.5 + 0.5;
            print!("{}", GRADIENT[(val / 0.2) as uint]);
        }
        println!("");
    }
}
