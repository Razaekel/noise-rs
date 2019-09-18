use noise_fns::NoiseFn;

/// Noise function that outputs the smaller of the two output values from two source
/// functions.
pub struct Min<'a, T: 'a> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T>,
}

impl<'a, T> Min<'a, T> {
    pub fn new(source1: &'a dyn NoiseFn<T>, source2: &'a dyn NoiseFn<T>) -> Self {
        Min { source1, source2 }
    }
}

impl<'a, T> NoiseFn<T> for Min<'a, T>
where
    T: Copy,
{
    fn get(&self, point: T) -> f64 {
        (self.source1.get(point)).min(self.source2.get(point))
    }
}
