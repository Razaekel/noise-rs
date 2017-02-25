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

use num_traits::Float;
use math;
use math::{Point2, Point3, Point4};
use std::f64::consts::PI;
use modules::NoiseModule;

/// Noise Module that rotates the input value around the origin before
/// returning the output value from the source module.
///
/// The get() method rotates the coordinates of the input value around the
/// origin before returning the output value from the source module.
///
/// The coordinate system of the input value is assumed to be "right-handed"
/// (_x_ increases to the right, _y_ increases upward, and _z_ increases inward).
pub struct RotatePoint<Source, T> {
    /// Source Module that outputs a value
    pub source: Source,

    /// _x_ rotation angle applied to the input value, in degrees. The
    /// default angle is set to 0.0 degrees.
    pub x_angle: T,

    /// _y_ rotation angle applied to the input value, in degrees. The
    /// default angle is set to 0.0 degrees.
    pub y_angle: T,

    /// _z_ rotation angle applied to the input value, in degrees. The
    /// default angle is set to 0.0 degrees.
    pub z_angle: T,

    /// _u_ rotation angle applied to the input value, in degrees. The
    /// default angle is set to 0.0 degrees.
    pub u_angle: T,
}

impl<Source, T> RotatePoint<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> RotatePoint<Source, T> {
        RotatePoint {
            source: source,
            x_angle: T::zero(),
            y_angle: T::zero(),
            z_angle: T::zero(),
            u_angle: T::zero(),
        }
    }

    /// Sets the rotation angle around the _x_ axis to apply to the input
    /// value.
    pub fn set_x_angle(self, x_angle: T) -> RotatePoint<Source, T> {
        RotatePoint { x_angle: x_angle, ..self }
    }

    /// Sets the rotation angle around the _y_ axis to apply to the input
    /// value.
    pub fn set_y_angle(self, y_angle: T) -> RotatePoint<Source, T> {
        RotatePoint { y_angle: y_angle, ..self }
    }

    /// Sets the rotation angle around the _z_ axis to apply to the input
    /// value.
    pub fn set_z_angle(self, z_angle: T) -> RotatePoint<Source, T> {
        RotatePoint { z_angle: z_angle, ..self }
    }

    /// Sets the rotation angle around the _u_ axis to apply to the input
    /// value.
    pub fn set_u_angle(self, u_angle: T) -> RotatePoint<Source, T> {
        RotatePoint { u_angle: u_angle, ..self }
    }

    /// Sets the rotation angles around all of the axes to apply to the input
    /// value.
    pub fn set_angles(self,
                      x_angle: T,
                      y_angle: T,
                      z_angle: T,
                      u_angle: T)
                      -> RotatePoint<Source, T> {
        RotatePoint {
            x_angle: x_angle,
            y_angle: y_angle,
            z_angle: z_angle,
            u_angle: u_angle,
            ..self
        }
    }
}

impl<Source, T> NoiseModule<Point2<T>> for RotatePoint<Source, T>
    where Source: NoiseModule<Point2<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point2<T>) -> Self::Output {
        // In two dimensions, the plane is _xy_, and we rotate around the
        // z-axis.
        let x = point[0];
        let y = point[1];
        let theta = deg_to_rad(self.z_angle);

        let x2 = x * theta.cos() - y * theta.sin();
        let y2 = x * theta.sin() + y * theta.cos();

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.get([x2, y2])
    }
}

impl<Source, T> NoiseModule<Point3<T>> for RotatePoint<Source, T>
    where Source: NoiseModule<Point3<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point3<T>) -> Self::Output {
        // In three dimensions, we could rotate around any of the x, y, or z
        // axes. Need a more complicated function to handle this case.
        let x_cos = deg_to_rad(self.x_angle).cos();
        let y_cos = deg_to_rad(self.y_angle).cos();
        let z_cos = deg_to_rad(self.z_angle).cos();
        let x_sin = deg_to_rad(self.x_angle).sin();
        let y_sin = deg_to_rad(self.y_angle).sin();
        let z_sin = deg_to_rad(self.z_angle).sin();

        let x1 = x_sin * y_sin * z_sin + y_cos * z_cos;
        let y1 = x_cos * z_sin;
        let z1 = y_sin * z_cos - y_cos * x_sin * z_sin;
        let x2 = y_sin * x_sin * z_cos - y_cos * z_sin;
        let y2 = x_cos * z_cos;
        let z2 = -y_cos * x_sin * z_cos - y_sin * z_sin;
        let x3 = -y_sin * x_cos;
        let y3 = x_sin;
        let z3 = y_cos * x_cos;

        let x = (x1 * point[0]) + (y1 * point[1]) + (z1 * point[2]);
        let y = (x2 * point[0]) + (y2 * point[1]) + (z2 * point[2]);
        let z = (x3 * point[0]) + (y3 * point[1]) + (z3 * point[2]);

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.get([x, y, z])
    }
}

impl<Source, T> NoiseModule<Point4<T>> for RotatePoint<Source, T>
    where Source: NoiseModule<Point4<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, _point: Point4<T>) -> Self::Output {
        // 4d rotations are hard.
        panic!("4d not implemented!");
    }
}

fn deg_to_rad<T: Float>(x: T) -> T {
    (x / math::cast(180.0)) * math::cast(PI)
}
