use math::interp;
use noise_fns::NoiseFn;

/// Noise function that outputs a weighted blend of the output values from two
/// source functions given the output value supplied by a control function.
///
/// This noise function uses linear interpolation to perform the blending
/// operation.
pub struct Blend<'a, T: 'a> {
    /// Outputs one of the values to blend.
    pub source1: &'a dyn NoiseFn<T>,

    /// Outputs one of the values to blend.
    pub source2: &'a dyn NoiseFn<T>,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the `source1` function. Positive
    /// values weight the blend towards the output value from the `source2`
    /// function.
    pub control: &'a dyn NoiseFn<T>,
}

impl<'a, T> Blend<'a, T> {
    pub fn new(source1: &'a dyn NoiseFn<T>, source2: &'a dyn NoiseFn<T>, control: &'a dyn NoiseFn<T>) -> Self {
        Blend {
            source1,
            source2,
            control,
        }
    }
}

impl<'a, T> NoiseFn<T> for Blend<'a, T>
where
    T: Copy,
{
    fn get(&self, point: T) -> f64 {
        let lower = self.source1.get(point);
        let upper = self.source2.get(point);
        let control = self.control.get(point);

        interp::linear(lower, upper, control)
    }
}
