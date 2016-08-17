// Copyright 2016 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num_traits::Float;
use math::interp;
use NoiseModule;

/// Noise module that outputs the weighted blend of the output values from two
/// source modules given the output value supplied by a control module.
#[derive(Clone, Debug)]
pub struct Blend<Source1, Source2, Control> {
    /// Outputs one of the values to blend.
    pub source1: Source1,

    /// Outputs one of the values to blend.
    pub source2: Source2,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the _Source1_ module. Positive
    /// values weight the blend towards the output value from the _Source2_
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
