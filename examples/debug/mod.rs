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

//! Useful things for debugging noise functions.

extern crate image;
extern crate num;

use noise;
use self::num::{Float, NumCast};
use std::path::Path;

fn cast<T: NumCast, R: NumCast>(val: T) -> R {
    num::traits::cast(val).unwrap()
}

fn clamp<F: Float>(val: F, min: F, max: F) -> F {
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    }
}

pub fn render_png<T, F>(filename: &str, seed: &noise::Seed, width: u32, height: u32, func: F) where
    T: Float + NumCast,
    F: noise::GenFn2<T>,
{
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for y in (0..height) {
        for x in (0..width) {
            let value: f32 = cast(func(seed, &[cast::<_,T>(x) - cast::<_,T>(width/2), cast::<_,T>(y) - cast::<_,T>(height/2)]));
            pixels.push(cast(clamp(value * 0.5 + 0.5, 0.0, 1.0) * 255.0));
        }
    }

    let _ = image::save_buffer(&Path::new(filename), &*pixels, width, height, image::Gray(8));
}
