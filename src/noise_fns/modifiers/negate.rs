use crate::noise_fns::NoiseFn;
use core::marker::PhantomData;
use num_traits::Float;

/// Noise function that negates the output value from the source function.
pub struct Negate<F, Source, const DIM: usize>
where
    Source: NoiseFn<F, DIM>,
{
    /// Outputs a value.
    pub source: Source,

    phantom: PhantomData<F>,
}

impl<F, Source, const DIM: usize> Negate<F, Source, DIM>
where
    Source: NoiseFn<F, DIM>,
{
    pub fn new(source: Source) -> Self {
        Negate {
            source,
            phantom: PhantomData,
        }
    }
}

impl<F, Source, const DIM: usize> NoiseFn<F, DIM> for Negate<F, Source, DIM>
where
    F: Float,
    Source: NoiseFn<F, DIM>,
{
    fn get(&self, point: [F; DIM]) -> F {
        -self.source.get(point)
    }
}
