use crate::noise_fns::NoiseFn;

/// Noise function that raises the output value from the first source function
/// to the power of the output value of the second source function.
pub struct Power<'a, T, const DIM: usize> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T, DIM>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T, DIM>,
}

impl<'a, T, const DIM: usize> Power<'a, T, DIM> {
    pub fn new(source1: &'a dyn NoiseFn<T, DIM>, source2: &'a dyn NoiseFn<T, DIM>) -> Self {
        Self { source1, source2 }
    }
}

impl<'a, T, const DIM: usize> NoiseFn<T, DIM> for Power<'a, T, DIM>
where
    T: Copy,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        (self.source1.get(point)).powf(self.source2.get(point))
    }
}
