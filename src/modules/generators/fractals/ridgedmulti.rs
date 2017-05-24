// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math;
use math::{Point2, Point3, Point4};
use modules::{MultiFractal, NoiseModule, Perlin, Seedable};
use num_traits::Float;

/// Default noise seed for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_SEED: u32 = 0;
/// Default number of octaves for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_OCTAVE_COUNT: usize = 6;
/// Default frequency for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_FREQUENCY: f32 = 1.0;
/// Default lacunarity for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_LACUNARITY: f32 = 2.0;
/// Default persistence for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_PERSISTENCE: f32 = 1.0;
/// Default attenuation for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_ATTENUATION: f32 = 2.0;
/// Maximum number of octaves for the RidgedMulti noise module.
pub const RIDGED_MAX_OCTAVES: usize = 32;

/// Noise module that outputs ridged-multifractal noise.
///
/// This noise module, heavily based on the fBm-noise module, generates
/// ridged-multifractal noise. Ridged-multifractal noise is generated in much
/// the same way as fBm noise, except the output of each octave is modified by
/// an absolute-value function. Modifying the octave values in this way
/// produces ridge-like formations.
///
/// The values output from this module will usually range from -1.0 to 1.0 with
/// default values for the parameters, but there are no guarantees that all
/// output values will exist within this range. If the parameters are modified
/// from their defaults, then the output will need to be scaled to remain in
/// the [-1,1] range.
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
    pub frequency: T,

    /// A multiplier that determines how quickly the frequency increases for
    /// each successive octave in the noise function.
    ///
    /// The frequency of each successive octave is equal to the product of the
    /// previous octave's frequency and the lacunarity value.
    ///
    /// A lacunarity of 2.0 results in the frequency doubling every octave. For
    /// almost all cases, 2.0 is a good value to use.
    pub lacunarity: T,

    /// A multiplier that determines how quickly the amplitudes diminish for
    /// each successive octave in the noise function.
    ///
    /// The amplitude of each successive octave is equal to the product of the
    /// previous octave's amplitude and the persistence value. Increasing the
    /// persistence produces "rougher" noise.
    pub persistence: T,

    /// The attenuation to apply to the weight on each octave. This reduces
    /// the strength of each successive octave, making their respective
    /// ridges smaller. The default attenuation is 2.0, making each octave
    /// half the height of the previous.
    pub attenuation: T,

    seed: u32,
    sources: Vec<Perlin>,
}

impl<T: Float> RidgedMulti<T> {
    pub fn new() -> RidgedMulti<T> {
        RidgedMulti {
            seed: DEFAULT_RIDGED_SEED,
            octaves: DEFAULT_RIDGED_OCTAVE_COUNT,
            frequency: math::cast(DEFAULT_RIDGED_FREQUENCY),
            lacunarity: math::cast(DEFAULT_RIDGED_LACUNARITY),
            persistence: math::cast(DEFAULT_RIDGED_PERSISTENCE),
            attenuation: math::cast(DEFAULT_RIDGED_ATTENUATION),
            sources: super::build_sources(DEFAULT_RIDGED_SEED, DEFAULT_RIDGED_OCTAVE_COUNT),
        }
    }

    pub fn set_attenuation(self, attenuation: T) -> RidgedMulti<T> {
        RidgedMulti { attenuation: attenuation, ..self }
    }
}

impl<T> MultiFractal<T> for RidgedMulti<T> {
    fn set_octaves(self, mut octaves: usize) -> RidgedMulti<T> {
        if self.octaves == octaves {
            return self;
        } else if octaves > RIDGED_MAX_OCTAVES {
            octaves = RIDGED_MAX_OCTAVES;
        } else if octaves < 1 {
            octaves = 1;
        }
        RidgedMulti {
            octaves: octaves,
            sources: super::build_sources(self.seed, octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: T) -> RidgedMulti<T> {
        RidgedMulti { frequency: frequency, ..self }
    }

    fn set_lacunarity(self, lacunarity: T) -> RidgedMulti<T> {
        RidgedMulti { lacunarity: lacunarity, ..self }
    }

    fn set_persistence(self, persistence: T) -> RidgedMulti<T> {
        RidgedMulti { persistence: persistence, ..self }
    }
}

impl<T> Seedable for RidgedMulti<T> {
    fn set_seed(self, seed: u32) -> RidgedMulti<T> {
        if self.seed == seed {
            return self;
        }
        RidgedMulti {
            seed: seed,
            sources: super::build_sources(seed, self.octaves),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional RidgedMulti noise
impl<T: Float> NoiseModule<Point2<T>, T> for RidgedMulti<T> {
    fn get(&self, mut point: Point2<T>) -> T {
        let mut result = T::zero();
        let mut weight = T::one();

        point = math::mul2(point, self.frequency);

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point);

            // Make the ridges.
            signal = signal.abs();
            signal = T::one() - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal = signal * signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal = signal * weight;

            // Weight successive contributions by the previous signal.
            weight = signal / self.attenuation;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            if math::cast::<_, f32>(weight) > 1.0 {
                weight = T::one();
            } else if math::cast::<_, f32>(weight) < 0.0 {
                weight = T::zero();
            }

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency.
            point = math::mul2(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - 0.5.powi(self.octaves as i32 - 1);
        result.mul_add(math::cast(2.0 / scale), -T::one())
    }
}

/// 3-dimensional RidgedMulti noise
impl<T: Float> NoiseModule<Point3<T>, T> for RidgedMulti<T> {
    fn get(&self, mut point: Point3<T>) -> T {
        let mut result = T::zero();
        let mut weight = T::one();

        point = math::mul3(point, self.frequency);

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point);

            // Make the ridges.
            signal = signal.abs();
            signal = T::one() - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal = signal * signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal = signal * weight;

            // Weight successive contributions by the previous signal.
            weight = signal / self.attenuation;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            if math::cast::<_, f32>(weight) > 1.0 {
                weight = T::one();
            } else if math::cast::<_, f32>(weight) < 0.0 {
                weight = T::zero();
            }

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency.
            point = math::mul3(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - 0.5.powi(self.octaves as i32 - 1);
        result.mul_add(math::cast(2.0 / scale), -T::one())
    }
}

/// 4-dimensional RidgedMulti noise
impl<T: Float> NoiseModule<Point4<T>, T> for RidgedMulti<T> {
    fn get(&self, mut point: Point4<T>) -> T {
        let mut result = T::zero();
        let mut weight = T::one();

        point = math::mul4(point, self.frequency);

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point);

            // Make the ridges.
            signal = signal.abs();
            signal = T::one() - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal = signal * signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal = signal * weight;

            // Weight successive contributions by the previous signal.
            weight = signal * self.attenuation;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            if math::cast::<_, f32>(weight) > 1.0 {
                weight = T::one();
            } else if math::cast::<_, f32>(weight) < 0.0 {
                weight = T::zero();
            }

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency.
            point = math::mul4(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - 0.5.powi(self.octaves as i32 - 1);
        result.mul_add(math::cast(2.0 / scale), -T::one())
    }
}
