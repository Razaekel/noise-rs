// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use noise_fns::NoiseFn;
use num_traits::Float;

/// Noise function that inverts the output value from the source function.
pub struct Invert<'a, T: 'a, U: 'a> {
    /// Outputs a value.
    pub source: &'a NoiseFn<T, U>,
}

impl<'a, T, U> Invert<'a, T, U> {
    pub fn new(source: &'a NoiseFn<T, U>) -> Invert<'a, T, U> {
        Invert { source: source }
    }
}

impl<'a, T, U> NoiseFn<T, U> for Invert<'a, T, U>
where
    U: Float,
{
    fn get(&self, point: T) -> U {
        -self.source.get(point)
    }
}
