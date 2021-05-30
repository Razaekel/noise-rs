use crate::{noise_fns::NoiseFn, MultiFractal, Seedable};

/// Noise function that negates the output value from the source function.
pub struct Negate<T, const N: usize>
where
    T: NoiseFn<N>,
{
    /// Outputs a value.
    pub source: T,
}

impl<T, const N: usize> Negate<T, N>
where
    T: NoiseFn<N>,
{
    pub fn new(source: T) -> Self {
        Negate { source }
    }
}

impl<T, const N: usize> NoiseFn<N> for Negate<T, N>
where
    T: NoiseFn<N>,
{
    fn get(&self, point: [f64; N]) -> f64 {
        -self.source.get(point)
    }
}

impl<T, const N: usize> Seedable for Negate<T, N>
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

impl<T, const N: usize> MultiFractal for Negate<T, N>
where
    T: NoiseFn<N> + MultiFractal,
{
    fn set_octaves(self, octaves: usize) -> Self {
        Self::new(self.source.set_octaves(octaves))
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Self::new(self.source.set_frequency(frequency))
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self::new(self.source.set_lacunarity(lacunarity))
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Self::new(self.source.set_persistence(persistence))
    }
}
