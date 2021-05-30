use crate::{noise_fns::NoiseFn, MultiFractal, Seedable};

/// Noise function that applies a scaling factor and a bias to the output value
/// from the source function.
///
/// The function retrieves the output value from the source function, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
pub struct ScaleBias<T, const N: usize>
where
    T: NoiseFn<N>,
{
    /// Outputs a value.
    pub source: T,

    /// Scaling factor to apply to the output value from the source function.
    /// The default value is 1.0.
    pub scale: f64,

    /// Bias to apply to the scaled output value from the source function.
    /// The default value is 0.0.
    pub bias: f64,
}

impl<T, const N: usize> ScaleBias<T, N>
where
    T: NoiseFn<N>,
{
    pub fn new(source: T) -> Self {
        Self {
            source,
            scale: 1.0,
            bias: 0.0,
        }
    }

    pub fn set_scale(self, scale: f64) -> Self {
        Self { scale, ..self }
    }

    pub fn set_bias(self, bias: f64) -> Self {
        Self { bias, ..self }
    }
}

impl<T, const N: usize> NoiseFn<N> for ScaleBias<T, N>
where
    T: NoiseFn<N>,
{
    #[cfg(not(target_os = "emscripten"))]
    fn get(&self, point: [f64; N]) -> f64 {
        (self.source.get(point)).mul_add(self.scale, self.bias)
    }

    #[cfg(target_os = "emscripten")]
    fn get(&self, point: [T; N]) -> f64 {
        (self.source.get(point) * self.scale) + self.bias
    }
}

impl<T, const N: usize> Seedable for ScaleBias<T, N>
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

impl<T, const N: usize> MultiFractal for ScaleBias<T, N>
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
