use crate::noise_fns::NoiseFn;
use core::marker::PhantomData;
use num_traits::Float;

/// Noise function that outputs the absolute value of the output value from the
/// source function.
pub struct Abs<F, Source, const DIM: usize>
where
    Source: NoiseFn<F, DIM>,
{
    /// Outputs a value.
    pub source: Source,

    phantom: PhantomData<F>,
}

impl<F, Source, const DIM: usize> Abs<F, Source, DIM>
where
    Source: NoiseFn<F, DIM>,
{
    pub fn new(source: Source) -> Self {
        Self {
            source,
            phantom: PhantomData,
        }
    }
}

impl<F, Source, const DIM: usize> NoiseFn<F, DIM> for Abs<F, Source, DIM>
where
    F: Float,
    Source: NoiseFn<F, DIM>,
{
    fn get(&self, point: [F; DIM]) -> F {
        (self.source.get(point)).abs()
    }
}
