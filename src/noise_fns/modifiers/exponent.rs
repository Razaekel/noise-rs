use crate::{math::scale_shift, noise_fns::NoiseFn};
use num_traits::Float;

/// Noise function that maps the output value from the source function onto an
/// exponential curve.
///
/// Because most noise functions will output values that range from -1.0 to 1.0,
/// this noise function first normalizes the output value (the range becomes 0.0
/// to 1.0), maps that value onto an exponential curve, then rescales that
/// value back to the original range.
pub struct Exponent<F, Source, const DIM: usize>
where
    Source: NoiseFn<F, DIM>,
{
    /// Outputs a value.
    pub source: Source,

    /// Exponent to apply to the output value from the source function. Default
    /// is 1.0.
    pub exponent: F,
}

impl<F, Source, const DIM: usize> Exponent<F, Source, DIM>
where
    F: Float,
    Source: NoiseFn<F, DIM>,
{
    pub fn new(source: Source) -> Self {
        Self {
            source,
            exponent: F::one(),
        }
    }

    pub fn set_exponent(self, exponent: F) -> Self {
        Self { exponent, ..self }
    }
}

impl<F, Source, const DIM: usize> NoiseFn<F, DIM> for Exponent<F, Source, DIM>
where
    F: Float,
    Source: NoiseFn<F, DIM>,
{
    fn get(&self, point: [F; DIM]) -> F {
        let mut value = self.source.get(point);
        value = (value + F::one()) / (F::one() + F::one());
        value = value.abs();
        value = value.powf(self.exponent);
        scale_shift(value, F::one() + F::one())
    }
}
