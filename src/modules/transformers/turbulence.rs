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
use modules::{Fbm, MultiFractal, NoiseModule, Seedable};

pub const DEFAULT_TURBULENCE_SEED: usize = 0;
pub const DEFAULT_TURBULENCE_FREQUENCY: f32 = 1.0;
pub const DEFAULT_TURBULENCE_POWER: f32 = 1.0;
pub const DEFAULT_TURBULENCE_ROUGHNESS: usize = 3;

/// Noise Module that randomly displaces the input value before returning the
/// output value from the source module.
///
/// _Turbulence_ is the pseudo-random displacement of the input value. The
/// get() method randomly displaces the coordinates of the input value before
/// retrieving the output value from the source module. To control the
/// turbulence, an application can modify its frequency, its power, and its
/// roughness.
pub struct Turbulence<Source, T> {
    /// Source Module that outputs a value.
    pub source: Source,

    /// Seed value for the Turbulence module.
    pub seed: usize,

    /// Frequency value for the Turbulence module.
    pub frequency: T,

    /// Controls the strength of the turbulence by affecting how much each
    /// point is moved.
    pub power: T,

    /// Affects the roughness of the turbulence. Higher values are rougher.
    pub roughness: usize,

    x_distort_module: Fbm<T>,
    y_distort_module: Fbm<T>,
    z_distort_module: Fbm<T>,
    u_distort_module: Fbm<T>,
}

impl<Source, T> Turbulence<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> Turbulence<Source, T> {
        Turbulence {
            source: source,
            seed: DEFAULT_TURBULENCE_SEED,
            frequency: math::cast(DEFAULT_TURBULENCE_FREQUENCY),
            power: math::cast(DEFAULT_TURBULENCE_POWER),
            roughness: DEFAULT_TURBULENCE_ROUGHNESS,
            x_distort_module: Fbm::new()
                .set_seed(DEFAULT_TURBULENCE_SEED)
                .set_octaves(DEFAULT_TURBULENCE_ROUGHNESS)
                .set_frequency(math::cast(DEFAULT_TURBULENCE_FREQUENCY)),
            y_distort_module: Fbm::new()
                .set_seed(DEFAULT_TURBULENCE_SEED + 1)
                .set_octaves(DEFAULT_TURBULENCE_ROUGHNESS)
                .set_frequency(math::cast(DEFAULT_TURBULENCE_FREQUENCY)),
            z_distort_module: Fbm::new()
                .set_seed(DEFAULT_TURBULENCE_SEED + 2)
                .set_octaves(DEFAULT_TURBULENCE_ROUGHNESS)
                .set_frequency(math::cast(DEFAULT_TURBULENCE_FREQUENCY)),
            u_distort_module: Fbm::new()
                .set_seed(DEFAULT_TURBULENCE_SEED + 3)
                .set_octaves(DEFAULT_TURBULENCE_ROUGHNESS)
                .set_frequency(math::cast(DEFAULT_TURBULENCE_FREQUENCY)),
        }
    }

    pub fn set_frequency(self, frequency: T) -> Turbulence<Source, T> {
        Turbulence {
            frequency: frequency,
            x_distort_module: self.x_distort_module.set_frequency(frequency),
            y_distort_module: self.y_distort_module.set_frequency(frequency),
            z_distort_module: self.z_distort_module.set_frequency(frequency),
            u_distort_module: self.u_distort_module.set_frequency(frequency),
            ..self
        }
    }

    pub fn set_power(self, power: T) -> Turbulence<Source, T> {
        Turbulence { power: power, ..self }
    }

    pub fn set_roughness(self, roughness: usize) -> Turbulence<Source, T> {
        Turbulence {
            roughness: roughness,
            x_distort_module: self.x_distort_module.set_octaves(roughness),
            y_distort_module: self.y_distort_module.set_octaves(roughness),
            z_distort_module: self.z_distort_module.set_octaves(roughness),
            u_distort_module: self.u_distort_module.set_octaves(roughness),
            ..self
        }
    }
}

impl<Source, T> Seedable for Turbulence<Source, T> {
    fn set_seed(self, seed: usize) -> Turbulence<Source, T> {
        Turbulence {
            seed: seed,
            x_distort_module: self.x_distort_module.set_seed(seed),
            y_distort_module: self.y_distort_module.set_seed(seed + 1),
            z_distort_module: self.z_distort_module.set_seed(seed + 2),
            u_distort_module: self.u_distort_module.set_seed(seed + 3),
            ..self
        }
    }
}

