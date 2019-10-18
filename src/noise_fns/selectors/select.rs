use crate::math::interpolate;
use crate::noise_fns::NoiseFn;
use rayon::prelude::*;

/// Noise function that outputs the value selected from one of two source
/// functions chosen by the output value from a control function.
pub struct Select<'a, T: 'a> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T>,

    /// Determines the value to select. If the output value from
    /// the control function is within a range of values know as the _selection
    /// range_, this noise function outputs the value from `source2`.
    /// Otherwise, this noise function outputs the value from `source1`.
    pub control: &'a dyn NoiseFn<T>,

    /// Bounds of the selection range. Default is 0.0 to 1.0.
    pub bounds: (f64, f64),

    /// Edge falloff value. Default is 0.0.
    pub falloff: f64,
}

impl<'a, T> Select<'a, T> {
    pub fn new(
        source1: &'a dyn NoiseFn<T>,
        source2: &'a dyn NoiseFn<T>,
        control: &'a dyn NoiseFn<T>,
    ) -> Self {
        Select {
            source1,
            source2,
            control,
            bounds: (0.0, 1.0),
            falloff: 0.0,
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

impl<'a, T> NoiseFn<T> for Select<'a, T>
where
    T: Copy,
{
    fn generate(&self, points: &[T]) -> Vec<f64> {
        let source1 = self.source1.generate(points);
        let source2 = self.source2.generate(points);
        let control = self.control.generate(points);

        let (lower, upper) = self.bounds;
        let falloff = self.falloff;

        source1
            .par_iter()
            .zip(source2)
            .zip(control)
            .map(|((value1, value2), control_value)| {
                apply_select(*value1, value2, control_value, lower, upper, falloff)
            })
            .collect()
    }
}

fn apply_select(
    value1: f64,
    value2: f64,
    control_value: f64,
    lower: f64,
    upper: f64,
    falloff: f64,
) -> f64 {
    if falloff > 0.0 {
        match () {
            _ if control_value < (lower - falloff) => value1,
            _ if control_value < (lower + falloff) => {
                let lower_curve = lower - falloff;
                let upper_curve = lower + falloff;
                let alpha = interpolate::s_curve3(
                    (control_value - lower_curve) / (upper_curve - lower_curve),
                );

                interpolate::linear(value1, value2, alpha)
            }
            _ if control_value < (upper - falloff) => value2,
            _ if control_value < (upper + falloff) => {
                let lower_curve = upper - falloff;
                let upper_curve = upper + falloff;
                let alpha = interpolate::s_curve3(
                    (control_value - lower_curve) / (upper_curve - lower_curve),
                );

                interpolate::linear(value2, value1, alpha)
            }
            _ => value1,
        }
    } else if control_value < lower || control_value > upper {
        value1
    } else {
        value2
    }
}
