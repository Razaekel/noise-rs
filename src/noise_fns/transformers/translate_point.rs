// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math::{Point2, Point3, Point4};
use noise_fns::NoiseFn;

/// Noise function that moves the coordinates of the input value before
/// returning the output value from the source function.
///
/// The get() method moves the coordinates of the input value by a translation
/// amount before returning the output value from the source function.
pub struct TranslatePoint<Source> {
    /// Source function that outputs a value
    pub source: Source,

    /// Translation amount applied to the _x_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub x_translation: f64,

    /// Translation amount applied to the _y_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub y_translation: f64,

    /// Translation amount applied to the _z_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub z_translation: f64,

    /// Translation amount applied to the _u_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub u_translation: f64,
}

impl<Source> TranslatePoint<Source> {
    pub fn new(source: Source) -> TranslatePoint<Source> {
        TranslatePoint {
            source: source,
            x_translation: 0.0,
            y_translation: 0.0,
            z_translation: 0.0,
            u_translation: 0.0,
        }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_x_translation(self, x_translation: f64) -> TranslatePoint<Source> {
        TranslatePoint {
            x_translation: x_translation,
            ..self
        }
    }

    /// Sets the scaling factor to apply to the _y_ coordinate of the input
    /// value.
    pub fn set_y_translation(self, y_translation: f64) -> TranslatePoint<Source> {
        TranslatePoint {
            y_translation: y_translation,
            ..self
        }
    }

    /// Sets the scaling factor to apply to the _z_ coordinate of the input
    /// value.
    pub fn set_z_translation(self, z_translation: f64) -> TranslatePoint<Source> {
        TranslatePoint {
            z_translation: z_translation,
            ..self
        }
    }

    /// Sets the scaling factor to apply to the _u_ coordinate of the input
    /// value.
    pub fn set_u_translation(self, u_translation: f64) -> TranslatePoint<Source> {
        TranslatePoint {
            u_translation: u_translation,
            ..self
        }
    }

    /// Sets the translation amount to apply to all coordinates of the input value.
    pub fn set_translation(self, scale: f64) -> TranslatePoint<Source> {
        TranslatePoint {
            x_translation: scale,
            y_translation: scale,
            z_translation: scale,
            u_translation: scale,
            ..self
        }
    }

    /// Sets the individual translation amounts to apply to each coordinate of
    /// the input value.
    pub fn set_all_translations(
        self,
        x_translation: f64,
        y_translation: f64,
        z_translation: f64,
        u_translation: f64,
    ) -> TranslatePoint<Source> {
        TranslatePoint {
            x_translation: x_translation,
            y_translation: y_translation,
            z_translation: z_translation,
            u_translation: u_translation,
            ..self
        }
    }
}

impl<Source> NoiseFn<Point2<f64>> for TranslatePoint<Source>
where
    Source: NoiseFn<Point2<f64>>,
{
    fn get(&self, point: Point2<f64>) -> f64 {
        self.source.get(
            [
                point[0] + self.x_translation,
                point[1] + self.y_translation,
            ],
        )
    }
}

impl<Source> NoiseFn<Point3<f64>> for TranslatePoint<Source>
where
    Source: NoiseFn<Point3<f64>>,
{
    fn get(&self, point: Point3<f64>) -> f64 {
        self.source.get(
            [
                point[0] + self.x_translation,
                point[1] + self.y_translation,
                point[2] + self.z_translation,
            ],
        )
    }
}

impl<Source> NoiseFn<Point4<f64>> for TranslatePoint<Source>
where
    Source: NoiseFn<Point4<f64>>,
{
    fn get(&self, point: Point4<f64>) -> f64 {
        self.source.get(
            [
                point[0] + self.x_translation,
                point[1] + self.y_translation,
                point[2] + self.z_translation,
                point[3] + self.u_translation,
            ],
        )
    }
}
