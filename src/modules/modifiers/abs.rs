// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use modules::NoiseModule;
use num_traits::Float;

/// Noise module that outputs the absolute value of the output value from the
/// source module.
pub struct Abs<'a, T: 'a, U: 'a> {
    /// Outputs a value.
    pub source: &'a NoiseModule<T, U>,
}

impl<'a, T, U> Abs<'a, T, U> {
    pub fn new(source: &'a NoiseModule<T, U>) -> Abs<'a, T, U> {
        Abs { source: source }
    }
}

impl<'a, T, U> NoiseModule<T, U> for Abs<'a, T, U>
where
    U: Float,
{
    fn get(&self, point: T) -> U {
        (self.source.get(point)).abs()
    }
}
