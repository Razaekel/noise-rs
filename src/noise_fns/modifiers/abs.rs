use noise_fns::NoiseFn;

/// Noise function that outputs the absolute value of the output value from the
/// source function.
pub struct Abs<'a, T: 'a> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T>,
}

impl<'a, T> Abs<'a, T> {
    pub fn new(source: &'a dyn NoiseFn<T>) -> Self {
        Abs { source }
    }
}

impl<'a, T> NoiseFn<T> for Abs<'a, T> {
    fn get(&self, point: T) -> f64 {
        (self.source.get(point)).abs()
    }
}
