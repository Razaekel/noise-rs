// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math;
use modules::NoiseModule;
use num_traits::Float;

/// Noise module that maps the output value from the source module onto an
/// exponential curve.
///
/// Because most noise modules will output values that range from -1.0 to 1.0,
/// this noise module first normalizes the output value (the range becomes 0.0
/// to 1.0), maps that value onto an exponential curve, then rescales that
/// value back to the original range.
pub struct Exponent<'a, T: 'a, U: 'a> {
    /// Outputs a value.
    pub source: &'a NoiseModule<T, U>,

    /// Exponent to apply to the output value from the source module. Default
    /// is 1.0.
    pub exponent: U,
}

impl<'a, T, U> Exponent<'a, T, U>
where
    U: Float,
{
    pub fn new(source: &'a NoiseModule<T, U>) -> Exponent<'a, T, U> {
        Exponent {
            source: source,
            exponent: U::one(),
        }
    }

    pub fn set_exponent(self, exponent: U) -> Exponent<'a, T, U> {
        Exponent {
            exponent: exponent,
            ..self
        }
    }
}

impl<'a, T, U> NoiseModule<T, U> for Exponent<'a, T, U>
where
    U: Float,
{
    fn get(&self, point: T) -> U {
        let mut value = self.source.get(point);
        value = (value + U::one()) / math::cast(2.0);
        value = value.abs();
        value = value.powf(self.exponent);
        value.mul_add(math::cast(2.0), -U::one())
    }
}
