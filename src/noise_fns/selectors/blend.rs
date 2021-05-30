use crate::{math::interpolate, noise_fns::NoiseFn};

/// Noise function that outputs a weighted blend of the output values from two
/// source functions given the output value supplied by a control function.
///
/// This noise function uses linear interpolation to perform the blending
/// operation.
pub struct Blend<'a, T, U, V, const N: usize>
where
    T: NoiseFn<N>,
    U: NoiseFn<N>,
    V: NoiseFn<N>,
{
    /// Outputs one of the values to blend.
    pub source1: &'a T,

    /// Outputs one of the values to blend.
    pub source2: &'a U,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the `source1` function. Positive
    /// values weight the blend towards the output value from the `source2`
    /// function.
    pub control: &'a V,
}

impl<'a, T, U, V, const N: usize> Blend<'a, T, U, V, N>
where
    T: NoiseFn<N>,
    U: NoiseFn<N>,
    V: NoiseFn<N>,
{
    pub fn new(source1: &'a T, source2: &'a U, control: &'a V) -> Self {
        Blend {
            source1,
            source2,
            control,
        }
    }
}

impl<'a, T, U, V, const N: usize> NoiseFn<N> for Blend<'a, T, U, V, N>
where
    T: NoiseFn<N>,
    U: NoiseFn<N>,
    V: NoiseFn<N>,
{
    fn get(&self, point: [f64; N]) -> f64 {
        let lower = self.source1.get(point);
        let upper = self.source2.get(point);
        let control = self.control.get(point);

        interpolate::linear(lower, upper, control)
    }
}
