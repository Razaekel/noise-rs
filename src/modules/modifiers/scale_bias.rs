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

use modules::NoiseModule;
use num_traits::Float;

/// Noise module that applies a scaling factor and a bias to the output value
/// from the source module.
///
/// The module retrieves the output value from the source module, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
pub struct ScaleBias<Source, T> {
    /// Outputs a value.
    pub source: Source,

    /// Scaling factor to apply to the output value from the source module.
    /// The default value is 1.0.
    pub scale: T,

    /// Bias to apply to the scaled output value from the source module.
    /// The default value is 0.0.
    pub bias: T,
}

impl<Source, T> ScaleBias<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> ScaleBias<Source, T> {
        ScaleBias {
            source: source,
            scale: T::one(),
            bias: T::zero(),
        }
    }

    pub fn set_scale(self, scale: T) -> ScaleBias<Source, T> {
        ScaleBias { scale: scale, ..self }
    }

    pub fn set_bias(self, bias: T) -> ScaleBias<Source, T> {
        ScaleBias { bias: bias, ..self }
    }
}

impl<Source, T, U> NoiseModule<T> for ScaleBias<Source, U>
    where Source: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        (self.source.get(point)).mul_add(self.scale, self.bias)
    }
}
