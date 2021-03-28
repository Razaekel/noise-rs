use crate::noise_fns::NoiseFn;

/// Noise function that outputs the absolute value of the output value from the
/// source function.
pub struct Abs<'a, T, const DIM: usize> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T, DIM>,
}

impl<'a, T, const DIM: usize> Abs<'a, T, DIM> {
    pub fn new(source: &'a dyn NoiseFn<T, DIM>) -> Self {
        Self { source }
    }
}

impl<'a, T, const DIM: usize> NoiseFn<T, DIM> for Abs<'a, T, DIM> {
    fn get(&self, point: [T; DIM]) -> f64 {
        (self.source.get(point)).abs()
    }
}
