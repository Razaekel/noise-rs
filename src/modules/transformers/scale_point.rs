// Copyright 2016 The Noise-rs Developers.
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

use math::{Point2, Point3, Point4};
use modules::NoiseModule;
use num_traits::Float;

/// Noise Module that scales the coordinates of the input value before
/// returning the output value from the source module.
///
/// The get() method multiplies the coordinates of the input value with a
/// scaling factor before returning the output value from the source module.
pub struct ScalePoint<Source, T> {
    /// Source Module that outputs a value
    pub source: Source,

    /// Scaling factor applied to the _x_ coordinate of the input value. The
    /// default scaling factor is set to 1.0.
    pub x_scale: T,

    /// Scaling factor applied to the _y_ coordinate of the input value. The
    /// default scaling factor is set to 1.0.
    pub y_scale: T,

    /// Scaling factor applied to the _z_ coordinate of the input value. The
    /// default scaling factor is set to 1.0.
    pub z_scale: T,

    /// Scaling factor applied to the _u_ coordinate of the input value. The
    /// default scaling factor is set to 1.0.
    pub u_scale: T,
}

impl<Source, T> ScalePoint<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> ScalePoint<Source, T> {
        ScalePoint {
            source: source,
            x_scale: T::one(),
            y_scale: T::one(),
            z_scale: T::one(),
            u_scale: T::one(),
        }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_x_scale(self, x_scale: T) -> ScalePoint<Source, T> {
        ScalePoint { x_scale: x_scale, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_y_scale(self, y_scale: T) -> ScalePoint<Source, T> {
        ScalePoint { y_scale: y_scale, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_z_scale(self, z_scale: T) -> ScalePoint<Source, T> {
        ScalePoint { z_scale: z_scale, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_u_scale(self, u_scale: T) -> ScalePoint<Source, T> {
        ScalePoint { u_scale: u_scale, ..self }
    }

    /// Sets the scaling factor to apply to all coordinates of the input value.
    pub fn set_scale(self, scale: T) -> ScalePoint<Source, T> {
        ScalePoint {
            x_scale: scale,
            y_scale: scale,
            z_scale: scale,
            u_scale: scale,
            ..self
        }
    }

    /// Sets the individual scaling factors to apply to each coordinate of the
    /// input value.
    pub fn set_all_scales(self,
                          x_scale: T,
                          y_scale: T,
                          z_scale: T,
                          u_scale: T)
                          -> ScalePoint<Source, T> {
        ScalePoint {
            x_scale: x_scale,
            y_scale: y_scale,
            z_scale: z_scale,
            u_scale: u_scale,
            ..self
        }
    }
}

impl<Source, T> NoiseModule<Point2<T>> for ScalePoint<Source, T>
    where Source: NoiseModule<Point2<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point2<T>) -> Self::Output {
        self.source.get([point[0] * self.x_scale, point[1] * self.y_scale])
    }
}

impl<Source, T> NoiseModule<Point3<T>> for ScalePoint<Source, T>
    where Source: NoiseModule<Point3<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point3<T>) -> Self::Output {

        self.source.get([point[0] * self.x_scale, point[1] * self.y_scale, point[2] * self.z_scale])
    }
}

impl<Source, T> NoiseModule<Point4<T>> for ScalePoint<Source, T>
    where Source: NoiseModule<Point4<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point4<T>) -> Self::Output {
        self.source.get([point[0] * self.x_scale,
                         point[1] * self.y_scale,
                         point[2] * self.z_scale,
                         point[3] * self.u_scale])
    }
}
