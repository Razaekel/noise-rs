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
pub struct Clamp<Source, T> {
    /// Outputs a value.
    pub source: Source,

    /// Lower bound of the clamping range. Default is -1.0.
    pub lower_bound: T,

    /// Upper bound of the clamping range. Default is 1.0.
    pub upper_bound: T,
}

impl<Source, T> Clamp<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> Clamp<Source, T> {
        Clamp {
            source: source,
            lower_bound: -T::one(),
            upper_bound: T::one(),
        }
    }

    pub fn set_lower_bound(self, lower_bound: T) -> Clamp<Source, T> {
        Clamp { lower_bound: lower_bound, ..self }
    }

    pub fn set_upper_bound(self, upper_bound: T) -> Clamp<Source, T> {
        Clamp { upper_bound: upper_bound, ..self }
    }

    pub fn set_bounds(self, lower_bound: T, upper_bound: T) -> Clamp<Source, T> {
        Clamp {
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            ..self
        }
    }
}

impl<Source, T, U> NoiseModule<T> for Clamp<Source, U>
    where Source: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        let value = self.source.get(point);

        match () {
            _ if value < self.lower_bound => self.lower_bound,
            _ if value > self.upper_bound => self.upper_bound,
            _ => value,
        }
    }
}
