use crate::noise_fns::NoiseFn;
use core::marker::PhantomData;
use num_traits::Float;

/// Noise function that raises the output value from the first source function
/// to the power of the output value of the second source function.
pub struct Power<F, Source1, Source2, const DIM: usize>
where
    Source1: NoiseFn<F, DIM>,
    Source2: NoiseFn<F, DIM>,
{
    /// Outputs a value.
    pub source1: Source1,

    /// Outputs a value.
    pub source2: Source2,

    phantom: PhantomData<F>,
}

impl<F, Source1, Source2, const DIM: usize> Power<F, Source1, Source2, DIM>
where
    Source1: NoiseFn<F, DIM>,
    Source2: NoiseFn<F, DIM>,
{
    pub fn new(source1: Source1, source2: Source2) -> Self {
        Self {
            source1,
            source2,
            phantom: PhantomData,
        }
    }
}

impl<F, Source1, Source2, const DIM: usize> NoiseFn<F, DIM> for Power<F, Source1, Source2, DIM>
where
    F: Float,
    Source1: NoiseFn<F, DIM>,
    Source2: NoiseFn<F, DIM>,
{
    fn get(&self, point: [F; DIM]) -> F {
        (self.source1.get(point)).powf(self.source2.get(point))
    }
}
