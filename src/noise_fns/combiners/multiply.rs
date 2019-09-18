use noise_fns::NoiseFn;

/// Noise function that outputs the product of the two output values from two source
/// functions.
pub struct Multiply<'a, T: 'a> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T>,
}

impl<'a, T> Multiply<'a, T> {
    pub fn new(source1: &'a dyn NoiseFn<T>, source2: &'a dyn NoiseFn<T>) -> Self {
        Multiply { source1, source2 }
    }
}

impl<'a, T> NoiseFn<T> for Multiply<'a, T>
where
    T: Copy,
{
    fn get(&self, point: T) -> f64 {
        self.source1.get(point) * self.source2.get(point)
    }
}
