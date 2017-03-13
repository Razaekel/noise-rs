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
#[derive(Clone, Debug)]
pub struct Blend<Source1, Source2, Control> {
    /// Outputs one of the values to blend.
    pub source1: Source1,

    /// Outputs one of the values to blend.
    pub source2: Source2,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the `source1` module. Positive
    /// values weight the blend towards the output value from the `source2`
    /// module.
    pub control: Control,
}

impl<Source1, Source2, Control> Blend<Source1, Source2, Control> {
    pub fn new(source1: Source1,
               source2: Source2,
               control: Control)
               -> Blend<Source1, Source2, Control> {
        Blend {
            source1: source1,
            source2: source2,
            control: control,
        }
    }
}

impl<Source1, Source2, Control, T, U> NoiseModule<T> for Blend<Source1, Source2, Control>
    where Source1: NoiseModule<T, Output = U>,
          Source2: NoiseModule<T, Output = U>,
          Control: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        let lower = self.source1.get(point);
        let upper = self.source2.get(point);
        let control = self.control.get(point);

        interp::linear(lower, upper, control)
    }
}
