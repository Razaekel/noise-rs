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

use num_traits::Float;
use math;
use math::{Point2, Point3, Point4};
use NoiseModule;

/// Noise module that outputs concentric rings, cylinders, or spheres.
///
/// This noise module outputs concentric rings, cylinders, or spheres centered
/// on the origin. The cylinders are oriented along the z axis similar to the
/// concentric rings of a tree. Each cylinder extends infinitely along the y
/// axis.
#[derive(Clone, Copy, Debug)]
pub struct Cylinders<T: Float> {
    /// Frequency of the concentric objects.
    pub frequency: T,
}

impl<T: Float> Cylinders<T> {
    pub fn new(v: T) -> Cylinders<T> {
        Cylinders { frequency: v }
    }
}

impl<T: Float> NoiseModule<Point2<T>> for Cylinders<T> {
    type Output = T;

    fn get(&self, point: Point2<T>) -> Self::Output {
        let x = point[0] * self.frequency;

        let dist_from_smaller_sphere = x - x.floor();
        let dist_from_larger_sphere = T::one() - dist_from_smaller_sphere;
        let nearest_dist = dist_from_smaller_sphere.min(dist_from_larger_sphere);
        T::one() - (nearest_dist * math::cast(4.0))
    }
}

impl<T: Float> NoiseModule<Point3<T>> for Cylinders<T> {
    type Output = T;

    fn get(&self, point: Point3<T>) -> Self::Output {
        let x = point[0] * self.frequency;
        let y = point[1] * self.frequency;

        let dist_from_center = (x * x + y * y).sqrt();
        let dist_from_smaller_sphere = dist_from_center - dist_from_center.floor();
        let dist_from_larger_sphere = T::one() - dist_from_smaller_sphere;
        let nearest_dist = dist_from_smaller_sphere.min(dist_from_larger_sphere);
        T::one() - (nearest_dist * math::cast(4.0))
    }
}

impl<T: Float> NoiseModule<Point4<T>> for Cylinders<T> {
    type Output = T;

    fn get(&self, point: Point4<T>) -> Self::Output {
        let x = point[0] * self.frequency;
        let y = point[1] * self.frequency;
        let z = point[2] * self.frequency;

        let dist_from_center = (x * x + y * y + z * z).sqrt();
        let dist_from_smaller_sphere = dist_from_center - dist_from_center.floor();
        let dist_from_larger_sphere = T::one() - dist_from_smaller_sphere;
        let nearest_dist = dist_from_smaller_sphere.min(dist_from_larger_sphere);
        T::one() - (nearest_dist * math::cast(4.0))
    }
}
