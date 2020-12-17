use crate::noise_fns::NoiseFn;

/// Noise function that negates the output value from the source function.
pub struct Negate<'a, T, const N: usize> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T, N>,
}

impl<'a, T, const N: usize> Negate<'a, T, N> {
    pub fn new(source: &'a dyn NoiseFn<T, N>) -> Self {
        Negate { source }
    }
}

impl<'a, T, const N: usize> NoiseFn<T, N> for Negate<'a, T, N> {
    fn get(&self, point: [T; N]) -> f64 {
        -self.source.get(point)
    }
}
