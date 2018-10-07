use math;
use math::{Point2, Point3, Point4};
use noise_fns::{FractalParams, RandomFractal, NoiseFn, Random, Perlin, default_rng};
use std;
use rand::Rng;

/// Noise function that outputs heterogenous Multifractal noise.
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
pub struct BasicMulti {
    params: FractalParams,
    sources: Vec<Perlin>,
}

impl BasicMulti {
    pub const DEFAULT_PARAMS: FractalParams = FractalParams {
        frequency: 2.0,
        lacunarity: std::f64::consts::PI * 2.0 / 3.0,
        persistence: 0.5,
    };
}

impl RandomFractal for BasicMulti {
    fn from_rng<R: Rng + ?Sized>(rng: &mut R, octaves: usize, params: FractalParams) -> Self {
        let sources = (0..octaves).map(|_| Perlin::from_rng(rng)).collect();
        BasicMulti {
            params,
            sources,
        }
    }
}

impl Default for BasicMulti {
    fn default() -> Self { Self::from_rng(&mut default_rng(), 6, Self::DEFAULT_PARAMS) }
}

/// 2-dimensional `BasicMulti` noise
impl NoiseFn<Point2<f64>> for BasicMulti {
    fn get(&self, mut point: Point2<f64>) -> f64 {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul2(point, self.params.frequency);
        let mut result = self.sources[0].get(point);

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.sources.len() {
            // Raise the spatial frequency.
            point = math::mul2(point, self.params.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.params.persistence.powi(x as i32);

            // Scale the signal by the current 'altitude' of the function.
            signal *= result;

            // Add signal to result.
            result += signal;
        }

        // Scale the result to the [-1,1] range.
        result * 0.5
    }
}

/// 3-dimensional `BasicMulti` noise
impl NoiseFn<Point3<f64>> for BasicMulti {
    fn get(&self, mut point: Point3<f64>) -> f64 {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul3(point, self.params.frequency);
        let mut result = self.sources[0].get(point);

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.sources.len() {
            // Raise the spatial frequency.
            point = math::mul3(point, self.params.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.params.persistence.powi(x as i32);

            // Scale the signal by the current 'altitude' of the function.
            signal *= result;

            // Add signal to result.
            result += signal;
        }

        // Scale the result to the [-1,1] range.
        result * 0.5
    }
}

/// 4-dimensional `BasicMulti` noise
impl NoiseFn<Point4<f64>> for BasicMulti {
    fn get(&self, mut point: Point4<f64>) -> f64 {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul4(point, self.params.frequency);
        let mut result = self.sources[0].get(point);

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.sources.len() {
            // Raise the spatial frequency.
            point = math::mul4(point, self.params.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.params.persistence.powi(x as i32);

            // Scale the signal by the current 'altitude' of the function.
            signal *= result;

            // Add signal to result.
            result += signal;
        }

        // Scale the result to the [-1,1] range.
        result * 0.5
    }
}
