use crate::noise_fns::NoiseFn;

/// Noise function that outputs a constant value.
///
/// This function takes a input, value, and returns that input for all points,
/// producing a constant-valued field.
///
/// This function is not very useful by itself, but can be used as a source
/// function for other noise functions.
#[derive(Clone, Copy, Debug)]
pub struct Constant<F> {
    /// Constant value.
    pub value: F,
}

impl<F> Constant<F> {
    pub fn new(value: F) -> Self {
        Self { value }
    }
}

impl<F: Copy, const N: usize> NoiseFn<F, N> for Constant<F> {
    fn get(&self, _point: [F; N]) -> F {
        self.value
    }
}
