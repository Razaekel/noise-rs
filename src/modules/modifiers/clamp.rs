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

/// Noise module that clamps the output value from the source module to a
/// range of values.
pub struct Clamp<Source, T> {
    /// Outputs a value.
    pub source: Source,

    /// Lower bound of the clamping range. Default is -1.0.
    pub lower_bound: T,

    /// Upper bound of the clamping range. Default is 1.0.
    pub upper_bound: T,
}

impl<Source, T> Clamp<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> Clamp<Source, T> {
        Clamp {
            source: source,
            lower_bound: -T::one(),
            upper_bound: T::one(),
        }
    }

    pub fn set_lower_bound(self, lower_bound: T) -> Clamp<Source, T> {
        Clamp { lower_bound: lower_bound, ..self }
    }

    pub fn set_upper_bound(self, upper_bound: T) -> Clamp<Source, T> {
        Clamp { upper_bound: upper_bound, ..self }
    }

    pub fn set_bounds(self, lower_bound: T, upper_bound: T) -> Clamp<Source, T> {
        Clamp {
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            ..self
        }
    }
}

impl<Source, T, U> NoiseModule<T> for Clamp<Source, U>
    where Source: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        let value = self.source.get(point);

        match () {
            _ if value < self.lower_bound => self.lower_bound,
            _ if value > self.upper_bound => self.upper_bound,
            _ => value,
        }
    }
}
