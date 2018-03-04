use math::{self, scale_shift, Point2, Point3, Point4};
use noise_fns::{MultiFractal, NoiseFn, Perlin, Seedable};
use std;

/// Noise function that outputs "billowy" noise.
///
/// This noise function produces "billowy" noise suitable for clouds and rocks.
///
/// This noise function is nearly identical to fBm noise, except this noise
/// function modifies each octave with an absolute-value function. See the
/// documentation for fBm for more information.
#[derive(Clone, Debug)]
pub struct Billow {
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

impl Billow {
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_OCTAVE_COUNT: usize = 6;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;
    pub const DEFAULT_LACUNARITY: f64 = std::f64::consts::PI * 2.0 / 3.0;
    pub const DEFAULT_PERSISTENCE: f64 = 0.5;
    pub const MAX_OCTAVES: usize = 32;

    pub fn new() -> Self {
        Billow {
            seed: Self::DEFAULT_SEED,
            octaves: Self::DEFAULT_OCTAVE_COUNT,
            frequency: Self::DEFAULT_FREQUENCY,
            lacunarity: Self::DEFAULT_LACUNARITY,
            persistence: Self::DEFAULT_PERSISTENCE,
            sources: super::build_sources(Self::DEFAULT_SEED, Self::DEFAULT_OCTAVE_COUNT),
        }
    }
}

impl Default for Billow {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiFractal for Billow {
    fn set_octaves(self, mut octaves: usize) -> Self {
        if self.octaves == octaves {
            return self;
        }

        octaves = math::clamp(octaves, 1, Self::MAX_OCTAVES);
        Billow {
            octaves,
            sources: super::build_sources(self.seed, octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Billow { frequency, ..self }
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Billow { lacunarity, ..self }
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Billow {
            persistence,
            ..self
        }
    }
}

impl Seedable for Billow {
    fn set_seed(self, seed: u32) -> Self {
        if self.seed == seed {
            return self;
        }

        Billow {
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
impl NoiseFn<Point2<f64>> for Billow {
    fn get(&self, mut point: Point2<f64>) -> f64 {
        let mut result = 0.0;

        point = math::mul2(point, self.frequency);

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point);

            // Take the abs of the signal, then scale and shift back to
            // the [-1,1] range.
            signal = scale_shift(signal, 2.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency for the next octave.
            point = math::mul2(point, self.lacunarity);
        }

        // Scale the result to the [-1,1] range.
        result * 0.5
    }
}

/// 3-dimensional Billow noise
impl NoiseFn<Point3<f64>> for Billow {
    fn get(&self, mut point: Point3<f64>) -> f64 {
        let mut result = 0.0;

        point = math::mul3(point, self.frequency);

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point);

            // Take the abs of the signal, then scale and shift back to
            // the [-1,1] range.
            signal = scale_shift(signal, 2.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency for the next octave.
            point = math::mul3(point, self.lacunarity);
        }

        // Scale the result to the [-1,1] range.
        result * 0.5
    }
}

/// 4-dimensional Billow noise
impl NoiseFn<Point4<f64>> for Billow {
    fn get(&self, mut point: Point4<f64>) -> f64 {
        let mut result = 0.0;

        point = math::mul4(point, self.frequency);

        for x in 0..self.octaves {
            // Get the signal.
            let mut signal = self.sources[x].get(point);

            // Take the abs of the signal, then scale and shift back to
            // the [-1,1] range.
            signal = scale_shift(signal, 2.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the output value.
            result += signal;

            // Increase the frequency for the next octave.
            point = math::mul4(point, self.lacunarity);
        }

        // Scale the result to the [-1,1] range.
        result * 0.5
    }
}
