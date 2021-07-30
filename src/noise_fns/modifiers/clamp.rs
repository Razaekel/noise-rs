use crate::noise_fns::NoiseFn;
use num_traits::Float;

/// Noise function that clamps the output value from the source function to a
/// range of values.
pub struct Clamp<F, Source, const DIM: usize>
where
    Source: NoiseFn<F, DIM>,
{
    /// Outputs a value.
    pub source: Source,

    /// Bound of the clamping range. Default is -1.0 to 1.0.
    pub bounds: (F, F),
}

impl<F, Source, const DIM: usize> Clamp<F, Source, DIM>
where
    F: Float,
    Source: NoiseFn<F, DIM>,
{
    pub fn new(source: Source) -> Self {
        Self {
            source,
            bounds: (-F::one(), F::one()),
        }
    }

    pub fn set_lower_bound(self, lower_bound: F) -> Self {
        assert!(lower_bound <= self.bounds.1);

        Self {
            bounds: (lower_bound, self.bounds.1),
            ..self
        }
    }

    pub fn set_upper_bound(self, upper_bound: F) -> Self {
        assert!(self.bounds.0 <= upper_bound);

        Self {
            bounds: (self.bounds.0, upper_bound),
            ..self
        }
    }

    pub fn set_bounds(self, lower_bound: F, upper_bound: F) -> Self {
        assert!(lower_bound <= upper_bound);

        Self {
            bounds: (lower_bound, upper_bound),
            ..self
        }
    }
}

impl<F, Source, const DIM: usize> NoiseFn<F, DIM> for Clamp<F, Source, DIM>
where
    F: Float,
    Source: NoiseFn<F, DIM>,
{
    fn get(&self, point: [F; DIM]) -> F {
        let value = self.source.get(point);

        if value < self.bounds.0 {
            self.bounds.0
        } else if value > self.bounds.1 {
            self.bounds.1
        } else {
            value
        }
    }
}
