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

/// Default noise seed for the fBm noise module.
pub const DEFAULT_FBM_SEED: u32 = 0;
/// Default number of octaves for the fBm noise module.
pub const DEFAULT_FBM_OCTAVE_COUNT: usize = 6;
/// Default frequency for the fBm noise module.
pub const DEFAULT_FBM_FREQUENCY: f32 = 1.0;
/// Default lacunarity for the fBm noise module.
pub const DEFAULT_FBM_LACUNARITY: f32 = 2.0;
/// Default Hurst exponent for the fBm noise module
pub const DEFAULT_FBM_PERSISTENCE: f32 = 0.5;
/// Maximum number of octaves for the fBm noise module.
pub const FBM_MAX_OCTAVES: usize = 32;

/// Noise module that outputs fBm (fractal Brownian motion) noise.
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
pub struct Fbm<T> {
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

impl<T: Float> Fbm<T> {
    pub fn new() -> Fbm<T> {
        Fbm {
            seed: DEFAULT_FBM_SEED,
            octaves: DEFAULT_FBM_OCTAVE_COUNT,
            frequency: math::cast(DEFAULT_FBM_FREQUENCY),
            lacunarity: math::cast(DEFAULT_FBM_LACUNARITY),
            persistence: math::cast(DEFAULT_FBM_PERSISTENCE),
            sources: super::build_sources(DEFAULT_FBM_SEED, DEFAULT_FBM_OCTAVE_COUNT),
        }
    }
}

impl<T> MultiFractal<T> for Fbm<T> {
    fn set_octaves(self, mut octaves: usize) -> Fbm<T> {
        if self.octaves == octaves {
            return self;
        } else if octaves > FBM_MAX_OCTAVES {
            octaves = FBM_MAX_OCTAVES;
        } else if octaves < 1 {
            octaves = 1;
        }
        Fbm {
            octaves: octaves,
            sources: super::build_sources(self.seed, octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: T) -> Fbm<T> {
        Fbm {
            frequency: frequency,
            ..self
        }
    }

    fn set_lacunarity(self, lacunarity: T) -> Fbm<T> {
        Fbm {
            lacunarity: lacunarity,
            ..self
        }
    }

    fn set_persistence(self, persistence: T) -> Fbm<T> {
        Fbm {
            persistence: persistence,
            ..self
        }
    }
}

impl<T> Seedable for Fbm<T> {
    fn set_seed(self, seed: u32) -> Fbm<T> {
        if self.seed == seed {
            return self;
        }
        Fbm {
            seed: seed,
            sources: super::build_sources(seed, self.octaves),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional Fbm noise
impl<T: Float> NoiseModule<Point2<T>, T> for Fbm<T> {
    fn get(&self, mut point: Point2<T>) -> T {
        let mut result = T::zero();

        point = math::mul2(point, self.frequency);

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency for the next octave.
            point = math::mul2(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        let scale = T::from(2.0).unwrap() - self.persistence.powi(self.octaves as i32 - 1);
        result / math::cast(scale)
    }
}

/// 3-dimensional Fbm noise
impl<T: Float> NoiseModule<Point3<T>, T> for Fbm<T> {
    fn get(&self, mut point: Point3<T>) -> T {
        let mut result = T::zero();

        point = math::mul3(point, self.frequency);

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency for the next octave.
            point = math::mul3(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        let scale = T::from(2.0).unwrap() - self.persistence.powi(self.octaves as i32 - 1);
        result / math::cast(scale)
    }
}

/// 4-dimensional Fbm noise
impl<T: Float> NoiseModule<Point4<T>, T> for Fbm<T> {
    fn get(&self, mut point: Point4<T>) -> T {
        let mut result = T::zero();

        point = math::mul4(point, self.frequency);

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency for the next octave.
            point = math::mul4(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        let scale = T::from(2.0).unwrap() - self.persistence.powi(self.octaves as i32 - 1);
        result / math::cast(scale)
    }
}
