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

/// Default noise seed for the Billow noise module.
pub const DEFAULT_BILLOW_SEED: u32 = 0;
/// Default number of octaves for the Billow noise module.
pub const DEFAULT_BILLOW_OCTAVE_COUNT: usize = 6;
/// Default frequency for the Billow noise module.
pub const DEFAULT_BILLOW_FREQUENCY: f32 = 1.0;
/// Default lacunarity for the Billow noise module.
pub const DEFAULT_BILLOW_LACUNARITY: f32 = 2.0;
/// Default persistence for the Billow noise module.
pub const DEFAULT_BILLOW_PERSISTENCE: f32 = 0.5;
/// Maximum number of octaves for the Billow noise module.
pub const BILLOW_MAX_OCTAVES: usize = 32;

/// Noise module that outputs "billowy" noise.
///
/// This noise module produces "billowy" noise suitable for clouds and rocks.
///
/// This noise module is nearly identical to fBm noise, except this noise
/// module modifes each octave with an absolute-value function. See the
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

    seed: u32,
    sources: Vec<Perlin>,
}

impl<T: Float> Billow<T> {
    pub fn new() -> Billow<T> {
        Billow {
            seed: DEFAULT_BILLOW_SEED,
            octaves: DEFAULT_BILLOW_OCTAVE_COUNT,
            frequency: math::cast(DEFAULT_BILLOW_FREQUENCY),
            lacunarity: math::cast(DEFAULT_BILLOW_LACUNARITY),
            persistence: math::cast(DEFAULT_BILLOW_PERSISTENCE),
            sources: super::build_sources(DEFAULT_BILLOW_SEED, DEFAULT_BILLOW_OCTAVE_COUNT),
        }
    }
}

impl<T> MultiFractal<T> for Billow<T> {
    fn set_octaves(self, mut octaves: usize) -> Billow<T> {
        if self.octaves == octaves {
            return self;
        } else if octaves > BILLOW_MAX_OCTAVES {
            octaves = BILLOW_MAX_OCTAVES;
        }
        Billow {
            octaves: octaves,
            sources: super::build_sources(self.seed, octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: T) -> Billow<T> {
        Billow { frequency: frequency, ..self }
    }

    fn set_lacunarity(self, lacunarity: T) -> Billow<T> {
        Billow { lacunarity: lacunarity, ..self }
    }

    fn set_persistence(self, persistence: T) -> Billow<T> {
        Billow { persistence: persistence, ..self }
    }
}

impl<T> Seedable for Billow<T> {
    fn set_seed(self, seed: u32) -> Billow<T> {
        if self.seed == seed {
            return self;
        }
        Billow {
            seed: seed,
            sources: super::build_sources(seed, self.octaves),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional Billow noise
impl<T: Float> NoiseModule<Point2<T>, T> for Billow<T> {
    fn get(&self, mut point: Point2<T>) -> T {
        let mut result = T::zero();

        point = math::mul2(point, self.frequency);

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point);

            // Take the abs of the signal, then scale and shift back to
            // the [-1,1] range.
            signal = signal.abs().mul_add(math::cast(2.0), -T::one());

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency for the next octave.
            point = math::mul2(point, self.lacunarity);
        }

        // Scale the result to the [-1,1] range.
        result * math::cast(0.5)
    }
}

/// 3-dimensional Billow noise
impl<T: Float> NoiseModule<Point3<T>, T> for Billow<T> {
    fn get(&self, mut point: Point3<T>) -> T {
        let mut result = T::zero();

        point = math::mul3(point, self.frequency);

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point);

            // Take the abs of the signal, then scale and shift back to
            // the [-1,1] range.
            signal = signal.abs().mul_add(math::cast(2.0), -T::one());

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency for the next octave.
            point = math::mul3(point, self.lacunarity);
        }

        // Scale the result to the [-1,1] range.
        result * math::cast(0.5)
    }
}

/// 4-dimensional Billow noise
impl<T: Float> NoiseModule<Point4<T>, T> for Billow<T> {
    fn get(&self, mut point: Point4<T>) -> T {
        let mut result = T::zero();

        point = math::mul4(point, self.frequency);

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point);

            // Take the abs of the signal, then scale and shift back to
            // the [-1,1] range.
            signal = signal.abs().mul_add(math::cast(2.0), -T::one());

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the output value.
            result = result + signal;

            // Increase the frequency for the next octave.
            point = math::mul4(point, self.lacunarity);
        }

        // Scale the result to the [-1,1] range.
        result * math::cast(0.5)
    }
}
