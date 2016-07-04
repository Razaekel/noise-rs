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
extern crate num_traits;

use noise;
use noise::Module;
use self::num_traits::{Float, NumCast};
use std::path::Path;

fn cast<T: NumCast, R: NumCast>(val: T) -> R {
    num_traits::cast(val).unwrap()
}

fn clamp<F: Float>(val: F, min: F, max: F) -> F {
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    }
}

pub fn render_png<T, F>(filename: &str, perm_table: &noise::PermutationTable, width: u32, height: u32, func: F) where
    T: Float + NumCast,
    F: noise::GenFn2<T>,
{
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let value: f64 = cast(func(perm_table, &[cast::<_,T>(x) - cast::<_,T>(width/2), cast::<_,T>(y) - cast::<_,T>(height/2)]));
            pixels.push(cast(clamp(value * 0.5 + 0.5, 0.0, 1.0) * 255.0));
        }
    }

    let _ = image::save_buffer(&Path::new(filename),
                               &*pixels,
                               width,
                               height,
                               image::Gray(8));

    println!("\nGenerated {}", filename);
}

pub fn render_png2<M>(filename: &str, module: M, width: u32, height: u32, zoom: u32)
    where M: Module<[f64; 3], Output = f64>
{
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let value: f64 = cast(module.get([((x as f64 - (width as f64 / 2.0)) / zoom as f64),
                                              ((y as f64 - (height as f64 / 2.0)) / zoom as f64),
                                              0.0]));
            pixels.push(cast(clamp(value * 0.5 + 0.5, 0.0, 1.0) * 255.0));
        }
    }

    let _ = image::save_buffer(&Path::new(filename),
                               &*pixels,
                               width,
                               height,
                               image::Gray(8));

    println!("\nGenerated {}", filename);
}
