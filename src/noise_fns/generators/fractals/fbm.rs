use crate::math;
use crate::noise_fns::{MultiFractal, NoiseFn, Perlin, Seedable};
use rayon::prelude::*;
use std;

/// Noise function that outputs fBm (fractal Brownian motion) noise.
///
/// fBm is a _monofractal_ method. In essence, fBm has a _constant_ fractal
/// dimension. It is as close to statistically _homogeneous_ and _isotropic_
/// as possible. Homogeneous means "the same everywhere" and isotropic means
/// "the same in all directions" (note that the two do not mean the same
/// thing).
///
/// The main difference between fractal Brownian motion and regular Brownian
/// motion is that while the increments in Brownian motion are independent,
/// the increments in fractal Brownian motion depend on the previous increment.
///
/// fBm is the result of several noise functions of ever-increasing frequency
/// and ever-decreasing amplitude.
///
/// fBm is commonly referred to as Perlin noise.
#[derive(Clone, Debug)]
pub struct Fbm {
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
    sources: Vec<Perlin>,
}

impl Fbm {
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_OCTAVE_COUNT: usize = 6;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;
    pub const DEFAULT_LACUNARITY: f64 = std::f64::consts::PI * 2.0 / 3.0;
    pub const DEFAULT_PERSISTENCE: f64 = 0.5;
    pub const MAX_OCTAVES: usize = 32;

    pub fn new() -> Self {
        Self {
            seed: Self::DEFAULT_SEED,
            octaves: Self::DEFAULT_OCTAVE_COUNT,
            frequency: Self::DEFAULT_FREQUENCY,
            lacunarity: Self::DEFAULT_LACUNARITY,
            persistence: Self::DEFAULT_PERSISTENCE,
            sources: super::build_sources(Self::DEFAULT_SEED, Self::DEFAULT_OCTAVE_COUNT),
        }
    }
}

impl Default for Fbm {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiFractal for Fbm {
    fn set_octaves(self, mut octaves: usize) -> Self {
        if self.octaves == octaves {
            return self;
        }

        octaves = math::clamp(octaves, 1, Self::MAX_OCTAVES);
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

impl Seedable for Fbm {
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

/// 2-dimensional Fbm noise
impl NoiseFn<[f64; 2]> for Fbm {
    fn generate(&self, points: &[[f64; 2]]) -> Vec<f64> {
        let frequency = self.frequency;
        let lacunarity = self.lacunarity;
        let persistence = self.persistence;

        let mut results = vec![0.0; points.len()];

        let mut points = points
            .par_iter()
            .map(|point| math::mul2(*point, frequency))
            .collect::<Vec<_>>();

        for x in 0..self.octaves {
            // Get the signal, scale the amplitude appropriately for this frequency.
            results = self.sources[x]
                .generate(&points)
                .par_iter()
                .map(|signal| *signal * persistence.powi(x as i32))
                .zip(&results)
                // Add the signal to the result.
                .map(|(signal, result)| signal + result)
                .collect();

            // Increase the frequency for the next octave.
            points = points
                .par_iter()
                .map(|point| math::mul2(*point, lacunarity))
                .collect();
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - self.persistence.powi(self.octaves as i32 - 1);

        results.par_iter().map(|result| result / scale).collect()
    }
}

/// 3-dimensional Fbm noise
impl NoiseFn<[f64; 3]> for Fbm {
    fn generate(&self, points: &[[f64; 3]]) -> Vec<f64> {
        let frequency = self.frequency;
        let lacunarity = self.lacunarity;
        let persistence = self.persistence;

        let mut results = vec![0.0; points.len()];

        let mut points = points
            .par_iter()
            .map(|point| math::mul3(*point, frequency))
            .collect::<Vec<_>>();

        for x in 0..self.octaves {
            // Get the signal, scale the amplitude appropriately for this frequency.
            results = self.sources[x]
                .generate(&points)
                .par_iter()
                .map(|signal| *signal * persistence.powi(x as i32))
                .zip(&results)
                // Add the signal to the result.
                .map(|(signal, result)| signal + result)
                .collect();

            // Increase the frequency for the next octave.
            points = points
                .par_iter()
                .map(|point| math::mul3(*point, lacunarity))
                .collect();
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - self.persistence.powi(self.octaves as i32 - 1);

        results.par_iter().map(|result| result / scale).collect()
    }
}

/// 4-dimensional Fbm noise
impl NoiseFn<[f64; 4]> for Fbm {
    fn generate(&self, points: &[[f64; 4]]) -> Vec<f64> {
        let frequency = self.frequency;
        let lacunarity = self.lacunarity;
        let persistence = self.persistence;

        let mut results = vec![0.0; points.len()];

        let mut points = points
            .par_iter()
            .map(|point| math::mul4(*point, frequency))
            .collect::<Vec<_>>();

        for x in 0..self.octaves {
            // Get the signal, scale the amplitude appropriately for this frequency.
            results = self.sources[x]
                .generate(&points)
                .par_iter()
                .map(|signal| *signal * persistence.powi(x as i32))
                .zip(&results)
                // Add the signal to the result.
                .map(|(signal, result)| signal + result)
                .collect();

            // Increase the frequency for the next octave.
            points = points
                .par_iter()
                .map(|point| math::mul4(*point, lacunarity))
                .collect();
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - self.persistence.powi(self.octaves as i32 - 1);

        results.par_iter().map(|result| result / scale).collect()
    }
}
