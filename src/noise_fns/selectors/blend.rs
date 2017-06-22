// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math::interp;
use noise_fns::NoiseFn;
use num_traits::Float;

/// Noise function that outputs a weighted blend of the output values from two
/// source functions given the output value supplied by a control function.
///
/// This noise function uses linear interpolation to perform the blending
/// operation.
pub struct Blend<'a, T: 'a, U: 'a> {
    /// Outputs one of the values to blend.
    pub source1: &'a NoiseFn<T, U>,

    /// Outputs one of the values to blend.
    pub source2: &'a NoiseFn<T, U>,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the `source1` function. Positive
    /// values weight the blend towards the output value from the `source2`
    /// function.
    pub control: &'a NoiseFn<T, U>,
}

impl<'a, T, U> Blend<'a, T, U> {
    pub fn new(
        source1: &'a NoiseFn<T, U>,
        source2: &'a NoiseFn<T, U>,
        control: &'a NoiseFn<T, U>,
    ) -> Blend<'a, T, U> {
        Blend {
            source1: source1,
            source2: source2,
            control: control,
        }
    }
}

impl<'a, T, U> NoiseFn<T, U> for Blend<'a, T, U>
where
    T: Copy,
    U: Float,
{
    fn get(&self, point: T) -> U {
        let lower = self.source1.get(point);
        let upper = self.source2.get(point);
        let control = self.control.get(point);

        interp::linear(lower, upper, control)
    }
}
