use crate::noise_fns::NoiseFn;
use num_traits::{Float, MulAdd};

/// Noise function that applies a scaling factor and a bias to the output value
/// from the source function.
///
/// The function retrieves the output value from the source function, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
pub struct ScaleBias<F, Source, const DIM: usize> {
    /// Outputs a value.
    pub source: Source,

    /// Scaling factor to apply to the output value from the source function.
    /// The default value is 1.0.
    pub scale: F,

    /// Bias to apply to the scaled output value from the source function.
    /// The default value is 0.0.
    pub bias: F,
}

impl<F, Source, const DIM: usize> ScaleBias<F, Source, DIM>
where
    F: Float,
    Source: NoiseFn<F, DIM>,
{
    pub fn new(source: Source) -> Self {
        Self {
            source,
            scale: F::one(),
            bias: F::zero(),
        }
    }

    pub fn set_scale(self, scale: F) -> Self {
        Self { scale, ..self }
    }

    pub fn set_bias(self, bias: F) -> Self {
        Self { bias, ..self }
    }
}

impl<F, Source, const DIM: usize> NoiseFn<F, DIM> for ScaleBias<F, Source, DIM>
where
    F: Float + MulAdd<Output = F>,
    Source: NoiseFn<F, DIM>,
{
    #[cfg(not(target_os = "emscripten"))]
    fn get(&self, point: [F; DIM]) -> F {
        MulAdd::mul_add(self.source.get(point), self.scale, self.bias)
    }

    #[cfg(target_os = "emscripten")]
    fn get(&self, point: [F; DIM]) -> f64 {
        (self.source.get(point) * self.scale) + self.bias
    }
}