impl<Source, T> NoiseModule<Point2<T>> for Turbulence<Source, T>
    where Source: NoiseModule<Point2<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point2<T>) -> Self::Output {
        // First, create offsets based on the input values to keep the sampled
        // points from being near a integer boundary. This is a result of
        // using perlin noise, which returns zero at integer boundaries.
        let x0 = point[0] + math::cast(12414.0 / 65536.0);
        let y0 = point[1] + math::cast(65124.0 / 65536.0);

        let x1 = point[0] + math::cast(26519.0 / 65536.0);
        let y1 = point[1] + math::cast(18128.0 / 65536.0);

        let x_distort = point[0] + (self.x_distort_module.get([x0, y0]) * self.power);
        let y_distort = point[1] + (self.y_distort_module.get([x1, y1]) * self.power);

        self.source.get([x_distort, y_distort])
    }
}

impl<Source, T> NoiseModule<Point3<T>> for Turbulence<Source, T>
    where Source: NoiseModule<Point3<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point3<T>) -> Self::Output {
        // First, create offsets based on the input values to keep the sampled
        // points from being near a integer boundary. This is a result of
        // using perlin noise, which returns zero at integer boundaries.
        let x0 = point[0] + math::cast(12414.0 / 65536.0);
        let y0 = point[1] + math::cast(65124.0 / 65536.0);
        let z0 = point[2] + math::cast(31337.0 / 65536.0);

        let x1 = point[0] + math::cast(26519.0 / 65536.0);
        let y1 = point[1] + math::cast(18128.0 / 65536.0);
        let z1 = point[2] + math::cast(60943.0 / 65536.0);

        let x2 = point[0] + math::cast(53820.0 / 65536.0);
        let y2 = point[1] + math::cast(11213.0 / 65536.0);
        let z2 = point[2] + math::cast(44845.0 / 65536.0);

        let x_distort = point[0] + (self.x_distort_module.get([x0, y0, z0]) * self.power);
        let y_distort = point[1] + (self.y_distort_module.get([x1, y1, z1]) * self.power);
        let z_distort = point[2] + (self.z_distort_module.get([x2, y2, z2]) * self.power);

        self.source.get([x_distort, y_distort, z_distort])
    }
}

impl<Source, T> NoiseModule<Point4<T>> for Turbulence<Source, T>
    where Source: NoiseModule<Point4<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point4<T>) -> Self::Output {
        // First, create offsets based on the input values to keep the sampled
        // points from being near a integer boundary. This is a result of
        // using perlin noise, which returns zero at integer boundaries.
        let x0 = point[0] + math::cast(12414.0 / 65536.0);
        let y0 = point[1] + math::cast(65124.0 / 65536.0);
        let z0 = point[2] + math::cast(31337.0 / 65536.0);
        let u0 = point[3] + math::cast(57948.0 / 65536.0);

        let x1 = point[0] + math::cast(26519.0 / 65536.0);
        let y1 = point[1] + math::cast(18128.0 / 65536.0);
        let z1 = point[2] + math::cast(60943.0 / 65536.0);
        let u1 = point[3] + math::cast(48513.0 / 65536.0);

        let x2 = point[0] + math::cast(53820.0 / 65536.0);
        let y2 = point[1] + math::cast(11213.0 / 65536.0);
        let z2 = point[2] + math::cast(44845.0 / 65536.0);
        let u2 = point[3] + math::cast(39357.0 / 65536.0);

        let x3 = point[0] + math::cast(18128.0 / 65536.0);
        let y3 = point[1] + math::cast(44845.0 / 65536.0);
        let z3 = point[2] + math::cast(12414.0 / 65536.0);
        let u3 = point[3] + math::cast(60943.0 / 65536.0);

        let x_distort = point[0] + (self.x_distort_module.get([x0, y0, z0, u0]) * self.power);
        let y_distort = point[1] + (self.y_distort_module.get([x1, y1, z1, u1]) * self.power);
        let z_distort = point[2] + (self.z_distort_module.get([x2, y2, z2, u2]) * self.power);
        let u_distort = point[3] + (self.u_distort_module.get([x3, y3, z3, u3]) * self.power);

        self.source.get([x_distort, y_distort, z_distort, u_distort])
    }
}
