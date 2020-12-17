use crate::noise_fns::NoiseFn;

/// Noise function that outputs the larger of the two output values from two source
/// functions.
pub struct Max<'a, T, const N: usize> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T, N>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T, N>,
}

impl<'a, T, const N: usize> Max<'a, T, N> {
    pub fn new(source1: &'a dyn NoiseFn<T, N>, source2: &'a dyn NoiseFn<T, N>) -> Self {
        Self { source1, source2 }
    }
}

impl<'a, T, const N: usize> NoiseFn<T, N> for Max<'a, T, N>
where
    T: Copy,
{
    fn get(&self, point: [T; N]) -> f64 {
        (self.source1.get(point)).max(self.source2.get(point))
    }
}
