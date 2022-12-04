use crate::{math::scale_shift, noise_fns::NoiseFn};
use core::marker::PhantomData;

/// Noise function that maps the output value from the source function onto an
/// exponential curve.
///
/// Because most noise functions will output values that range from -1.0 to 1.0,
/// this noise function first normalizes the output value (the range becomes 0.0
/// to 1.0), maps that value onto an exponential curve, then rescales that
/// value back to the original range.
pub struct Exponent<T, Source, const DIM: usize>
where
    Source: NoiseFn<T, DIM>,
{
    /// Outputs a value.
    pub source: Source,

    /// Exponent to apply to the output value from the source function. Default
    /// is 1.0.
    pub exponent: f64,

    phantom: PhantomData<T>,
}

impl<T, Source, const DIM: usize> Exponent<T, Source, DIM>
where
    Source: NoiseFn<T, DIM>,
{
    pub fn new(source: Source) -> Self {
        Self {
            source,
            exponent: 1.0,
            phantom: PhantomData,
        }
    }

    pub fn set_exponent(self, exponent: f64) -> Self {
        Self { exponent, ..self }
    }
}

impl<T, Source, const DIM: usize> NoiseFn<T, DIM> for Exponent<T, Source, DIM>
where
    Source: NoiseFn<T, DIM>,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        let mut value = self.source.get(point);
        value = (value + 1.0) / 2.0;
        value = value.abs();
        value = value.powf(self.exponent);
        scale_shift(value, 2.0)
    }
}
