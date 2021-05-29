use crate::noise_fns::NoiseFn;

/// Noise function that outputs the absolute value of the output value from the
/// source function.
pub struct Abs<'a, T, const DIM: usize>
where
    T: NoiseFn<DIM>,
{
    /// Outputs a value.
    pub source: &'a T,
}

impl<'a, T, const DIM: usize> Abs<'a, T, DIM>
where
    T: NoiseFn<DIM>,
{
    pub fn new(source: &'a T) -> Self {
        Self { source }
    }
}

impl<'a, T, const DIM: usize> NoiseFn<DIM> for Abs<'a, T, DIM>
where
    T: NoiseFn<DIM>,
{
    fn get(&self, point: [f64; DIM]) -> f64 {
        (self.source.get(point)).abs()
    }
}
