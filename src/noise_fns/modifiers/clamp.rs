use math;
use noise_fns::NoiseFn;

/// Noise function that clamps the output value from the source function to a
/// range of values.
pub struct Clamp<'a, T: 'a> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T>,

    /// Bound of the clamping range. Default is -1.0 to 1.0.
    pub bounds: (f64, f64),
}

impl<'a, T> Clamp<'a, T> {
    pub fn new(source: &'a dyn NoiseFn<T>) -> Self {
        Clamp {
            source,
            bounds: (-1.0, 1.0),
        }
    }

    pub fn set_lower_bound(self, lower_bound: f64) -> Self {
        Clamp {
            bounds: (lower_bound, self.bounds.1),
            ..self
        }
    }

    pub fn set_upper_bound(self, upper_bound: f64) -> Self {
        Clamp {
            bounds: (self.bounds.0, upper_bound),
            ..self
        }
    }

    pub fn set_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        Clamp {
            bounds: (lower_bound, upper_bound),
            ..self
        }
    }
}

impl<'a, T> NoiseFn<T> for Clamp<'a, T> {
    fn get(&self, point: T) -> f64 {
        let value = self.source.get(point);

        math::clamp(value, self.bounds.0, self.bounds.1)
    }
}
