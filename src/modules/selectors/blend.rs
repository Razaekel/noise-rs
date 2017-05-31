// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math::interp;
use modules::NoiseModule;
use num_traits::Float;

/// Noise module that outputs a weighted blend of the output values from two
/// source modules given the output value supplied by a control module.
///
/// This noise module uses linear interpolation to perform the blending
/// operation.
pub struct Blend<'a, T: 'a, U: 'a> {
    /// Outputs one of the values to blend.
    pub source1: &'a NoiseModule<T, U>,

    /// Outputs one of the values to blend.
    pub source2: &'a NoiseModule<T, U>,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the `source1` module. Positive
    /// values weight the blend towards the output value from the `source2`
    /// module.
    pub control: &'a NoiseModule<T, U>,
}

impl<'a, T, U> Blend<'a, T, U> {
    pub fn new(source1: &'a NoiseModule<T, U>,
               source2: &'a NoiseModule<T, U>,
               control: &'a NoiseModule<T, U>)
               -> Blend<'a, T, U> {
        Blend {
            source1: source1,
            source2: source2,
            control: control,
        }
    }
}

impl<'a, T, U> NoiseModule<T, U> for Blend<'a, T, U>
    where T: Copy,
          U: Float,
{
    fn get(&self, point: T) -> U {
        let lower = self.source1.get(point);
        let upper = self.source2.get(point);
        let control = self.control.get(point);

        interp::linear(lower, upper, control)
    }
}
