// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math::{Point2, Point3, Point4};
use noise_fns::NoiseFn;

/// Noise function that rotates the input value around the origin before
/// returning the output value from the source function.
///
/// The get() method rotates the coordinates of the input value around the
/// origin before returning the output value from the source function.
///
/// The coordinate system of the input value is assumed to be "right-handed"
/// (_x_ increases to the right, _y_ increases upward, and _z_ increases inward).
pub struct RotatePoint<Source> {
    /// Source function that outputs a value
    pub source: Source,

    /// _x_ rotation angle applied to the input value, in degrees. The
    /// default angle is set to 0.0 degrees.
    pub x_angle: f64,

    /// _y_ rotation angle applied to the input value, in degrees. The
    /// default angle is set to 0.0 degrees.
    pub y_angle: f64,

    /// _z_ rotation angle applied to the input value, in degrees. The
    /// default angle is set to 0.0 degrees.
    pub z_angle: f64,

    /// _u_ rotation angle applied to the input value, in degrees. The
    /// default angle is set to 0.0 degrees.
    pub u_angle: f64,
}

impl<Source> RotatePoint<Source> {
    pub fn new(source: Source) -> RotatePoint<Source> {
        RotatePoint {
            source: source,
            x_angle: 0.0,
            y_angle: 0.0,
            z_angle: 0.0,
            u_angle: 0.0,
        }
    }

    /// Sets the rotation angle around the _x_ axis to apply to the input
    /// value.
    pub fn set_x_angle(self, x_angle: f64) -> RotatePoint<Source> {
        RotatePoint {
            x_angle: x_angle,
            ..self
        }
    }

    /// Sets the rotation angle around the _y_ axis to apply to the input
    /// value.
    pub fn set_y_angle(self, y_angle: f64) -> RotatePoint<Source> {
        RotatePoint {
            y_angle: y_angle,
            ..self
        }
    }

    /// Sets the rotation angle around the _z_ axis to apply to the input
    /// value.
    pub fn set_z_angle(self, z_angle: f64) -> RotatePoint<Source> {
        RotatePoint {
            z_angle: z_angle,
            ..self
        }
    }

    /// Sets the rotation angle around the _u_ axis to apply to the input
    /// value.
    pub fn set_u_angle(self, u_angle: f64) -> RotatePoint<Source> {
        RotatePoint {
            u_angle: u_angle,
            ..self
        }
    }

    /// Sets the rotation angles around all of the axes to apply to the input
    /// value.
    pub fn set_angles(
        self,
        x_angle: f64,
        y_angle: f64,
        z_angle: f64,
        u_angle: f64,
    ) -> RotatePoint<Source> {
        RotatePoint {
            x_angle: x_angle,
            y_angle: y_angle,
            z_angle: z_angle,
            u_angle: u_angle,
            ..self
        }
    }
}

impl<Source> NoiseFn<Point2<f64>> for RotatePoint<Source>
where
    Source: NoiseFn<Point2<f64>>,
{
    fn get(&self, point: Point2<f64>) -> f64 {
        // In two dimensions, the plane is _xy_, and we rotate around the
        // z-axis.
        let x = point[0];
        let y = point[1];
        let theta = self.z_angle.to_radians();

        let x2 = x * theta.cos() - y * theta.sin();
        let y2 = x * theta.sin() + y * theta.cos();

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.get([x2, y2])
    }
}

impl<Source> NoiseFn<Point3<f64>> for RotatePoint<Source>
where
    Source: NoiseFn<Point3<f64>>,
{
    fn get(&self, point: Point3<f64>) -> f64 {
        // In three dimensions, we could rotate around any of the x, y, or z
        // axes. Need a more complicated function to handle this case.
        let x_cos = self.x_angle.to_radians().cos();
        let y_cos = self.y_angle.to_radians().cos();
        let z_cos = self.z_angle.to_radians().cos();
        let x_sin = self.x_angle.to_radians().sin();
        let y_sin = self.y_angle.to_radians().sin();
        let z_sin = self.z_angle.to_radians().sin();

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

impl<Source> NoiseFn<Point4<f64>> for RotatePoint<Source>
where
    Source: NoiseFn<Point4<f64>>,
{
    fn get(&self, _point: Point4<f64>) -> f64 {
        // 4d rotations are hard.
        unimplemented!();
    }
}
