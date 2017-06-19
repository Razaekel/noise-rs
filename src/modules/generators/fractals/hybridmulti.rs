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

/// Default noise seed for the BasicMulti noise module.
pub const DEFAULT_HYBRIDMULTI_SEED: u32 = 0;
/// Default number of octaves for the BasicMulti noise module.
pub const DEFAULT_HYBRIDMULTI_OCTAVES: usize = 6;
/// Default frequency for the BasicMulti noise module.
pub const DEFAULT_HYBRIDMULTI_FREQUENCY: f32 = 2.0;
/// Default lacunarity for the BasicMulti noise module.
pub const DEFAULT_HYBRIDMULTI_LACUNARITY: f32 = 2.0;
/// Default persistence for the BasicMulti noise module.
pub const DEFAULT_HYBRIDMULTI_PERSISTENCE: f32 = 0.25;
/// Maximum number of octaves for the BasicMulti noise module.
pub const HYBRIDMULTI_MAX_OCTAVES: usize = 32;

/// Noise module that outputs hybrid Multifractal noise.
///
/// The result of this multifractal noise is that valleys in the noise should
/// have smooth bottoms at all altitudes.
#[derive(Clone, Debug)]
pub struct HybridMulti<T> {
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

impl<T: Float> HybridMulti<T> {
    pub fn new() -> HybridMulti<T> {
        HybridMulti {
            seed: DEFAULT_HYBRIDMULTI_SEED,
            octaves: DEFAULT_HYBRIDMULTI_OCTAVES,
            frequency: math::cast(DEFAULT_HYBRIDMULTI_FREQUENCY),
            lacunarity: math::cast(DEFAULT_HYBRIDMULTI_LACUNARITY),
            persistence: math::cast(DEFAULT_HYBRIDMULTI_PERSISTENCE),
            sources: super::build_sources(DEFAULT_HYBRIDMULTI_SEED, DEFAULT_HYBRIDMULTI_OCTAVES),
        }
    }
}

impl<T> MultiFractal<T> for HybridMulti<T> {
    fn set_octaves(self, mut octaves: usize) -> HybridMulti<T> {
        if self.octaves == octaves {
            return self;
        } else if octaves > HYBRIDMULTI_MAX_OCTAVES {
            octaves = HYBRIDMULTI_MAX_OCTAVES;
        } else if octaves < 1 {
            octaves = 1;
        }
        HybridMulti {
            octaves: octaves,
            sources: super::build_sources(self.seed, octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: T) -> HybridMulti<T> {
        HybridMulti {
            frequency: frequency,
            ..self
        }
    }

    fn set_lacunarity(self, lacunarity: T) -> HybridMulti<T> {
        HybridMulti {
            lacunarity: lacunarity,
            ..self
        }
    }

    fn set_persistence(self, persistence: T) -> HybridMulti<T> {
        HybridMulti {
            persistence: persistence,
            ..self
        }
    }
}

impl<T> Seedable for HybridMulti<T> {
    fn set_seed(self, seed: u32) -> HybridMulti<T> {
        if self.seed == seed {
            return self;
        }
        HybridMulti {
            seed: seed,
            sources: super::build_sources(seed, self.octaves),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional HybridMulti noise
impl<T: Float> NoiseModule<Point2<T>, T> for HybridMulti<T> {
    fn get(&self, mut point: Point2<T>) -> T {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul2(point, self.frequency);
        let mut result = self.sources[0].get(point) * self.persistence;
        let mut weight = result;

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Prevent divergence.
            if weight > T::one() {
                weight = T::one();
            }

            // Raise the spatial frequency.
            point = math::mul2(point, self.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add it in, weighted by previous octave's noise value.
            result = result + (weight * signal);

            // Update the weighting value.
            weight = weight * signal;
        }

        // Scale the result to the [-1,1] range
        result * math::cast(3.0)
    }
}

/// 3-dimensional HybridMulti noise
impl<T: Float> NoiseModule<Point3<T>, T> for HybridMulti<T> {
    fn get(&self, mut point: Point3<T>) -> T {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul3(point, self.frequency);
        let mut result = self.sources[0].get(point) * self.persistence;
        let mut weight = result;

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Prevent divergence.
            if weight > T::one() {
                weight = T::one();
            }

            // Raise the spatial frequency.
            point = math::mul3(point, self.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add it in, weighted by previous octave's noise value.
            result = result + (weight * signal);

            // Update the weighting value.
            weight = weight * signal;
        }

        // Scale the result to the [-1,1] range
        result * math::cast(3.0)
    }
}

/// 4-dimensional HybridMulti noise
impl<T: Float> NoiseModule<Point4<T>, T> for HybridMulti<T> {
    fn get(&self, mut point: Point4<T>) -> T {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul4(point, self.frequency);
        let mut result = self.sources[0].get(point) * self.persistence;
        let mut weight = result;

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Prevent divergence.
            if weight > T::one() {
                weight = T::one();
            }

            // Raise the spatial frequency.
            point = math::mul4(point, self.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add it in, weighted by previous octave's noise value.
            result = result + (weight * signal);

            // Update the weighting value.
            weight = weight * signal;
        }

        // Scale the result to the [-1,1] range
        result * math::cast(3.0)
    }
}
