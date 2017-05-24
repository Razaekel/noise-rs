// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use modules::NoiseModule;
use num_traits::Float;

/// Noise module that clamps the output value from the source module to a
/// range of values.
pub struct Clamp<'a, T: 'a, U: 'a> {
    /// Outputs a value.
    pub source: &'a NoiseModule<T, U>,

    /// Lower bound of the clamping range. Default is -1.0.
    pub lower_bound: U,

    /// Upper bound of the clamping range. Default is 1.0.
    pub upper_bound: U,
}

impl<'a, T, U> Clamp<'a, T, U>
    where U: Float,
{
    pub fn new(source: &'a NoiseModule<T, U>) -> Clamp<'a, T, U> {
        Clamp {
            source: source,
            lower_bound: -U::one(),
            upper_bound: U::one(),
        }
    }

    pub fn set_lower_bound(self, lower_bound: U) -> Clamp<'a, T, U> {
        Clamp { lower_bound: lower_bound, ..self }
    }

    pub fn set_upper_bound(self, upper_bound: U) -> Clamp<'a, T, U> {
        Clamp { upper_bound: upper_bound, ..self }
    }

    pub fn set_bounds(self, lower_bound: U, upper_bound: U) -> Clamp<'a, T, U> {
        Clamp {
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            ..self
        }
    }
}

impl<'a, T, U> NoiseModule<T, U> for Clamp<'a, T, U>
    where U: Float,
{
    fn get(&self, point: T) -> U {
        let value = self.source.get(point);

        match () {
            _ if value < self.lower_bound => self.lower_bound,
            _ if value > self.upper_bound => self.upper_bound,
            _ => value,
        }
    }
}
