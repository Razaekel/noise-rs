use crate::noise_fns::NoiseFn;

/// Noise function that outputs the sum of the two output values from two source
/// functions.
pub struct Add<'a, T, U, const DIM: usize>
where
    T: NoiseFn<DIM>,
    U: NoiseFn<DIM>,
{
    /// Outputs a value.
    pub source1: &'a T,

    /// Outputs a value.
    pub source2: &'a U,
}

impl<'a, T, U, const DIM: usize> Add<'a, T, U, DIM>
where
    T: NoiseFn<DIM>,
    U: NoiseFn<DIM>,
{
    pub fn new(source1: &'a T, source2: &'a U) -> Self {
        Self { source1, source2 }
    }
}

impl<'a, T, U, const DIM: usize> NoiseFn<DIM> for Add<'a, T, U, DIM>
where
    T: NoiseFn<DIM>,
    U: NoiseFn<DIM>,
{
    fn get(&self, point: [f64; DIM]) -> f64 {
        self.source1.get(point) + self.source2.get(point)
    }
}
