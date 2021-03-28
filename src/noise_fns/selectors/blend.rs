use crate::{math::interpolate, noise_fns::NoiseFn};

/// Noise function that outputs a weighted blend of the output values from two
/// source functions given the output value supplied by a control function.
///
/// This noise function uses linear interpolation to perform the blending
/// operation.
pub struct Blend<'a, T, const DIM: usize> {
    /// Outputs one of the values to blend.
    pub source1: &'a dyn NoiseFn<T, DIM>,

    /// Outputs one of the values to blend.
    pub source2: &'a dyn NoiseFn<T, DIM>,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the `source1` function. Positive
    /// values weight the blend towards the output value from the `source2`
    /// function.
    pub control: &'a dyn NoiseFn<T, DIM>,
}

impl<'a, T, const DIM: usize> Blend<'a, T, DIM> {
    pub fn new(
        source1: &'a dyn NoiseFn<T, DIM>,
        source2: &'a dyn NoiseFn<T, DIM>,
        control: &'a dyn NoiseFn<T, DIM>,
    ) -> Self {
        Blend {
            source1,
            source2,
            control,
        }
    }
}

impl<'a, T, const DIM: usize> NoiseFn<T, DIM> for Blend<'a, T, DIM>
where
    T: Copy,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        let lower = self.source1.get(point);
        let upper = self.source2.get(point);
        let control = self.control.get(point);

        interpolate::linear(lower, upper, control)
    }
}
