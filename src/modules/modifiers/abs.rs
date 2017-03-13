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
pub struct Abs<Source> {
    /// Outputs a value.
    pub source: Source,
}

impl<Source> Abs<Source> {
    pub fn new(source: Source) -> Abs<Source> {
        Abs { source: source }
    }
}

impl<Source, T, U> NoiseModule<T> for Abs<Source>
    where Source: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        (self.source.get(point)).abs()
    }
}
