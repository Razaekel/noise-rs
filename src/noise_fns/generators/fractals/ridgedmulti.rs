// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math::{self, scale_shift, Point2, Point3, Point4};
use noise_fns::{MultiFractal, NoiseFn, Perlin, Seedable};

/// Default noise seed for the `RidgedMulti` noise function.
pub const DEFAULT_RIDGED_SEED: u32 = 0;
/// Default number of octaves for the `RidgedMulti` noise function.
pub const DEFAULT_RIDGED_OCTAVE_COUNT: usize = 6;
/// Default frequency for the `RidgedMulti` noise function.
pub const DEFAULT_RIDGED_FREQUENCY: f64 = 1.0;
/// Default lacunarity for the `RidgedMulti` noise function.
pub const DEFAULT_RIDGED_LACUNARITY: f64 = 2.0;
/// Default persistence for the `RidgedMulti` noise function.
pub const DEFAULT_RIDGED_PERSISTENCE: f64 = 1.0;
/// Default attenuation for the `RidgedMulti` noise function.
pub const DEFAULT_RIDGED_ATTENUATION: f64 = 2.0;
/// Maximum number of octaves for the `RidgedMulti` noise function.
pub const RIDGED_MAX_OCTAVES: usize = 32;

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
/// the [-1,1] range.
///
/// Ridged-multifractal noise is often used to generate craggy mountainous
/// terrain or marble-like textures.
#[derive(Clone, Debug)]
pub struct RidgedMulti {
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
    sources: Vec<Perlin>,
}

impl RidgedMulti {
    pub fn new() -> Self {
        RidgedMulti {
            seed: DEFAULT_RIDGED_SEED,
            octaves: DEFAULT_RIDGED_OCTAVE_COUNT,
            frequency: DEFAULT_RIDGED_FREQUENCY,
            lacunarity: DEFAULT_RIDGED_LACUNARITY,
            persistence: DEFAULT_RIDGED_PERSISTENCE,
            attenuation: DEFAULT_RIDGED_ATTENUATION,
            sources: super::build_sources(DEFAULT_RIDGED_SEED, DEFAULT_RIDGED_OCTAVE_COUNT),
        }
    }

    pub fn set_attenuation(self, attenuation: f64) -> Self {
        RidgedMulti {
            attenuation,
            ..self
        }
    }
}

impl Default for RidgedMulti {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiFractal for RidgedMulti {
    fn set_octaves(self, mut octaves: usize) -> Self {
        if self.octaves == octaves {
            return self;
        }

        octaves = math::clamp(octaves, 1, RIDGED_MAX_OCTAVES);
        RidgedMulti {
            octaves,
            sources: super::build_sources(self.seed, octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: f64) -> Self {
        RidgedMulti {
            frequency,
            ..self
        }
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        RidgedMulti {
            lacunarity,
            ..self
        }
    }

    fn set_persistence(self, persistence: f64) -> Self {
        RidgedMulti {
            persistence,
            ..self
        }
    }
}

impl Seedable for RidgedMulti {
    fn set_seed(self, seed: u32) -> Self {
        if self.seed == seed {
            return self;
        }

        RidgedMulti {
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
impl NoiseFn<Point2<f64>> for RidgedMulti {
    fn get(&self, mut point: Point2<f64>) -> f64 {
        let mut result = 0.0;
        let mut weight = 1.0;

        point = math::mul2(point, self.frequency);

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point);

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
            weight = math::clamp(weight, 0.0, 1.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency.
            point = math::mul2(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - 0.5_f64.powi(self.octaves as i32 - 1);
        scale_shift(result, 2.0 / scale)
    }
}

/// 3-dimensional `RidgedMulti` noise
impl NoiseFn<Point3<f64>> for RidgedMulti {
    fn get(&self, mut point: Point3<f64>) -> f64 {
        let mut result = 0.0;
        let mut weight = 1.0;

        point = math::mul3(point, self.frequency);

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point);

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
            weight = math::clamp(weight, 0.0, 1.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency.
            point = math::mul3(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - 0.5_f64.powi(self.octaves as i32 - 1);
        scale_shift(result, 2.0 / scale)
    }
}

/// 4-dimensional `RidgedMulti` noise
impl NoiseFn<Point4<f64>> for RidgedMulti {
    fn get(&self, mut point: Point4<f64>) -> f64 {
        let mut result = 0.0;
        let mut weight = 1.0;

        point = math::mul4(point, self.frequency);

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point);

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
            weight = signal * self.attenuation;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            weight = math::clamp(weight, 0.0, 1.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency.
            point = math::mul4(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - 0.5_f64.powi(self.octaves as i32 - 1);
        scale_shift(result, 2.0 / scale)
    }
}
