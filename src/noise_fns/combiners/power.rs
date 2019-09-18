use noise_fns::NoiseFn;

/// Noise function that raises the output value from the first source function
/// to the power of the output value of the second source function.
pub struct Power<'a, T: 'a> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T>,
}

impl<'a, T> Power<'a, T> {
    pub fn new(source1: &'a dyn NoiseFn<T>, source2: &'a dyn NoiseFn<T>) -> Self {
        Power { source1, source2 }
    }
}

impl<'a, T> NoiseFn<T> for Power<'a, T>
where
    T: Copy,
{
    fn get(&self, point: T) -> f64 {
        (self.source1.get(point)).powf(self.source2.get(point))
    }
}
