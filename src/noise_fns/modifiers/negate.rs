use crate::noise_fns::NoiseFn;

/// Noise function that negates the output value from the source function.
pub struct Negate<'a, T, const DIM: usize> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T, DIM>,
}

impl<'a, T, const DIM: usize> Negate<'a, T, DIM> {
    pub fn new(source: &'a dyn NoiseFn<T, DIM>) -> Self {
        Negate { source }
    }
}

impl<'a, T, const DIM: usize> NoiseFn<T, DIM> for Negate<'a, T, DIM> {
    fn get(&self, point: [T; DIM]) -> f64 {
        -self.source.get(point)
    }
}
