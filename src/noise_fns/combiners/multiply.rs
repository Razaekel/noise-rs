use crate::noise_fns::NoiseFn;

/// Noise function that outputs the product of the two output values from two source
/// functions.
pub struct Multiply<'a, T, const DIM: usize> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T, DIM>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T, DIM>,
}

impl<'a, T, const DIM: usize> Multiply<'a, T, DIM> {
    pub fn new(source1: &'a dyn NoiseFn<T, DIM>, source2: &'a dyn NoiseFn<T, DIM>) -> Self {
        Self { source1, source2 }
    }
}

impl<'a, T, const DIM: usize> NoiseFn<T, DIM> for Multiply<'a, T, DIM>
where
    T: Copy,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        self.source1.get(point) * self.source2.get(point)
    }
}
