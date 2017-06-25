// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use noise_fns::NoiseFn;

/// Noise function that maps the output value from the source function onto an
/// exponential curve.
///
/// Because most noise functions will output values that range from -1.0 to 1.0,
/// this noise function first normalizes the output value (the range becomes 0.0
/// to 1.0), maps that value onto an exponential curve, then rescales that
/// value back to the original range.
pub struct Exponent<'a, T: 'a> {
    /// Outputs a value.
    pub source: &'a NoiseFn<T>,

    /// Exponent to apply to the output value from the source function. Default
    /// is 1.0.
    pub exponent: f64,
}

impl<'a, T> Exponent<'a, T> {
    pub fn new(source: &'a NoiseFn<T>) -> Exponent<'a, T> {
        Exponent {
            source: source,
            exponent: 1.0,
        }
    }

    pub fn set_exponent(self, exponent: f64) -> Exponent<'a, T> {
        Exponent {
            exponent: exponent,
            ..self
        }
    }
}

impl<'a, T> NoiseFn<T> for Exponent<'a, T> {
    fn get(&self, point: T) -> f64 {
        let mut value = self.source.get(point);
        value = (value + 1.0) / 2.0;
        value = value.abs();
        value = value.powf(self.exponent);
        value.mul_add(2.0, -1.0)
    }
}
