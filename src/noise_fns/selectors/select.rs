// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math::interp;
use noise_fns::NoiseFn;

/// Noise function that outputs the value selected from one of two source
/// functions chosen by the output value from a control function.
pub struct Select<'a, T: 'a> {
    /// Outputs a value.
    pub source1: &'a NoiseFn<T>,

    /// Outputs a value.
    pub source2: &'a NoiseFn<T>,

    /// Determines the value to select. If the output value from
    /// the control function is within a range of values know as the _selection
    /// range_, this noise function outputs the value from `source2`.
    /// Otherwise, this noise function outputs the value from `source1`.
    pub control: &'a NoiseFn<T>,

    /// Lower bound of the selection range. Default is 0.0.
    pub lower_bound: f64,

    /// Upper bound of the selection range. Default is 1.0.
    pub upper_bound: f64,

    /// Edge-falloff value. Default is 0.0.
    pub edge_falloff: f64,
}

impl<'a, T> Select<'a, T> {
    pub fn new(
        source1: &'a NoiseFn<T>,
        source2: &'a NoiseFn<T>,
        control: &'a NoiseFn<T>,
    ) -> Select<'a, T> {
        Select {
            source1: source1,
            source2: source2,
            control: control,
            lower_bound: 0.0,
            upper_bound: 1.0,
            edge_falloff: 0.0,
        }
    }

    pub fn set_bounds(self, lower: f64, upper: f64) -> Select<'a, T> {
        Select {
            lower_bound: lower,
            upper_bound: upper,
            ..self
        }
    }

    pub fn set_edge_falloff(self, falloff: f64) -> Select<'a, T> {
        Select {
            edge_falloff: falloff,
            ..self
        }
    }
}

impl<'a, T> NoiseFn<T> for Select<'a, T>
where
    T: Copy,
{
    fn get(&self, point: T) -> f64 {
        let control_value = self.control.get(point);

        if self.edge_falloff > 0.0 {
            match () {
                _ if control_value < (self.lower_bound - self.edge_falloff) => {
                    self.source1.get(point)
                },
                _ if control_value < (self.lower_bound + self.edge_falloff) => {
                    let lower_curve = self.lower_bound - self.edge_falloff;
                    let upper_curve = self.lower_bound + self.edge_falloff;
                    let alpha = interp::s_curve3(
                        (control_value - lower_curve) / (upper_curve - lower_curve),
                    );

                    interp::linear(self.source1.get(point), self.source2.get(point), alpha)
                },
                _ if control_value < (self.upper_bound - self.edge_falloff) => {
                    self.source2.get(point)
                },
                _ if control_value < (self.upper_bound + self.edge_falloff) => {
                    let lower_curve = self.upper_bound - self.edge_falloff;
                    let upper_curve = self.upper_bound + self.edge_falloff;
                    let alpha = interp::s_curve3(
                        (control_value - lower_curve) / (upper_curve - lower_curve),
                    );

                    interp::linear(self.source2.get(point), self.source1.get(point), alpha)
                },
                _ => self.source1.get(point),
            }
        } else if control_value < self.lower_bound || control_value > self.upper_bound {
            self.source1.get(point)
        } else {
            self.source2.get(point)
        }
    }
}
