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
use modules::{MultiFractal, NoiseModule, Perlin, Seedable};

/// Default noise seed for the BasicMulti noise module.
pub const DEFAULT_BASICMULTI_SEED: usize = 0;
/// Default number of octaves for the BasicMulti noise module.
pub const DEFAULT_BASICMULTI_OCTAVES: usize = 6;
/// Default frequency for the BasicMulti noise module.
pub const DEFAULT_BASICMULTI_FREQUENCY: f32 = 2.0;
/// Default lacunarity for the BasicMulti noise module.
pub const DEFAULT_BASICMULTI_LACUNARITY: f32 = 2.0;
/// Default persistence for the BasicMulti noise module.
pub const DEFAULT_BASICMULTI_PERSISTENCE: f32 = 0.5;
/// Maximum number of octaves for the BasicMulti noise module.
pub const BASICMULTI_MAX_OCTAVES: usize = 32;

/// Noise module that outputs heterogenous Multifractal noise.
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

impl<T: Float> BasicMulti<T> {
    pub fn new() -> BasicMulti<T> {
        BasicMulti {
            seed: DEFAULT_BASICMULTI_SEED,
            octaves: DEFAULT_BASICMULTI_OCTAVES,
            frequency: math::cast(DEFAULT_BASICMULTI_FREQUENCY),
            lacunarity: math::cast(DEFAULT_BASICMULTI_LACUNARITY),
            persistence: math::cast(DEFAULT_BASICMULTI_PERSISTENCE),
            sources: super::build_sources(DEFAULT_BASICMULTI_SEED, DEFAULT_BASICMULTI_OCTAVES),
        }
    }
}

impl<T> MultiFractal<T> for BasicMulti<T> {
    fn set_octaves(self, mut octaves: usize) -> BasicMulti<T> {
        if self.octaves == octaves {
            return self;
        } else if octaves > BASICMULTI_MAX_OCTAVES {
            octaves = BASICMULTI_MAX_OCTAVES;
        } else if octaves < 1 {
            octaves = 1;
        }
        BasicMulti {
            octaves: octaves,
            sources: super::build_sources(self.seed, octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: T) -> BasicMulti<T> {
        BasicMulti { frequency: frequency, ..self }
    }

    fn set_lacunarity(self, lacunarity: T) -> BasicMulti<T> {
        BasicMulti { lacunarity: lacunarity, ..self }
    }

    fn set_persistence(self, persistence: T) -> BasicMulti<T> {
        BasicMulti { persistence: persistence, ..self }
    }
}

impl<T> Seedable for BasicMulti<T> {
    fn set_seed(self, seed: usize) -> BasicMulti<T> {
        if self.seed == seed {
            return self;
        }
        BasicMulti {
            seed: seed,
            sources: super::build_sources(seed, self.octaves),
            ..self
        }
    }
}

/// 2-dimensional BasicMulti noise
impl<T: Float> NoiseModule<Point2<T>> for BasicMulti<T> {
    type Output = T;

    fn get(&self, mut point: Point2<T>) -> T {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul2(point, self.frequency);
        let mut result = self.sources[0].get(point);

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Raise the spatial frequency.
            point = math::mul2(point, self.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Scale the signal by the current 'altitude' of the function.
            signal = signal * result;

            // Add signal to result.
            result = result + signal;
        }

        // Scale the result to the [-1,1] range.
        result * math::cast(0.5)
    }
}

/// 3-dimensional BasicMulti noise
impl<T: Float> NoiseModule<Point3<T>> for BasicMulti<T> {
    type Output = T;

    fn get(&self, mut point: Point3<T>) -> T {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul3(point, self.frequency);
        let mut result = self.sources[0].get(point);

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Raise the spatial frequency.
            point = math::mul3(point, self.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Scale the signal by the current 'altitude' of the function.
            signal = signal * result;

            // Add signal to result.
            result = result + signal;
        }

        // Scale the result to the [-1,1] range.
        result * math::cast(0.5)
    }
}

/// 4-dimensional BasicMulti noise
impl<T: Float> NoiseModule<Point4<T>> for BasicMulti<T> {
    type Output = T;

    fn get(&self, mut point: Point4<T>) -> T {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul4(point, self.frequency);
        let mut result = self.sources[0].get(point);

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Raise the spatial frequency.
            point = math::mul4(point, self.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Scale the signal by the current 'altitude' of the function.
            signal = signal * result;

            // Add signal to result.
            result = result + signal;
        }

        // Scale the result to the [-1,1] range.
        result * math::cast(0.5)
    }
}
