use crate::{noise_fns::NoiseFn, MultiFractal, Seedable};

/// Noise function that clamps the output value from the source function to a
/// range of values.
pub struct Clamp<T, const N: usize>
where
    T: NoiseFn<N>,
{
    /// Outputs a value.
    pub source: T,

    /// Bound of the clamping range. Default is -1.0 to 1.0.
    pub bounds: (f64, f64),
}

impl<T, const N: usize> Clamp<T, N>
where
    T: NoiseFn<N>,
{
    pub fn new(source: T) -> Self {
        Self {
            source,
            bounds: (-1.0, 1.0),
        }
    }

    pub fn set_lower_bound(self, lower_bound: f64) -> Self {
        Self {
            bounds: (lower_bound, self.bounds.1),
            ..self
        }
    }

    pub fn set_upper_bound(self, upper_bound: f64) -> Self {
        Self {
            bounds: (self.bounds.0, upper_bound),
            ..self
        }
    }

    pub fn set_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            bounds: (lower_bound, upper_bound),
            ..self
        }
    }
}

impl<T, const N: usize> NoiseFn<N> for Clamp<T, N>
where
    T: NoiseFn<N>,
{
    fn get(&self, point: [f64; N]) -> f64 {
        let value = self.source.get(point);

        value.clamp(self.bounds.0, self.bounds.1)
    }
}

impl<T, const N: usize> Seedable for Clamp<T, N>
where
    T: NoiseFn<N> + Seedable,
{
    fn new(seed: u32) -> Self {
        Self {
            source: T::new(seed),
            bounds: (-1.0, 1.0),
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

impl<T, const N: usize> MultiFractal for Clamp<T, N>
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
