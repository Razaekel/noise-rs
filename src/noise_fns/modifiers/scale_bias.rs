use crate::noise_fns::NoiseFn;
use rayon::prelude::*;

/// Noise function that applies a scaling factor and a bias to the output value
/// from the source function.
///
/// The function retrieves the output value from the source function, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
pub struct ScaleBias<'a, T: 'a> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T>,

    /// Scaling factor to apply to the output value from the source function.
    /// The default value is 1.0.
    pub scale: f64,

    /// Bias to apply to the scaled output value from the source function.
    /// The default value is 0.0.
    pub bias: f64,
}

impl<'a, T> ScaleBias<'a, T> {
    pub fn new(source: &'a dyn NoiseFn<T>) -> Self {
        Self {
            source,
            scale: 1.0,
            bias: 0.0,
        }
    }

    pub fn set_scale(self, scale: f64) -> Self {
        Self { scale, ..self }
    }

    pub fn set_bias(self, bias: f64) -> Self {
        Self { bias, ..self }
    }
}

impl<'a, T> NoiseFn<T> for ScaleBias<'a, T> {
    #[cfg(not(target_os = "emscripten"))]
    fn generate(&self, points: &[T]) -> Vec<f64> {
        let scale = self.scale;
        let bias = self.bias;

        self.source
            .generate(points)
            .par_iter()
            .map(|value| value.mul_add(scale, bias))
            .collect()
    }

    #[cfg(target_os = "emscripten")]
    fn generate(&self, points: &[T]) -> Vec<f64> {
        let scale = self.scale;
        let bias = self.bias;

        self.source
            .generate(points)
            .par_iter()
            .map(|value| (value * scale) + bias)
            .collect()
    }
}
