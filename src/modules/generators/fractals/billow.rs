// Copyright 2016 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num_traits::Float;
use math;
use math::{Point2, Point3, Point4};
use NoiseModule;
use modules::Perlin;

/// Default noise seed for the Billow noise module.
pub const DEFAULT_BILLOW_SEED: usize = 0;
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
    /// Seed.
    pub seed: usize,

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

    pub fn set_seed(self, seed: usize) -> Billow<T> {
        if self.seed == seed {
            return self;
        }
        Billow {
            seed: seed,
            sources: super::build_sources(seed, self.octaves),
            ..self
        }
    }

    pub fn set_octaves(self, mut octaves: usize) -> Billow<T> {
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

    pub fn set_frequency(self, frequency: T) -> Billow<T> {
        Billow { frequency: frequency, ..self }
    }

    pub fn set_lacunarity(self, lacunarity: T) -> Billow<T> {
        Billow { lacunarity: lacunarity, ..self }
    }

    pub fn set_persistence(self, persistence: T) -> Billow<T> {
        Billow { persistence: persistence, ..self }
    }
}

/// 2-dimensional Billow noise
impl<T: Float> NoiseModule<Point2<T>> for Billow<T> {
    type Output = T;

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
impl<T: Float> NoiseModule<Point3<T>> for Billow<T> {
    type Output = T;

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
impl<T: Float> NoiseModule<Point4<T>> for Billow<T> {
    type Output = T;

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
