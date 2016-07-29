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
use math;
use NoiseModule;

/// Noise module that maps the output value from the source module onto an
/// exponential curve.
///
/// Because most noise modules will output values that range from -1.0 to 1.0,
/// this noise module first normalizes the output value (the range becomes 0.0
/// to 1.0), maps that value onto an exponential curve, then rescales that
/// value back to the original range.
pub struct Exponent<Source, T> {
    /// Outputs a value.
    source: Source,

    /// Exponent to apply to the output value from the source module. Default
    /// is 1.0.
    exponent: T,
}

impl<Source, T> Exponent<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> Exponent<Source, T> {
        Exponent {
            source: source,
            exponent: T::one(),
        }
    }

    pub fn set_exponent(self, exponent: T) -> Exponent<Source, T> {
        Exponent { exponent: exponent, ..self }
    }
}

impl<Source, T, U> NoiseModule<T> for Exponent<Source, U>
    where Source: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        let mut value = self.source.get(point);
        value = (value + U::one()) / math::cast(2.0);
        value = value.abs();
        value = value.powf(self.exponent);
        value.mul_add(math::cast(2.0), -U::one())
    }
}
