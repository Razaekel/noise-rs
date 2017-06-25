// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math::{Point2, Point3, Point4};
use noise_fns::NoiseFn;

/// Default Checkerboard size
pub const DEFAULT_CHECKERBOARD_SIZE: usize = 0;

/// Noise function that outputs a checkerboard pattern.
///
/// This noise function can take one input, size, and outputs 2<sup>size</sup>-sized
/// blocks of alternating values. The values of these blocks alternate between
/// -1.0 and 1.0.
///
/// This noise function is not very useful by itself, but it can be used for
/// debugging purposes.
#[derive(Clone, Copy, Debug)]
pub struct Checkerboard {
    /// Controls the size of the block in 2^(size).
    pub size: usize,

    // Dummy field to prevent struct initialization except through the
    // new() constructor.
    _dummy: (),
}

impl Checkerboard {
    pub fn new() -> Checkerboard {
        Checkerboard {
            size: 1 << DEFAULT_CHECKERBOARD_SIZE,
            _dummy: (),
        }
    }

    pub fn set_size(self, size: usize) -> Checkerboard {
        Checkerboard {
            size: 1 << size,
            ..self
        }
    }
}

impl Default for Checkerboard {
    fn default() -> Self {
        Self::new()
    }
}

// These impl's should be made generic over Point, but there is no higher Point
// type. Keep the code the same anyway.
impl NoiseFn<Point2<f64>> for Checkerboard {
    fn get(&self, point: Point2<f64>) -> f64 {
        calculate_checkerboard(&point, self.size)
    }
}

impl NoiseFn<Point3<f64>> for Checkerboard {
    fn get(&self, point: Point3<f64>) -> f64 {
        calculate_checkerboard(&point, self.size)
    }
}

impl NoiseFn<Point4<f64>> for Checkerboard {
    fn get(&self, point: Point4<f64>) -> f64 {
        calculate_checkerboard(&point, self.size)
    }
}

fn calculate_checkerboard(point: &[f64], size: usize) -> f64 {
    let result = point
        .iter()
        .map(|&a| a.floor() as usize)
        .fold(0, |a, b| (a & size) ^ (b & size));

    if result > 0 { -1.0 } else { 1.0 }
}
