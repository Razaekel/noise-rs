use crate::noise_fns::NoiseFn;

/// Noise function that outputs the larger of the two output values from two source
/// functions.
pub struct Max<T, U, const DIM: usize>
where
    T: NoiseFn<DIM>,
    U: NoiseFn<DIM>,
{
    /// Outputs a value.
    pub source1: T,

    /// Outputs a value.
    pub source2: U,
}

impl<T, U, const DIM: usize> Max<T, U, DIM>
where
    T: NoiseFn<DIM>,
    U: NoiseFn<DIM>,
{
    pub fn new(source1: T, source2: U) -> Self {
        Self { source1, source2 }
    }
}

impl<T, U, const DIM: usize> NoiseFn<DIM> for Max<T, U, DIM>
where
    T: NoiseFn<DIM>,
    U: NoiseFn<DIM>,
{
    fn get(&self, point: [f64; DIM]) -> f64 {
        (self.source1.get(point)).max(self.source2.get(point))
    }
}
