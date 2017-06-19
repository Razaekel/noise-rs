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

/// Noise module that outputs the value selected from one of two source
/// modules chosen by the output value from a control module.
pub struct Select<'a, T: 'a, U: 'a> {
    /// Outputs a value.
    pub source1: &'a NoiseModule<T, U>,

    /// Outputs a value.
    pub source2: &'a NoiseModule<T, U>,

    /// Determines the value to select. If the output value from
    /// the control module is within a range of values know as the _selection
    /// range_, this noise module outputs the value from `source2`.
    /// Otherwise, this noise module outputs the value from `source1`.
    pub control: &'a NoiseModule<T, U>,

    /// Lower bound of the selection range. Default is 0.0.
    pub lower_bound: U,

    /// Upper bound of the selection range. Default is 1.0.
    pub upper_bound: U,

    /// Edge-falloff value. Default is 0.0.
    pub edge_falloff: U,
}

impl<'a, T, U> Select<'a, T, U>
where
    U: Float,
{
    pub fn new(
        source1: &'a NoiseModule<T, U>,
        source2: &'a NoiseModule<T, U>,
        control: &'a NoiseModule<T, U>,
    ) -> Select<'a, T, U> {
        Select {
            source1: source1,
            source2: source2,
            control: control,
            lower_bound: U::zero(),
            upper_bound: U::one(),
            edge_falloff: U::zero(),
        }
    }

    pub fn set_bounds(self, lower: U, upper: U) -> Select<'a, T, U> {
        Select {
            lower_bound: lower,
            upper_bound: upper,
            ..self
        }
    }

    pub fn set_edge_falloff(self, falloff: U) -> Select<'a, T, U> {
        Select {
            edge_falloff: falloff,
            ..self
        }
    }
}

impl<'a, T, U> NoiseModule<T, U> for Select<'a, T, U>
where
    T: Copy,
    U: Float,
{
    fn get(&self, point: T) -> U {
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
