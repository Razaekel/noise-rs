use crate::{math::interpolate, noise_fns::NoiseFn};
use core::marker::PhantomData;

/// Noise function that outputs a weighted blend of the output values from two
/// source functions given the output value supplied by a control function.
///
/// This noise function uses linear interpolation to perform the blending
/// operation.
pub struct Blend<T, Source1, Source2, Control, const DIM: usize>
where
    Source1: NoiseFn<T, DIM>,
    Source2: NoiseFn<T, DIM>,
    Control: NoiseFn<T, DIM>,
{
    /// Outputs one of the values to blend.
    pub source1: Source1,

    /// Outputs one of the values to blend.
    pub source2: Source2,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the `source1` function. Positive
    /// values weight the blend towards the output value from the `source2`
    /// function.
    pub control: Control,

    phantom: PhantomData<T>,
}

impl<T, Source1, Source2, Control, const DIM: usize> Blend<T, Source1, Source2, Control, DIM>
where
    Source1: NoiseFn<T, DIM>,
    Source2: NoiseFn<T, DIM>,
    Control: NoiseFn<T, DIM>,
{
    pub fn new(source1: Source1, source2: Source2, control: Control) -> Self {
        Blend {
            source1,
            source2,
            control,
            phantom: PhantomData,
        }
    }
}

impl<T, Source1, Source2, Control, const DIM: usize> NoiseFn<T, DIM>
    for Blend<T, Source1, Source2, Control, DIM>
where
    T: Copy,
    Source1: NoiseFn<T, DIM>,
    Source2: NoiseFn<T, DIM>,
    Control: NoiseFn<T, DIM>,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        let lower = self.source1.get(point);
        let upper = self.source2.get(point);
        let control = self.control.get(point);

        interpolate::linear(lower, upper, control)
    }
}
