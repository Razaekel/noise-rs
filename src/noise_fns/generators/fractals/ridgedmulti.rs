use alloc::vec::Vec;

use crate::{
    math::vectors::*,
    noise_fns::{
        MultiFractal,
        NoiseFn,
        Seedable,
    },
};

/// Noise function that outputs ridged-multifractal noise.
///
/// This noise function, heavily based on the fBm-noise function, generates
/// ridged-multifractal noise. Ridged-multifractal noise is generated in much
/// the same way as fBm noise, except the output of each octave is modified by
/// an absolute-value function. Modifying the octave values in this way
/// produces ridge-like formations.
///
/// The values output from this function will usually range from -1.0 to 1.0 with
/// default values for the parameters, but there are no guarantees that all
/// output values will exist within this range. If the parameters are modified
/// from their defaults, then the output will need to be scaled to remain in
/// the [-1, 1] range.
///
/// Ridged-multifractal noise is often used to generate craggy mountainous
/// terrain or marble-like textures.
#[derive(Clone, Debug)]
pub struct RidgedMulti<T> {
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

    /// The attenuation to apply to the weight on each octave. This reduces
    /// the strength of each successive octave, making their respective
    /// ridges smaller. The default attenuation is 2.0, making each octave
    /// half the height of the previous.
    pub attenuation: f64,

    seed: u32,
    sources: Vec<T>,
    scale_factor: f64,
}

impl<T> RidgedMulti<T>
where
    T: Default + Seedable,
{
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_OCTAVE_COUNT: usize = 6;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;
    pub const DEFAULT_LACUNARITY: f64 = core::f64::consts::PI * 2.0 / 3.0;
    pub const DEFAULT_PERSISTENCE: f64 = 1.0;
    pub const DEFAULT_ATTENUATION: f64 = 2.0;
    pub const MAX_OCTAVES: usize = 32;

    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            octaves: Self::DEFAULT_OCTAVE_COUNT,
            frequency: Self::DEFAULT_FREQUENCY,
            lacunarity: Self::DEFAULT_LACUNARITY,
            persistence: Self::DEFAULT_PERSISTENCE,
            attenuation: Self::DEFAULT_ATTENUATION,
            sources: super::build_sources(seed, Self::DEFAULT_OCTAVE_COUNT),
            scale_factor: Self::calc_scale_factor(
                Self::DEFAULT_PERSISTENCE,
                Self::DEFAULT_ATTENUATION,
                Self::DEFAULT_OCTAVE_COUNT,
            ),
        }
    }

    pub fn set_attenuation(self, attenuation: f64) -> Self {
        Self {
            attenuation,
            scale_factor: Self::calc_scale_factor(self.persistence, attenuation, self.octaves),
            ..self
        }
    }

    pub fn set_sources(self, sources: Vec<T>) -> Self {
        Self { sources, ..self }
    }

    fn calc_scale_factor(persistence: f64, attenuation: f64, octaves: usize) -> f64 {
        let mut denom = 0.0;

        // Do octave 0
        let mut amplitude = 1.0;
        let mut weight = 1.0;
        let mut signal = weight * amplitude;

        denom += signal;

        if octaves >= 1 {
            denom += (1..=octaves).fold(0.0, |acc, x| {
                amplitude *= persistence;
                weight = (signal / attenuation.powi(x as i32)).clamp(0.0, 1.0);
                signal = weight * amplitude;
                acc + signal
            });
        }

        2.0 / denom
    }
}

impl<T> Default for RidgedMulti<T>
where
    T: Default + Seedable,
{
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl<T> MultiFractal for RidgedMulti<T>
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
            scale_factor: Self::calc_scale_factor(self.persistence, self.attenuation, octaves),
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
            scale_factor: Self::calc_scale_factor(persistence, self.attenuation, self.octaves),
            ..self
        }
    }
}

impl<T> Seedable for RidgedMulti<T>
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

/// 2-dimensional `RidgedMulti` noise
impl<T> NoiseFn<f64, 2> for RidgedMulti<T>
where
    T: NoiseFn<f64, 2>,
{
    fn get(&self, point: [f64; 2]) -> f64 {
        let mut point = Vector2::from(point);

        let mut result = 0.0;
        let mut weight = 1.0;

        point *= self.frequency;

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point.into_array());

            // Make the ridges.
            signal = signal.abs();
            signal = 1.0 - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal *= signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal *= weight;

            // Weight successive contributions by the previous signal.
            weight = signal / self.attenuation;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            weight = weight.clamp(0.0, 1.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency.
            point *= self.lacunarity;
        }

        // The result before scaling will be 0 to something positive, so need to sale it back down
        // to -1 to 1. We don't know what the upper limit is, but it can be calculated based on the
        // number of octaves, and the persistence and attenuation values. By dividing the result by
        // what the upper limit should be / 2, we should get a value between 0 and 2. Then we can
        // shift the result to cover the -1 to 1 range.

        // Scale the result to [0, 2]
        result *= self.scale_factor;

        // Shift the result to [-1, 1]
        result - 1.0
    }
}

/// 3-dimensional `RidgedMulti` noise
impl<T> NoiseFn<f64, 3> for RidgedMulti<T>
where
    T: NoiseFn<f64, 3>,
{
    fn get(&self, point: [f64; 3]) -> f64 {
        let mut point = Vector3::from(point);

        let mut result = 0.0;
        let mut weight = 1.0;

        point *= self.frequency;

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point.into_array());

            // Make the ridges.
            signal = signal.abs();
            signal = 1.0 - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal *= signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal *= weight;

            // Weight successive contributions by the previous signal.
            weight = signal / self.attenuation;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            weight = weight.clamp(0.0, 1.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency.
            point *= self.lacunarity;
        }

        // The result before scaling will be 0 to something positive, so need to sale it back down
        // to -1 to 1. We don't know what the upper limit is, but it can be calculated based on the
        // number of octaves, and the persistence and attenuation values. By dividing the result by
        // what the upper limit should be / 2, we should get a value between 0 and 2. Then we can
        // shift the result to cover the -1 to 1 range.

        // Scale the result to [0, 2]
        result *= self.scale_factor;

        // Shift the result to [-1, 1]
        result - 1.0
    }
}

/// 4-dimensional `RidgedMulti` noise
impl<T> NoiseFn<f64, 4> for RidgedMulti<T>
where
    T: NoiseFn<f64, 4>,
{
    fn get(&self, point: [f64; 4]) -> f64 {
        let mut point = Vector4::from(point);

        let mut result = 0.0;
        let mut weight = 1.0;

        point *= self.frequency;

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point.into_array());

            // Make the ridges.
            signal = signal.abs();
            signal = 1.0 - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal *= signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal *= weight;

            // Weight successive contributions by the previous signal.
            weight = signal / self.attenuation;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            weight = weight.clamp(0.0, 1.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency.
            point *= self.lacunarity;
        }

        // The result before scaling will be 0 to something positive, so need to sale it back down
        // to -1 to 1. We don't know what the upper limit is, but it can be calculated based on the
        // number of octaves, and the persistence and attenuation values. By dividing the result by
        // what the upper limit should be / 2, we should get a value between 0 and 2. Then we can
        // shift the result to cover the -1 to 1 range.

        // Scale the result to [0, 2]
        result *= self.scale_factor;

        // Shift the result to [-1, 1]
        result - 1.0
    }
}
