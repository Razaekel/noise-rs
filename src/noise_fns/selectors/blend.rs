use crate::{math::interpolate, noise_fns::NoiseFn};

/// Noise function that outputs a weighted blend of the output values from two
/// source functions given the output value supplied by a control function.
///
/// This noise function uses linear interpolation to perform the blending
/// operation.
pub struct Blend<'a, T, const N: usize> {
    /// Outputs one of the values to blend.
    pub source1: &'a dyn NoiseFn<T, N>,

    /// Outputs one of the values to blend.
    pub source2: &'a dyn NoiseFn<T, N>,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the `source1` function. Positive
    /// values weight the blend towards the output value from the `source2`
    /// function.
    pub control: &'a dyn NoiseFn<T, N>,
}

impl<'a, T, const N: usize> Blend<'a, T, N> {
    pub fn new(
        source1: &'a dyn NoiseFn<T, N>,
        source2: &'a dyn NoiseFn<T, N>,
        control: &'a dyn NoiseFn<T, N>,
    ) -> Self {
        Blend {
            source1,
            source2,
            control,
        }
    }
}

impl<'a, T, const N: usize> NoiseFn<T, N> for Blend<'a, T, N>
where
    T: Copy,
{
    fn get(&self, point: [T; N]) -> f64 {
        let lower = self.source1.get(point);
        let upper = self.source2.get(point);
        let control = self.control.get(point);

        interpolate::linear(lower, upper, control)
    }
}
