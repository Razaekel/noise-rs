use noise_fns::NoiseFn;

/// Noise function that outputs the larger of the two output values from two source
/// functions.
pub struct Max<'a, T: 'a> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T>,
}

impl<'a, T> Max<'a, T> {
    pub fn new(source1: &'a dyn NoiseFn<T>, source2: &'a dyn NoiseFn<T>) -> Self {
        Max { source1, source2 }
    }
}

impl<'a, T> NoiseFn<T> for Max<'a, T>
where
    T: Copy,
{
    fn get(&self, point: T) -> f64 {
        (self.source1.get(point)).max(self.source2.get(point))
    }
}
