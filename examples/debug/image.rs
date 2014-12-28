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

extern crate image;
extern crate noise;

use std::num::{cast, Float, NumCast};

#[allow(dead_code)]
#[path = "../../src/math.rs"]
mod math;

pub fn render_to_png<T, F>(filename: &str, seed: &noise::Seed, width: u32, height: u32, func: F) where
    T: Float + NumCast,
    F: Fn(&noise::Seed, &noise::Point2<T>) -> T,
{
    let mut pixels = Vec::with_capacity((width * height) as uint);

    for y in range(0, height) {
        for x in range(0, width) {
            let value: f32 = math::cast(func(seed, &[c(x), c(y)]));
            pixels.push(c(math::clamp(value * 0.5 + 0.5, 0.0, 1.0) * 255.0));
        }
    }

    let _ = image::save_buffer(&Path::new(filename), pixels.as_slice(), width, height, image::ColorType::Grey(8));
}

fn c<T: NumCast, R: NumCast>(val: T) -> R {
    cast(val).unwrap()
}
