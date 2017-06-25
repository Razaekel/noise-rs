// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math;
use noise_fns::NoiseFn;

/// Noise function that clamps the output value from the source function to a
/// range of values.
pub struct Clamp<'a, T: 'a> {
    /// Outputs a value.
    pub source: &'a NoiseFn<T>,

    /// Lower bound of the clamping range. Default is -1.0.
    pub lower_bound: f64,

    /// Upper bound of the clamping range. Default is 1.0.
    pub upper_bound: f64,
}

impl<'a, T> Clamp<'a, T> {
    pub fn new(source: &'a NoiseFn<T>) -> Clamp<'a, T> {
        Clamp {
            source: source,
            lower_bound: -1.0,
            upper_bound: 1.0,
        }
    }

    pub fn set_lower_bound(self, lower_bound: f64) -> Clamp<'a, T> {
        Clamp {
            lower_bound: lower_bound,
            ..self
        }
    }

    pub fn set_upper_bound(self, upper_bound: f64) -> Clamp<'a, T> {
        Clamp {
            upper_bound: upper_bound,
            ..self
        }
    }

    pub fn set_bounds(self, lower_bound: f64, upper_bound: f64) -> Clamp<'a, T> {
        Clamp {
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            ..self
        }
    }
}

impl<'a, T> NoiseFn<T> for Clamp<'a, T> {
    fn get(&self, point: T) -> f64 {
        let value = self.source.get(point);

        math::clamp(value, self.lower_bound, self.upper_bound)
    }
}
