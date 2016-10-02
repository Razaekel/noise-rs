// Copyright 2013 The Noise-rs Developers.
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

use num_traits::Float;
use math;
use math::{Point2, Point3, Point4};
use modules::NoiseModule;

/// Default Checkerboard size
pub const DEFAULT_CHECKERBOARD_SIZE: usize = 0;

/// Noise module that outputs a checkerboard pattern.
///
/// This noise module can take one input, size, and outputs 2<sup>size</sup>-sized blocks
/// of alternating values. The values of these blocks alternate between -1.0
/// and 1.0.
///
/// This noise module is not very useful by itself, but it can be used for
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
        Checkerboard { size: 1 << size, ..self }
    }
}

fn fast_floor<T: Float>(x: T) -> usize {
    if x > T::zero() {
        math::cast(x)
    } else {
        math::cast(x - T::one())
    }
}

// These impl's should be made generic over Point, but there is no higher Point type.
// Keep the code the same anyway.
impl<T: Float> NoiseModule<Point2<T>> for Checkerboard {
    type Output = T;

    fn get(&self, point: Point2<T>) -> Self::Output {
        calculate_checkerboard(&point, self.size)
    }
}

impl<T: Float> NoiseModule<Point3<T>> for Checkerboard {
    type Output = T;

    fn get(&self, point: Point3<T>) -> Self::Output {
        calculate_checkerboard(&point, self.size)
    }
}

impl<T: Float> NoiseModule<Point4<T>> for Checkerboard {
    type Output = T;

    fn get(&self, point: Point4<T>) -> Self::Output {
        calculate_checkerboard(&point, self.size)
    }
}

fn calculate_checkerboard<T: Float>(point: &[T], size: usize) -> T {
    let result = point.iter()
        .map(|&a| fast_floor(a))
        .fold(0, |a, b| (a & size) ^ (b & size));

    if result > 0 {
        -T::one()
    } else {
        T::one()
    }
}
