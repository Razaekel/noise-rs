use crate::{noise_fns::NoiseFn, MultiFractal, Seedable};

/// Noise function that outputs the absolute value of the output value from the
/// source function.
pub struct Abs<T, const N: usize>
where
    T: NoiseFn<N>,
{
    /// Outputs a value.
    pub source: T,
}

impl<T, const N: usize> Abs<T, N>
where
    T: NoiseFn<N>,
{
    pub fn new(source: T) -> Self {
        Self { source }
    }
}

impl<T, const N: usize> NoiseFn<N> for Abs<T, N>
where
    T: NoiseFn<N>,
{
    fn get(&self, point: [f64; N]) -> f64 {
        (self.source.get(point)).abs()
    }
}

impl<T, const N: usize> Seedable for Abs<T, N>
where
    T: NoiseFn<N> + Seedable,
{
    fn new(seed: u32) -> Self {
        Self {
            source: T::new(seed),
        }
    }

    fn set_seed(self, seed: u32) -> Self {
        Self::new(self.source.set_seed(seed))
    }

    fn seed(&self) -> u32 {
        self.source.seed()
    }
}

impl<T, const N: usize> MultiFractal for Abs<T, N>
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
