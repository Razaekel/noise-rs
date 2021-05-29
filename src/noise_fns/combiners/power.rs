use crate::noise_fns::NoiseFn;

/// Noise function that raises the output value from the first source function
/// to the power of the output value of the second source function.
pub struct Power<T, U, const DIM: usize>
where
    T: NoiseFn<DIM>,
    U: NoiseFn<DIM>,
{
    /// Outputs a value.
    pub source1: T,

    /// Outputs a value.
    pub source2: U,
}

impl<T, U, const DIM: usize> Power<T, U, DIM>
where
    T: NoiseFn<DIM>,
    U: NoiseFn<DIM>,
{
    pub fn new(source1: T, source2: U) -> Self {
        Self { source1, source2 }
    }
}

impl<T, U, const DIM: usize> NoiseFn<DIM> for Power<T, U, DIM>
where
    T: NoiseFn<DIM>,
    U: NoiseFn<DIM>,
{
    fn get(&self, point: [f64; DIM]) -> f64 {
        (self.source1.get(point)).powf(self.source2.get(point))
    }
}
