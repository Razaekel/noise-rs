use crate::noise_fns::NoiseFn;

/// Noise function that negates the output value from the source function.
pub struct Negate<'a, T> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T>,
}

impl<'a, T> Negate<'a, T> {
    pub fn new(source: &'a dyn NoiseFn<T>) -> Self {
        Negate { source }
    }
}

impl<'a, T> NoiseFn<T> for Negate<'a, T> {
    fn get(&self, point: T) -> f64 {
        -self.source.get(point)
    }
}
