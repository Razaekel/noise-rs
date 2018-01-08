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

use noise::NoiseFn;
use std;
use std::path::Path;

#[inline]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    assert!(max >= min);

    let mut x = input;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

fn write_to_file(filename: &str, pixels: Vec<u8>, width: u32, height: u32) {
    // Create the output directory for the images, if it doesn't already exist
    std::fs::create_dir("example_images/").expect("failed to create example_images directory");

    //concatenate the directory to the filename string
    let directory: String = "example_images/".to_owned();
    let filepath = directory + filename;

    let _ = image::save_buffer(
        &Path::new(&filepath),
        &*pixels,
        width,
        height,
        image::Gray(8),
    );

    println!("\nFinished generating {}", filename);
}

#[allow(dead_code)]
pub fn render_noise_module2<M>(filename: &str, module: &M, width: u32, height: u32, zoom: u32)
where
    M: NoiseFn<[f64; 2]>,
{
    let mut pixels = Vec::with_capacity((width * height) as usize);

    println!("\nGenerating {} points for {}", width * height, filename);
    let mut min_value = std::f64::MAX;
    let mut max_value = std::f64::MIN;

    for y in 0..height {
        for x in 0..width {
            let value = module.get(
                [
                    ((x as f64 - (width as f64 / 2.0)) / zoom as f64),
                    ((y as f64 - (height as f64 / 2.0)) / zoom as f64),
                ],
            );
            pixels.push((clamp(value * 0.5 + 0.5, 0.0, 1.0) * 255.0) as u8);

            print!("\rProcessing {} of {}",
                   (y * width) + height,
                   width * height);

            if value > max_value {
                max_value = value
            };
            if value < min_value {
                min_value = value
            };
        }
    }

    println!("\nMaxValue: {}", max_value);
    println!("\nMinValue: {}", min_value);

    write_to_file(filename, pixels, width, height);
}

#[allow(dead_code)]
pub fn render_noise_module3<M>(filename: &str, module: &M, width: u32, height: u32, zoom: u32)
where
    M: NoiseFn<[f64; 3]>,
{
    let mut pixels = Vec::with_capacity((width * height) as usize);

    println!("\nGenerating {} points for {}", width * height, filename);
    let mut min_value = std::f64::MAX;
    let mut max_value = std::f64::MIN;

    for y in 0..height {
        for x in 0..width {
            let value = module.get(
                [
                    ((x as f64 - (width as f64 / 2.0)) / zoom as f64),
                    ((y as f64 - (height as f64 / 2.0)) / zoom as f64),
                    0.0,
                ],
            );
            pixels.push((clamp(value * 0.5 + 0.5, 0.0, 1.0) * 255.0) as u8);

            print!("\rProcessing {} of {}",
                   (y * width) + height,
                   width * height);

            if value > max_value {
                max_value = value
            };
            if value < min_value {
                min_value = value
            };
        }
    }

    println!("\nMaxValue: {}", max_value);
    println!("\nMinValue: {}", min_value);

    write_to_file(filename, pixels, width, height);
}

#[allow(dead_code)]
pub fn render_noise_module4<M>(filename: &str, module: &M, width: u32, height: u32, zoom: u32)
where
    M: NoiseFn<[f64; 4]>,
{
    let mut pixels = Vec::with_capacity((width * height) as usize);

    println!("\nGenerating {} points for {}", width * height, filename);
    let mut min_value = std::f64::MAX;
    let mut max_value = std::f64::MIN;

    for y in 0..height {
        for x in 0..width {
            let value = module.get(
                [
                    ((x as f64 - (width as f64 / 2.0)) / zoom as f64),
                    ((y as f64 - (height as f64 / 2.0)) / zoom as f64),
                    0.0,
                    0.0,
                ],
            );
            pixels.push((clamp(value * 0.5 + 0.5, 0.0, 1.0) * 255.0) as u8);

            print!("\rProcessing {} of {}",
                   (y * width) + height,
                   width * height);

            if value > max_value {
                max_value = value
            };
            if value < min_value {
                min_value = value
            };
        }
    }

    println!("\nMaxValue: {}", max_value);
    println!("\nMinValue: {}", min_value);

    write_to_file(filename, pixels, width, height);
}
