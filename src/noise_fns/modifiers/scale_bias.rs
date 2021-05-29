use crate::{noise_fns::NoiseFn, MultiFractal, Seedable};

/// Noise function that applies a scaling factor and a bias to the output value
/// from the source function.
///
/// The function retrieves the output value from the source function, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
pub struct ScaleBias<T, const DIM: usize>
where
    T: NoiseFn<DIM>,
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

impl<T, const DIM: usize> ScaleBias<T, DIM>
where
    T: NoiseFn<DIM>,
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

impl<T, const DIM: usize> NoiseFn<DIM> for ScaleBias<T, DIM>
where
    T: NoiseFn<DIM>,
{
    #[cfg(not(target_os = "emscripten"))]
    fn get(&self, point: [f64; DIM]) -> f64 {
        (self.source.get(point)).mul_add(self.scale, self.bias)
    }

    #[cfg(target_os = "emscripten")]
    fn get(&self, point: [T; DIM]) -> f64 {
        (self.source.get(point) * self.scale) + self.bias
    }
}

impl<T, const DIM: usize> Seedable for ScaleBias<T, DIM>
where
    T: NoiseFn<DIM> + Seedable,
{
    fn new(seed: u32) -> Self {
        Self {
            source: T::new(seed),
            scale: 1.0,
            bias: 0.0,
        }
    }

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

impl<T, const DIM: usize> MultiFractal for ScaleBias<T, DIM>
where
    T: NoiseFn<DIM> + MultiFractal,
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
