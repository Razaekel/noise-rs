// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

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
where
    T: Float,
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
        ScalePoint {
            x_scale: x_scale,
            ..self
        }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_y_scale(self, y_scale: T) -> ScalePoint<Source, T> {
        ScalePoint {
            y_scale: y_scale,
            ..self
        }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_z_scale(self, z_scale: T) -> ScalePoint<Source, T> {
        ScalePoint {
            z_scale: z_scale,
            ..self
        }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_u_scale(self, u_scale: T) -> ScalePoint<Source, T> {
        ScalePoint {
            u_scale: u_scale,
            ..self
        }
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
    pub fn set_all_scales(
        self,
        x_scale: T,
        y_scale: T,
        z_scale: T,
        u_scale: T,
    ) -> ScalePoint<Source, T> {
        ScalePoint {
            x_scale: x_scale,
            y_scale: y_scale,
            z_scale: z_scale,
            u_scale: u_scale,
            ..self
        }
    }
}

impl<Source, T> NoiseModule<Point2<T>, T> for ScalePoint<Source, T>
where
    Source: NoiseModule<Point2<T>, T>,
    T: Float,
{
    fn get(&self, point: Point2<T>) -> T {
        self.source
            .get([point[0] * self.x_scale, point[1] * self.y_scale])
    }
}

impl<Source, T> NoiseModule<Point3<T>, T> for ScalePoint<Source, T>
where
    Source: NoiseModule<Point3<T>, T>,
    T: Float,
{
    fn get(&self, point: Point3<T>) -> T {

        self.source.get(
            [
                point[0] * self.x_scale,
                point[1] * self.y_scale,
                point[2] * self.z_scale,
            ],
        )
    }
}

impl<Source, T> NoiseModule<Point4<T>, T> for ScalePoint<Source, T>
where
    Source: NoiseModule<Point4<T>, T>,
    T: Float,
{
    fn get(&self, point: Point4<T>) -> T {
        self.source.get(
            [
                point[0] * self.x_scale,
                point[1] * self.y_scale,
                point[2] * self.z_scale,
                point[3] * self.u_scale,
            ],
        )
    }
}
