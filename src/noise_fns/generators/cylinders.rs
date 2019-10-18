use crate::noise_fns::NoiseFn;
use rayon::prelude::*;

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

impl NoiseFn<[f64; 2]> for Cylinders {
    fn generate(&self, points: &[[f64; 2]]) -> Vec<f64> {
        points
            .par_iter()
            .map(|point| calculate_cylinders(point, self.frequency))
            .collect()
    }
}

impl NoiseFn<[f64; 3]> for Cylinders {
    fn generate(&self, points: &[[f64; 3]]) -> Vec<f64> {
        points
            .par_iter()
            .map(|point| calculate_cylinders(point, self.frequency))
            .collect()
    }
}

impl NoiseFn<[f64; 4]> for Cylinders {
    fn generate(&self, points: &[[f64; 4]]) -> Vec<f64> {
        points
            .par_iter()
            .map(|point| calculate_cylinders(point, self.frequency))
            .collect()
    }
}

fn calculate_cylinders(point: &[f64], frequency: f64) -> f64 {
    // Scale the inputs by the frequency.
    let x = point[0] * frequency;
    let y = point[1] * frequency;

    // Calculate the distance of the point from the origin.
    let dist_from_center = (x.powi(2) + y.powi(2)).sqrt();

    let dist_from_smaller_sphere = dist_from_center - dist_from_center.floor();
    let dist_from_larger_sphere = 1.0 - dist_from_smaller_sphere;
    let nearest_dist = dist_from_smaller_sphere.min(dist_from_larger_sphere);

    // Shift the result to be in the -1.0 to +1.0 range.
    1.0 - (nearest_dist * 4.0)
}
