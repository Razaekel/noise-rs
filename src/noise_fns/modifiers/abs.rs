use crate::noise_fns::NoiseFn;

/// Noise function that outputs the absolute value of the output value from the
/// source function.
pub struct Abs<'a, T, const N: usize> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T, N>,
}

impl<'a, T, const N: usize> Abs<'a, T, N> {
    pub fn new(source: &'a dyn NoiseFn<T, N>) -> Self {
        Self { source }
    }
}

impl<'a, T, const N: usize> NoiseFn<T, N> for Abs<'a, T, N> {
    fn get(&self, point: [T; N]) -> f64 {
        (self.source.get(point)).abs()
    }
}
