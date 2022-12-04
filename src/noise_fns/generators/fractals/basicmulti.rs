use crate::{
    math::vectors::*,
    noise_fns::{MultiFractal, NoiseFn, Seedable},
};
use alloc::vec::Vec;

/// Noise function that outputs heterogenous Multifractal noise.
///
/// This is a multifractal method, meaning that it has a fractal dimension
/// that varies with location.
///
/// The result of this multifractal method is that in areas near zero, higher
/// frequencies will be heavily damped, resulting in the terrain remaining
/// smooth. As the value moves further away from zero, higher frequencies will
/// not be as damped and thus will grow more jagged as iteration progresses.
///
#[derive(Clone, Debug)]
pub struct BasicMulti<T> {
    /// Total number of frequency octaves to generate the noise with.
    ///
    /// The number of octaves control the _amount of detail_ in the noise
    /// function. Adding more octaves increases the detail, with the drawback
    /// of increasing the calculation time.
    pub octaves: usize,

    /// The number of cycles per unit length that the noise function outputs.
    pub frequency: f64,

    /// A multiplier that determines how quickly the frequency increases for
    /// each successive octave in the noise function.
    ///
    /// The frequency of each successive octave is equal to the product of the
    /// previous octave's frequency and the lacunarity value.
    ///
    /// A lacunarity of 2.0 results in the frequency doubling every octave. For
    /// almost all cases, 2.0 is a good value to use.
    pub lacunarity: f64,

    /// A multiplier that determines how quickly the amplitudes diminish for
    /// each successive octave in the noise function.
    ///
    /// The amplitude of each successive octave is equal to the product of the
    /// previous octave's amplitude and the persistence value. Increasing the
    /// persistence produces "rougher" noise.
    pub persistence: f64,

    seed: u32,
    sources: Vec<T>,
}

impl<T> BasicMulti<T>
where
    T: Default + Seedable,
{
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_OCTAVES: usize = 6;
    pub const DEFAULT_FREQUENCY: f64 = 2.0;
    pub const DEFAULT_LACUNARITY: f64 = core::f64::consts::PI * 2.0 / 3.0;
    pub const DEFAULT_PERSISTENCE: f64 = 0.5;
    pub const MAX_OCTAVES: usize = 32;

    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            octaves: Self::DEFAULT_OCTAVES,
            frequency: Self::DEFAULT_FREQUENCY,
            lacunarity: Self::DEFAULT_LACUNARITY,
            persistence: Self::DEFAULT_PERSISTENCE,
            sources: super::build_sources(seed, Self::DEFAULT_OCTAVES),
        }
    }

    pub fn set_sources(self, sources: Vec<T>) -> Self {
        Self { sources, ..self }
    }
}

impl<T> Default for BasicMulti<T>
where
    T: Default + Seedable,
{
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl<T> MultiFractal for BasicMulti<T>
where
    T: Default + Seedable,
{
    fn set_octaves(self, mut octaves: usize) -> Self {
        if self.octaves == octaves {
            return self;
        }

        octaves = octaves.clamp(1, Self::MAX_OCTAVES);
        Self {
            octaves,
            sources: super::build_sources(self.seed, octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self { lacunarity, ..self }
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Self {
            persistence,
            ..self
        }
    }
}

impl<T> Seedable for BasicMulti<T>
where
    T: Default + Seedable,
{
    fn set_seed(self, seed: u32) -> Self {
        if self.seed == seed {
            return self;
        }

        Self {
            seed,
            sources: super::build_sources(seed, self.octaves),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional `BasicMulti` noise
impl<T> NoiseFn<f64, 2> for BasicMulti<T>
where
    T: NoiseFn<f64, 2>,
{
    fn get(&self, point: [f64; 2]) -> f64 {
        let mut point = Vector2::from(point);

        // First unscaled octave of function; later octaves are scaled.
        point *= self.frequency;
        let mut result = self.sources[0].get(point.into_array());

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Raise the spatial frequency.
            point *= self.lacunarity;

            // Get noise value.
            let mut signal = self.sources[x].get(point.into_array());

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Scale the signal by the current 'altitude' of the function.
            signal *= result;

            // Add signal to result.
            result += signal;
        }

        // Scale the result to the [-1,1] range.
        result * 0.5
    }
}

/// 3-dimensional `BasicMulti` noise
impl<T> NoiseFn<f64, 3> for BasicMulti<T>
where
    T: NoiseFn<f64, 3>,
{
    fn get(&self, point: [f64; 3]) -> f64 {
        let mut point = Vector3::from(point);

        // First unscaled octave of function; later octaves are scaled.
        point *= self.frequency;
        let mut result = self.sources[0].get(point.into_array());

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Raise the spatial frequency.
            point *= self.lacunarity;

            // Get noise value.
            let mut signal = self.sources[x].get(point.into_array());

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Scale the signal by the current 'altitude' of the function.
            signal *= result;

            // Add signal to result.
            result += signal;
        }

        // Scale the result to the [-1,1] range.
        result * 0.5
    }
}

/// 4-dimensional `BasicMulti` noise
impl<T> NoiseFn<f64, 4> for BasicMulti<T>
where
    T: NoiseFn<f64, 4>,
{
    fn get(&self, point: [f64; 4]) -> f64 {
        let mut point = Vector4::from(point);

        // First unscaled octave of function; later octaves are scaled.
        point *= self.frequency;
        let mut result = self.sources[0].get(point.into_array());

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Raise the spatial frequency.
            point *= self.lacunarity;

            // Get noise value.
            let mut signal = self.sources[x].get(point.into_array());

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Scale the signal by the current 'altitude' of the function.
            signal *= result;

            // Add signal to result.
            result += signal;
        }

        // Scale the result to the [-1,1] range.
        result * 0.5
    }
}
