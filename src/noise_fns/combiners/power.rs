use crate::noise_fns::NoiseFn;

/// Noise function that raises the output value from the first source function
/// to the power of the output value of the second source function.
pub struct Power<T, U, const N: usize>
where
    T: NoiseFn<N>,
    U: NoiseFn<N>,
{
    /// Outputs a value.
    pub source1: T,

    /// Outputs a value.
    pub source2: U,
}

impl<T, U, const N: usize> Power<T, U, N>
where
    T: NoiseFn<N>,
    U: NoiseFn<N>,
{
    pub fn new(source1: T, source2: U) -> Self {
        Self { source1, source2 }
    }
}

impl<T, U, const N: usize> NoiseFn<N> for Power<T, U, N>
where
    T: NoiseFn<N>,
    U: NoiseFn<N>,
{
    fn get(&self, point: [f64; N]) -> f64 {
        (self.source1.get(point)).powf(self.source2.get(point))
    }
}
