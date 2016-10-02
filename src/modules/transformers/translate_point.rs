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
use math::{Point2, Point3, Point4};
use NoiseModule;

/// Noise Module that moves the coordinates of the input value before
/// returning the output value from the source module.
///
/// The get() method moves the coordinates of the input value by a translation
/// amount before returning the output value from the source module.
pub struct TranslatePoint<Source, T> {
    /// Source Module that outputs a value
    pub source: Source,

    /// Translation amount applied to the _x_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub x_translation: T,

    /// Translation amount applied to the _y_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub y_translation: T,

    /// Translation amount applied to the _z_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub z_translation: T,

    /// Translation amount applied to the _u_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub u_translation: T,
}

impl<Source, T> TranslatePoint<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> TranslatePoint<Source, T> {
        TranslatePoint {
            source: source,
            x_translation: T::zero(),
            y_translation: T::zero(),
            z_translation: T::zero(),
            u_translation: T::zero(),
        }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_x_translation(self, x_translation: T) -> TranslatePoint<Source, T> {
        TranslatePoint { x_translation: x_translation, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_y_translation(self, y_translation: T) -> TranslatePoint<Source, T> {
        TranslatePoint { y_translation: y_translation, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_z_translation(self, z_translation: T) -> TranslatePoint<Source, T> {
        TranslatePoint { z_translation: z_translation, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_u_translation(self, u_translation: T) -> TranslatePoint<Source, T> {
        TranslatePoint { u_translation: u_translation, ..self }
    }

    /// Sets the translation amount to apply to all coordinates of the input value.
    pub fn set_translation(self, scale: T) -> TranslatePoint<Source, T> {
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
    pub fn set_all_translations(self,
                                x_translation: T,
                                y_translation: T,
                                z_translation: T,
                                u_translation: T)
                                -> TranslatePoint<Source, T> {
        TranslatePoint {
            x_translation: x_translation,
            y_translation: y_translation,
            z_translation: z_translation,
            u_translation: u_translation,
            ..self
        }
    }
}

impl<Source, T> NoiseModule<Point2<T>> for TranslatePoint<Source, T>
    where Source: NoiseModule<Point2<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point2<T>) -> Self::Output {
        self.source.get([point[0] + self.x_translation, point[1] + self.y_translation])
    }
}

impl<Source, T> NoiseModule<Point3<T>> for TranslatePoint<Source, T>
    where Source: NoiseModule<Point3<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point3<T>) -> Self::Output {

        self.source.get([point[0] + self.x_translation,
                         point[1] + self.y_translation,
                         point[2] + self.z_translation])
    }
}

impl<Source, T> NoiseModule<Point4<T>> for TranslatePoint<Source, T>
    where Source: NoiseModule<Point4<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point4<T>) -> Self::Output {
        self.source.get([point[0] + self.x_translation,
                         point[1] + self.y_translation,
                         point[2] + self.z_translation,
                         point[3] + self.u_translation])
    }
}
