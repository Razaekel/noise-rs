use crate::noise_fns::NoiseFn;

/// Noise function that raises the output value from the first source function
/// to the power of the output value of the second source function.
pub struct Power<'a, T, const N: usize> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T, N>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T, N>,
}

impl<'a, T, const N: usize> Power<'a, T, N> {
    pub fn new(source1: &'a dyn NoiseFn<T, N>, source2: &'a dyn NoiseFn<T, N>) -> Self {
        Self { source1, source2 }
    }
}

impl<'a, T, const N: usize> NoiseFn<T, N> for Power<'a, T, N>
where
    T: Copy,
{
    fn get(&self, point: [T; N]) -> f64 {
        (self.source1.get(point)).powf(self.source2.get(point))
    }
}
