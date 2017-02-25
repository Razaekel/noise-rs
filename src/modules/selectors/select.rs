// Copyright 2013 The Noise-rs Developers.
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
use modules::NoiseModule;

/// Noise module that outputs the value selected from one of two source
/// modules chosen by the output value from a control module.
#[derive(Clone, Copy, Debug)]
pub struct Select<Source1, Source2, Control, T> {
    /// Outputs a value.
    pub source1: Source1,

    /// Outputs a value.
    pub source2: Source2,

    /// Determines the value to select. If the output value from
    /// the control module is within a range of values know as the _selection
    /// range_, this noise module outputs the value from `source2`.
    /// Otherwise, this noise module outputs the value from `source1`.
    pub control: Control,

    /// Lower bound of the selection range. Default is 0.0.
    pub lower_bound: T,

    /// Upper bound of the selection range. Default is 1.0.
    pub upper_bound: T,

    /// Edge-falloff value. Default is 0.0.
    pub edge_falloff: T,
}

impl<Source1, Source2, Control, T> Select<Source1, Source2, Control, T>
    where T: Float,
{
    pub fn new(source1: Source1,
               source2: Source2,
               control: Control)
               -> Select<Source1, Source2, Control, T> {
        Select {
            source1: source1,
            source2: source2,
            control: control,
            lower_bound: T::zero(),
            upper_bound: T::one(),
            edge_falloff: T::zero(),
        }
    }

    pub fn set_bounds(self, lower: T, upper: T) -> Select<Source1, Source2, Control, T> {
        Select {
            lower_bound: lower,
            upper_bound: upper,
            ..self
        }
    }

    pub fn set_edge_falloff(self, falloff: T) -> Select<Source1, Source2, Control, T> {
        Select { edge_falloff: falloff, ..self }
    }
}

impl<Source1, Source2, Control, T, U> NoiseModule<T> for Select<Source1, Source2, Control, U>
    where Source1: NoiseModule<T, Output = U>,
          Source2: NoiseModule<T, Output = U>,
          Control: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        let control_value = self.control.get(point);

        if self.edge_falloff > U::zero() {
            match () {
                _ if control_value < (self.lower_bound - self.edge_falloff) => {
                    self.source1.get(point)
                },
                _ if control_value < (self.lower_bound + self.edge_falloff) => {
                    let lower_curve: U = self.lower_bound - self.edge_falloff;
                    let upper_curve: U = self.lower_bound + self.edge_falloff;
                    let alpha = interp::s_curve3((control_value - lower_curve) /
                                                 (upper_curve - lower_curve));

                    interp::linear(self.source1.get(point), self.source2.get(point), alpha)
                },
                _ if control_value < (self.upper_bound - self.edge_falloff) => {
                    self.source2.get(point)
                },
                _ if control_value < (self.upper_bound + self.edge_falloff) => {
                    let lower_curve: U = self.upper_bound - self.edge_falloff;
                    let upper_curve: U = self.upper_bound + self.edge_falloff;
                    let alpha = interp::s_curve3((control_value - lower_curve) /
                                                 (upper_curve - lower_curve));

                    interp::linear(self.source2.get(point), self.source1.get(point), alpha)
                },
                _ => self.source1.get(point),
            }
        } else {
            if control_value < self.lower_bound || control_value > self.upper_bound {
                self.source1.get(point)
            } else {
                self.source2.get(point)
            }
        }
    }
}
