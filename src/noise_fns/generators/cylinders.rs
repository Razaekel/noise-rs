use crate::{core::spheres::*, math::vectors::Vector2, noise_fns::NoiseFn};

/// Noise function that outputs concentric cylinders.
///
/// This noise function outputs concentric cylinders centered on the origin. The
/// cylinders are oriented along the z axis similar to the concentric rings of
/// a tree. Each cylinder extends infinitely along the z axis.
#[derive(Clone, Copy, Debug)]
pub struct Cylinders {
    /// Frequency of the concentric objects.
    pub frequency: f64,
}

impl Cylinders {
    pub const DEFAULT_FREQUENCY: f64 = 1.0;

    pub fn new() -> Self {
        Self {
            frequency: Self::DEFAULT_FREQUENCY,
        }
    }

    pub fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency }
    }
}

impl Default for Cylinders {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseFn<f64, 2> for Cylinders {
    fn get(&self, point: [f64; 2]) -> f64 {
        spheres_2d(point.into(), self.frequency)
    }
}

impl NoiseFn<f64, 3> for Cylinders {
    fn get(&self, point: [f64; 3]) -> f64 {
        spheres_2d(Vector2::new(point[0], point[1]), self.frequency)
    }
}

impl NoiseFn<f64, 4> for Cylinders {
    fn get(&self, point: [f64; 4]) -> f64 {
        spheres_2d(Vector2::new(point[0], point[1]), self.frequency)
    }
}
