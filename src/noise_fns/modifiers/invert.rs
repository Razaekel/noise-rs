use noise_fns::NoiseFn;
use rayon::prelude::*;

/// Noise function that inverts the output value from the source function.
pub struct Invert<'a, T: 'a> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T>,
}

impl<'a, T> Invert<'a, T> {
    pub fn new(source: &'a dyn NoiseFn<T>) -> Self {
        Invert { source }
    }
}

impl<'a, T> NoiseFn<T> for Invert<'a, T> {
    fn generate(&self, points: &[T]) -> Vec<f64> {
        self.source
            .generate(points)
            .par_iter()
            .map(|value| -value)
            .collect()
    }
}
