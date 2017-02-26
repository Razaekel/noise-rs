// Copyright 2015 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use math;
use math::{Point2, Point3, Point4};
use modules::NoiseModule;
use num_traits::Float;

/// Default cylinders frequency
pub const DEFAULT_CYLINDERS_FREQUENCY: f32 = 1.0;

/// Noise module that outputs concentric cylinders.
///
/// This noise module outputs concentric cylinders centered on the origin. The
/// cylinders are oriented along the z axis similar to the concentric rings of
/// a tree. Each cylinder extends infinitely along the z axis.
#[derive(Clone, Copy, Debug)]
pub struct Cylinders<T: Float> {
    /// Frequency of the concentric objects.
    pub frequency: T,
}

impl<T: Float> Cylinders<T> {
    pub fn new() -> Cylinders<T> {
        Cylinders { frequency: math::cast(DEFAULT_CYLINDERS_FREQUENCY) }
    }

    pub fn set_frequency(self, frequency: T) -> Cylinders<T> {
        Cylinders { frequency: frequency }
    }
}

impl<T: Float> NoiseModule<Point2<T>> for Cylinders<T> {
    type Output = T;

    fn get(&self, point: Point2<T>) -> Self::Output {
        calculate_cylinders(&point, self.frequency)
    }
}

impl<T: Float> NoiseModule<Point3<T>> for Cylinders<T> {
    type Output = T;

    fn get(&self, point: Point3<T>) -> Self::Output {
        calculate_cylinders(&point, self.frequency)
    }
}

impl<T: Float> NoiseModule<Point4<T>> for Cylinders<T> {
    type Output = T;

    fn get(&self, point: Point4<T>) -> Self::Output {
        calculate_cylinders(&point, self.frequency)
    }
}

fn calculate_cylinders<T: Float>(point: &[T], frequency: T) -> T {

    // Scale the inputs by the frequency.
    let x = point[0] * frequency;
    let y = point[1] * frequency;

    // Calculate the distance of the point from the origin.
    let dist_from_center = (x.powi(2) + y.powi(2)).sqrt();

    let dist_from_smaller_sphere = dist_from_center - dist_from_center.floor();
    let dist_from_larger_sphere = T::one() - dist_from_smaller_sphere;
    let nearest_dist = dist_from_smaller_sphere.min(dist_from_larger_sphere);

    // Shift the result to be in the -1.0 to +1.0 range.
    T::one() - (nearest_dist * math::cast(4.0))
}
