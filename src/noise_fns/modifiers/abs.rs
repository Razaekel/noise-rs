use core::marker::PhantomData;

use crate::noise_fns::NoiseFn;

/// Noise function that outputs the absolute value of the output value from the
/// source function.
#[derive(Clone)]
pub struct Abs<T, Source, const DIM: usize>
where
    Source: NoiseFn<T, DIM>,
{
    /// Outputs a value.
    pub source: Source,

    phantom: PhantomData<T>,
}

impl<T, Source, const DIM: usize> Abs<T, Source, DIM>
where
    Source: NoiseFn<T, DIM>,
{
    pub fn new(source: Source) -> Self {
        Self {
            source,
            phantom: PhantomData,
        }
    }
}

impl<T, Source, const DIM: usize> NoiseFn<T, DIM> for Abs<T, Source, DIM>
where
    Source: NoiseFn<T, DIM>,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        (self.source.get(point)).abs()
    }
}
