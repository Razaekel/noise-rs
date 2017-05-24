// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use modules::NoiseModule;
use num_traits::Float;

/// Noise module that inverts the output value from the source module.
pub struct Invert<Source> {
    /// Outputs a value.
    pub source: Source,
}

impl<Source> Invert<Source> {
    pub fn new(source: Source) -> Invert<Source> {
        Invert { source: source }
    }
}

impl<Source, T, U> NoiseModule<T, U> for Invert<Source>
    where Source: NoiseModule<T, U>,
          T: Copy,
          U: Float,
{
    fn get(&self, point: T) -> U {
        -self.source.get(point)
    }
}
