// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use modules::NoiseModule;
use num_traits::Float;

/// Noise module that outputs the sum of the two output values from two source
/// modules.
pub struct Add<'a, T: 'a, U: 'a> {
    /// Outputs a value.
    pub source1: &'a NoiseModule<T, U>,

    /// Outputs a value.
    pub source2: &'a NoiseModule<T, U>,
}

impl<'a, T, U> Add<'a, T, U> {
    pub fn new(source1: &'a NoiseModule<T, U>, source2: &'a NoiseModule<T, U>) -> Add<'a, T, U> {
        Add {
            source1: source1,
            source2: source2,
        }
    }
}

impl<'a, T, U> NoiseModule<T, U> for Add<'a, T, U>
    where T: Copy, 
          U: Float,
{
    fn get(&self, point: T) -> U {
        self.source1.get(point) + self.source2.get(point)
    }
}
