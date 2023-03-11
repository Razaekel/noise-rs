use alloc::vec::Vec;

use crate::{
    math::{
        scale_shift,
        vectors::*,
    },
    noise_fns::{
        MultiFractal,
        NoiseFn,
        Seedable,
    },
};

/// Noise function that outputs "billowy" noise.
///
/// This noise function produces "billowy" noise suitable for clouds and rocks.
///
/// This noise function is nearly identical to fBm noise, except this noise
/// function modifies each octave with an absolute-value function. See the
/// documentation for fBm for more information.
#[derive(Clone, Debug)]
pub struct Billow<T> {
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
    scale_factor: f64,
}

impl<T> Billow<T>
where
    T: Default + Seedable,
{
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_OCTAVE_COUNT: usize = 6;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;
    pub const DEFAULT_LACUNARITY: f64 = core::f64::consts::PI * 2.0 / 3.0;
    pub const DEFAULT_PERSISTENCE: f64 = 0.5;
    pub const MAX_OCTAVES: usize = 32;

    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            octaves: Self::DEFAULT_OCTAVE_COUNT,
            frequency: Self::DEFAULT_FREQUENCY,
            lacunarity: Self::DEFAULT_LACUNARITY,
            persistence: Self::DEFAULT_PERSISTENCE,
            sources: super::build_sources(seed, Self::DEFAULT_OCTAVE_COUNT),
            scale_factor: Self::calc_scale_factor(
                Self::DEFAULT_PERSISTENCE,
                Self::DEFAULT_OCTAVE_COUNT,
            ),
        }
    }

    fn calc_scale_factor(persistence: f64, octaves: usize) -> f64 {
        let denom = (1..=octaves).fold(0.0, |acc, x| acc + persistence.powi(x as i32));

        1.0 / denom
    }

    pub fn set_sources(self, sources: Vec<T>) -> Self {
        Self { sources, ..self }
    }
}

impl<T> Default for Billow<T>
where
    T: Default + Seedable,
{
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl<T> MultiFractal for Billow<T>
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
            scale_factor: Self::calc_scale_factor(self.persistence, octaves),
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
            scale_factor: Self::calc_scale_factor(persistence, self.octaves),
            ..self
        }
    }
}

impl<T> Seedable for Billow<T>
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

/// 2-dimensional Billow noise
impl<T> NoiseFn<f64, 2> for Billow<T>
where
    T: NoiseFn<f64, 2>,
{
    fn get(&self, point: [f64; 2]) -> f64 {
        let mut point = Vector2::from(point);

        let mut result = 0.0;

        point *= self.frequency;

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point.into_array());

            // Take the abs of the signal, then scale and shift back to
            // the [-1,1] range.
            signal = scale_shift(signal, 2.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi((x as i32) + 1);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency for the next octave.
            point *= self.lacunarity;
        }

        // Scale the result to the [-1,1] range.
        result * self.scale_factor
    }
}

/// 3-dimensional Billow noise
impl<T> NoiseFn<f64, 3> for Billow<T>
where
    T: NoiseFn<f64, 3>,
{
    fn get(&self, point: [f64; 3]) -> f64 {
        let mut point = Vector3::from(point);

        let mut result = 0.0;

        point *= self.frequency;

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point.into_array());

            // Take the abs of the signal, then scale and shift back to
            // the [-1,1] range.
            signal = scale_shift(signal, 2.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi((x as i32) + 1);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency for the next octave.
            point *= self.lacunarity;
        }

        // Scale the result to the [-1,1] range.
        result * self.scale_factor
    }
}

/// 4-dimensional Billow noise
impl<T> NoiseFn<f64, 4> for Billow<T>
where
    T: NoiseFn<f64, 4>,
{
    fn get(&self, point: [f64; 4]) -> f64 {
        let mut point = Vector4::from(point);

        let mut result = 0.0;

        point *= self.frequency;

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point.into_array());

            // Take the abs of the signal, then scale and shift back to
            // the [-1,1] range.
            signal = scale_shift(signal, 2.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi((x as i32) + 1);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency for the next octave.
            point *= self.lacunarity;
        }

        // Scale the result to the [-1,1] range.
        result * self.scale_factor
    }
}
