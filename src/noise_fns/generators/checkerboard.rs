// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math;
use math::{Point2, Point3, Point4};
use noise_fns::NoiseFn;
use num_traits::Float;

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
#[derive(Clone, Copy, Debug, Default)]
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

fn fast_floor<T: Float>(x: T) -> usize {
    if x > T::zero() {
        math::cast(x)
    } else {
        math::cast(x - T::one())
    }
}

// These impl's should be made generic over Point, but there is no higher Point
// type. Keep the code the same anyway.
impl<T: Float> NoiseFn<Point2<T>, T> for Checkerboard {
    fn get(&self, point: Point2<T>) -> T {
        calculate_checkerboard(&point, self.size)
    }
}

impl<T: Float> NoiseFn<Point3<T>, T> for Checkerboard {
    fn get(&self, point: Point3<T>) -> T {
        calculate_checkerboard(&point, self.size)
    }
}

impl<T: Float> NoiseFn<Point4<T>, T> for Checkerboard {
    fn get(&self, point: Point4<T>) -> T {
        calculate_checkerboard(&point, self.size)
    }
}

fn calculate_checkerboard<T: Float>(point: &[T], size: usize) -> T {
    let result = point
        .iter()
        .map(|&a| fast_floor(a))
        .fold(0, |a, b| (a & size) ^ (b & size));

    if result > 0 { -T::one() } else { T::one() }
}
