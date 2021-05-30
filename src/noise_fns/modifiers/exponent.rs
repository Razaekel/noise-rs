use crate::{math::scale_shift, noise_fns::NoiseFn, MultiFractal, Seedable};

/// Noise function that maps the output value from the source function onto an
/// exponential curve.
///
/// Because most noise functions will output values that range from -1.0 to 1.0,
/// this noise function first normalizes the output value (the range becomes 0.0
/// to 1.0), maps that value onto an exponential curve, then rescales that
/// value back to the original range.
pub struct Exponent<T, const N: usize>
where
    T: NoiseFn<N>,
{
    /// Outputs a value.
    pub source: T,

    /// Exponent to apply to the output value from the source function. Default
    /// is 1.0.
    pub exponent: f64,
}

impl<T, const N: usize> Exponent<T, N>
where
    T: NoiseFn<N>,
{
    pub fn new(source: T) -> Self {
        Self {
            source,
            exponent: 1.0,
        }
    }

    pub fn set_exponent(self, exponent: f64) -> Self {
        Self { exponent, ..self }
    }
}

impl<T, const N: usize> NoiseFn<N> for Exponent<T, N>
where
    T: NoiseFn<N>,
{
    fn get(&self, point: [f64; N]) -> f64 {
        let mut value = self.source.get(point);
        value = (value + 1.0) / 2.0;
        value = value.abs();
        value = value.powf(self.exponent);
        scale_shift(value, 2.0)
    }
}

impl<T, const N: usize> Seedable for Exponent<T, N>
where
    T: NoiseFn<N> + Seedable,
{
    fn set_seed(self, seed: u32) -> Self {
        Self {
            source: self.source.set_seed(seed),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.source.seed()
    }
}

impl<T, const N: usize> MultiFractal for Exponent<T, N>
where
    T: NoiseFn<N> + MultiFractal,
{
    fn set_octaves(self, octaves: usize) -> Self {
        Self {
            source: self.source.set_octaves(octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Self {
            source: self.source.set_frequency(frequency),
            ..self
        }
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self {
            source: self.source.set_lacunarity(lacunarity),
            ..self
        }
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Self {
            source: self.source.set_persistence(persistence),
            ..self
        }
    }
}
