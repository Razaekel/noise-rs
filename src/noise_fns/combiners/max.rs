use crate::noise_fns::NoiseFn;
use rayon::prelude::*;

/// Noise function that outputs the larger of the two output values from two source
/// functions.
pub struct Max<'a, T: 'a> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T>,
}

impl<'a, T> Max<'a, T> {
    pub fn new(source1: &'a dyn NoiseFn<T>, source2: &'a dyn NoiseFn<T>) -> Self {
        Self { source1, source2 }
    }
}

impl<'a, T> NoiseFn<T> for Max<'a, T>
where
    T: Copy,
{
    fn generate(&self, points: &[T]) -> Vec<f64> {
        self.source1
            .generate(points)
            .par_iter()
            .zip(self.source2.generate(points))
            .map(|(value1, value2)| value1.max(value2))
            .collect()
    }
}
