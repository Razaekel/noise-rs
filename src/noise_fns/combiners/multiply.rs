use core::marker::PhantomData;

use crate::noise_fns::NoiseFn;

/// Noise function that outputs the product of the two output values from two source
/// functions.
#[derive(Clone)]
pub struct Multiply<T, Source1, Source2, const DIM: usize>
where
    Source1: NoiseFn<T, DIM>,
    Source2: NoiseFn<T, DIM>,
{
    /// Outputs a value.
    pub source1: Source1,

    /// Outputs a value.
    pub source2: Source2,

    phantom: PhantomData<T>,
}

impl<T, Source1, Source2, const DIM: usize> Multiply<T, Source1, Source2, DIM>
where
    Source1: NoiseFn<T, DIM>,
    Source2: NoiseFn<T, DIM>,
{
    pub fn new(source1: Source1, source2: Source2) -> Self {
        Self {
            source1,
            source2,
            phantom: PhantomData,
        }
    }
}

impl<T, Source1, Source2, const DIM: usize> NoiseFn<T, DIM> for Multiply<T, Source1, Source2, DIM>
where
    T: Copy,
    Source1: NoiseFn<T, DIM>,
    Source2: NoiseFn<T, DIM>,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        self.source1.get(point) * self.source2.get(point)
    }
}
