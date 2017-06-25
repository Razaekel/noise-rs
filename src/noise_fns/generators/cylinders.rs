// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math::{Point2, Point3, Point4};
use noise_fns::NoiseFn;

/// Default cylinders frequency
pub const DEFAULT_CYLINDERS_FREQUENCY: f64 = 1.0;

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
    pub fn new() -> Cylinders {
        Cylinders { frequency: DEFAULT_CYLINDERS_FREQUENCY }
    }

    pub fn set_frequency(self, frequency: f64) -> Cylinders {
        Cylinders { frequency: frequency }
    }
}

impl Default for Cylinders {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseFn<Point2<f64>> for Cylinders {
    fn get(&self, point: Point2<f64>) -> f64 {
        calculate_cylinders(&point, self.frequency)
    }
}

impl NoiseFn<Point3<f64>> for Cylinders {
    fn get(&self, point: Point3<f64>) -> f64 {
        calculate_cylinders(&point, self.frequency)
    }
}

impl NoiseFn<Point4<f64>> for Cylinders {
    fn get(&self, point: Point4<f64>) -> f64 {
        calculate_cylinders(&point, self.frequency)
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
