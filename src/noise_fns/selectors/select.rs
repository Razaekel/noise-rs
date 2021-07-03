use crate::{
    math::{interpolate, s_curve::cubic::Cubic},
    noise_fns::NoiseFn,
};
use core::marker::PhantomData;

/// Noise function that outputs the value selected from one of two source
/// functions chosen by the output value from a control function.
pub struct Select<T, Source1, Source2, Control, const DIM: usize>
where
    Source1: NoiseFn<T, DIM>,
    Source2: NoiseFn<T, DIM>,
    Control: NoiseFn<T, DIM>,
{
    /// Outputs a value.
    pub source1: Source1,

    /// Outputs a value.
    pub source2: Source2,

    /// Determines the value to select. If the output value from
    /// the control function is within a range of values know as the _selection
    /// range_, this noise function outputs the value from `source2`.
    /// Otherwise, this noise function outputs the value from `source1`.
    pub control: Control,

    /// Bounds of the selection range. Default is 0.0 to 1.0.
    pub bounds: (f64, f64),

    /// Edge falloff value. Default is 0.0.
    pub falloff: f64,

    phantom: PhantomData<T>,
}

impl<T, Source1, Source2, Control, const DIM: usize> Select<T, Source1, Source2, Control, DIM>
where
    Source1: NoiseFn<T, DIM>,
    Source2: NoiseFn<T, DIM>,
    Control: NoiseFn<T, DIM>,
{
    pub fn new(source1: Source1, source2: Source2, control: Control) -> Self {
        Select {
            source1,
            source2,
            control,
            bounds: (0.0, 1.0),
            falloff: 0.0,
            phantom: PhantomData,
        }
    }

    pub fn set_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        Select {
            bounds: (lower_bound, upper_bound),
            ..self
        }
    }

    pub fn set_falloff(self, falloff: f64) -> Self {
        Select { falloff, ..self }
    }
}

impl<T, Source1, Source2, Control, const DIM: usize> NoiseFn<T, DIM>
    for Select<T, Source1, Source2, Control, DIM>
where
    T: Copy,
    Source1: NoiseFn<T, DIM>,
    Source2: NoiseFn<T, DIM>,
    Control: NoiseFn<T, DIM>,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        let control_value = self.control.get(point);
        let (lower, upper) = self.bounds;

        if self.falloff > 0.0 {
            match () {
                _ if control_value < (lower - self.falloff) => self.source1.get(point),
                _ if control_value < (lower + self.falloff) => {
                    let lower_curve = lower - self.falloff;
                    let upper_curve = lower + self.falloff;
                    let alpha =
                        ((control_value - lower_curve) / (upper_curve - lower_curve)).map_cubic();

                    interpolate::linear(self.source1.get(point), self.source2.get(point), alpha)
                }
                _ if control_value < (upper - self.falloff) => self.source2.get(point),
                _ if control_value < (upper + self.falloff) => {
                    let lower_curve = upper - self.falloff;
                    let upper_curve = upper + self.falloff;
                    let alpha =
                        ((control_value - lower_curve) / (upper_curve - lower_curve)).map_cubic();

                    interpolate::linear(self.source2.get(point), self.source1.get(point), alpha)
                }
                _ => self.source1.get(point),
            }
        } else if control_value < lower || control_value > upper {
            self.source1.get(point)
        } else {
            self.source2.get(point)
        }
    }
}
