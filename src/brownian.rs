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

use std::num::Float;

use {math, Seed};
use {GenFn2, GenFn3, GenFn4};
use {Point2, Point3, Point4};

/// A callable struct for applying 2-dimensional [fractional Brownian motion]
/// (http://en.wikipedia.org/wiki/Fractional_Brownian_motion).
///
/// Fractional Brownian motion is a way of combining multiple octaves of a noise
/// function to create a richer and more varied output. It can theoretically be
/// used with any noise function, but it tends to only produce good results with
/// gradient noise functions.
///
/// The struct contains many parameters, which can either be set using the
/// builder methods provided, or by constructing the type manually.
///
/// # Example
///
/// ```rust
/// extern crate noise;
/// extern crate rand;
///
/// use noise::{Brownian2, perlin2};
///
/// # fn main() {
/// let seed = rand::random();
/// let noise = Brownian2::new(perlin2, 4).wavelength(32.0);
/// let val = noise(&seed, &[42.0, 37.0]);
/// # }
/// ```
#[derive(Copy, Clone)]
pub struct Brownian2<T, F: GenFn2<T>> {
    /// The underlying noise function to call.
    pub function: F,
    /// The number of times to call the noise function.
    pub octaves: usize,
    /// The base frequency of the noise
    pub frequency: T,
    /// The rate at which the amplitude of the noise is reduced for each octave.
    pub persistence: T,
    /// The rate at which the frequency of the noise increases for each octave.
    pub lacunarity: T,
}

/// A callable struct for applying 3-dimensional [fractional Brownian motion]
/// (http://en.wikipedia.org/wiki/Fractional_Brownian_motion).
///
/// Fractional Brownian motion is a way of combining multiple octaves of a noise
/// function to create a richer and more varied output. It can theoretically be
/// used with any noise function, but it tends to only produce good results with
/// gradient noise functions.
///
/// The struct contains many parameters, which can either be set using the
/// builder methods provided, or by constructing the type manually.
///
/// # Example
///
/// ```rust
/// extern crate noise;
/// extern crate rand;
///
/// use noise::{Brownian3, perlin3};
///
/// # fn main() {
/// let seed = rand::random();
/// let noise = Brownian3::new(perlin3, 4).wavelength(32.0);
/// let val = noise(&seed, &[42.0, 37.0, 2.0]);
/// # }
/// ```
#[derive(Copy, Clone)]
pub struct Brownian3<T, F: GenFn3<T>> {
    /// The underlying noise function to call.
    pub function: F,
    /// The number of times to call the noise function.
    pub octaves: usize,
    /// The base frequency of the noise
    pub frequency: T,
    /// The rate at which the amplitude of the noise is reduced for each octave.
    pub persistence: T,
    /// The rate at which the frequency of the noise increases for each octave.
    pub lacunarity: T,
}

/// A callable struct for applying 4-dimensional [fractional Brownian motion]
/// (http://en.wikipedia.org/wiki/Fractional_Brownian_motion).
///
/// Fractional Brownian motion is a way of combining multiple octaves of a noise
/// function to create a richer and more varied output. It can theoretically be
/// used with any noise function, but it tends to only produce good results with
/// gradient noise functions.
///
/// The struct contains many parameters, which can either be set using the
/// builder methods provided, or by constructing the type manually.
///
/// # Example
///
/// ```rust
/// extern crate noise;
/// extern crate rand;
///
/// use noise::{Brownian4, perlin4};
///
/// # fn main() {
/// let seed = rand::random();
/// let noise = Brownian4::new(perlin4, 4).wavelength(32.0);
/// let val = noise(&seed, &[42.0, 37.0, 2.0, 3.0]);
/// # }
/// ```
#[derive(Copy, Clone)]
pub struct Brownian4<T, F: GenFn4<T>> {
    /// The underlying noise function to call on each octave.
    pub function: F,
    /// The number of times to call the noise function.
    pub octaves: usize,
    /// The base frequency of the noise
    pub frequency: T,
    /// The rate at which the amplitude of the noise is reduced on each octave.
    pub persistence: T,
    /// The rate at which the frequency of the noise increases on each octave.
    pub lacunarity: T,
}

macro_rules! impl_brownian {
    { $Brownian:ident, $GenFn:ident } => {
        impl<T: Float, F: $GenFn<T>> $Brownian<T, F> {
            /// Constructs a new brownian noise function, defaulting to:
            ///
            /// - frequency: `1.0`
            /// - lacunarity: `2.0`
            /// - persistence: `0.5`
            #[inline]
            pub fn new(function: F, octaves: usize) -> $Brownian<T, F> {
                $Brownian {
                    function: function,
                    octaves: octaves,
                    frequency: math::cast(1.0f32),
                    lacunarity: math::cast(2.0f32),
                    persistence: math::cast(0.5f32),
                }
            }

            /// A builder method that sets underlying noise function to call on
            /// each octave.
            #[inline]
            pub fn function<Q: $GenFn<T>>(self, function: Q) -> $Brownian<T, Q> {
                let $Brownian { octaves, frequency, lacunarity, persistence, .. } = self;
                $Brownian {
                    function: function,
                    octaves: octaves,
                    frequency: frequency,
                    persistence: persistence,
                    lacunarity: lacunarity,
                }
            }

            /// A builder method that sets the number of times to call the noise
            /// function.
            #[inline]
            pub fn octaves(self, octaves: usize) -> $Brownian<T, F> {
                $Brownian { octaves: octaves, ..self }
            }

            /// A builder method that sets the wavelength of the brownian noise.
            /// This is equivalent to `self.frequency(wavelength.recip())`.
            #[inline]
            pub fn wavelength(self, wavelength: T) -> $Brownian<T, F> {
                self.frequency(wavelength.recip())
            }

            /// A builder method that sets the base frequency of the noise.
            #[inline]
            pub fn frequency(self, frequency: T) -> $Brownian<T, F> {
                $Brownian { frequency: frequency, ..self }
            }

            /// A builder method that sets the rate at which the amplitude of
            /// the noise is reduced on each octave.
            #[inline]
            pub fn persistence(self, persistence: T) -> $Brownian<T, F> {
                $Brownian { persistence: persistence, ..self }
            }

            /// A builder method that sets the rate at which the frequency of
            /// the noise increases on each octave.
            #[inline]
            pub fn lacunarity(self, lacunarity: T) -> $Brownian<T, F> {
                $Brownian { lacunarity: lacunarity, ..self }
            }
        }
    }
}

impl_brownian! { Brownian2, GenFn2 }
impl_brownian! { Brownian3, GenFn3 }
impl_brownian! { Brownian4, GenFn4 }

impl<'a, 'b, T, F> Fn<(&'a Seed, &'b Point2<T>)> for Brownian2<T, F> where
    T: Float,
    F: GenFn2<T>,
{
    /// Applies the brownian noise function for the supplied seed and point.
    extern "rust-call" fn call(&self, (seed, point): (&'a Seed, &'b Point2<T>)) -> T {
        let mut frequency: T = self.frequency;
        let mut amplitude: T = math::cast(1);
        let mut result: T = math::cast(0);
        for _ in 0..self.octaves {
            let scaled_point = [point[0] * frequency,
                                point[1] * frequency];
            result = result + ((self.function)(seed, &scaled_point) * amplitude);
            amplitude = amplitude * self.persistence;
            frequency = frequency * self.lacunarity;
        }
        result
    }
}

impl<'a, 'b, T, F> FnMut<(&'a Seed, &'b Point2<T>)> for Brownian2<T, F> where
    T: Float,
    F: GenFn2<T>,
{
    extern "rust-call" fn call_mut(&mut self, (seed, point): (&'a Seed, &'b Point2<T>)) -> T {
        self.call((seed, point))
    }
}

impl<'a, 'b, T, F> FnOnce<(&'a Seed, &'b Point2<T>)> for Brownian2<T, F> where
    T: Float,
    F: GenFn2<T>,
{
    type Output = T;
    extern "rust-call" fn call_once(self, (seed, point): (&'a Seed, &'b Point2<T>)) -> T {
        self.call((seed, point))
    }
}

impl<'a, 'b, T, F> Fn<(&'a Seed, &'b Point3<T>)> for Brownian3<T, F> where
    T: Float,
    F: GenFn3<T>,
{
    /// Applies the brownian noise function for the supplied seed and point.
    extern "rust-call" fn call(&self, (seed, point): (&'a Seed, &'b Point3<T>)) -> T {
        let mut frequency: T = self.frequency;
        let mut amplitude: T = math::cast(1);
        let mut result: T = math::cast(0);
        for _ in 0..self.octaves {
            let scaled_point = [point[0] * frequency,
                                point[1] * frequency,
                                point[2] * frequency];
            result = result + ((self.function)(seed, &scaled_point) * amplitude);
            amplitude = amplitude * self.persistence;
            frequency = frequency * self.lacunarity;
        }
        result
    }
}

impl<'a, 'b, T, F> FnMut<(&'a Seed, &'b Point3<T>)> for Brownian3<T, F> where
    T: Float,
    F: GenFn3<T>,
{
    extern "rust-call" fn call_mut(&mut self, (seed, point): (&'a Seed, &'b Point3<T>)) -> T {
        self.call((seed, point))
    }
}

impl<'a, 'b, T, F> FnOnce<(&'a Seed, &'b Point3<T>)> for Brownian3<T, F> where
    T: Float,
    F: GenFn3<T>,
{
    type Output = T;
    extern "rust-call" fn call_once(self, (seed, point): (&'a Seed, &'b Point3<T>)) -> T {
        self.call((seed, point))
    }
}

impl<'a, 'b, T, F> Fn<(&'a Seed, &'b ::Point4<T>)> for Brownian4<T, F> where
    T: Float,
    F: GenFn4<T>,
{
    /// Applies the brownian noise function for the supplied seed and point.
    extern "rust-call" fn call(&self, (seed, point): (&'a Seed, &'b Point4<T>)) -> T {
        let mut frequency: T = self.frequency;
        let mut amplitude: T = math::cast(1);
        let mut result: T = math::cast(0);
        for _ in 0..self.octaves {
            let scaled_point = [point[0] * frequency,
                                point[1] * frequency,
                                point[2] * frequency,
                                point[3] * frequency];
            result = result + ((self.function)(seed, &scaled_point) * amplitude);
            amplitude = amplitude * self.persistence;
            frequency = frequency * self.lacunarity;
        }
        result
    }
}

impl<'a, 'b, T, F> FnMut<(&'a Seed, &'b Point4<T>)> for Brownian4<T, F> where
    T: Float,
    F: GenFn4<T>,
{
    extern "rust-call" fn call_mut(&mut self, (seed, point): (&'a Seed, &'b Point4<T>)) -> T {
        self.call((seed, point))
    }
}

impl<'a, 'b, T, F> FnOnce<(&'a Seed, &'b Point4<T>)> for Brownian4<T, F> where
    T: Float,
    F: GenFn4<T>,
{
    type Output = T;
    extern "rust-call" fn call_once(self, (seed, point): (&'a Seed, &'b Point4<T>)) -> T {
        self.call((seed, point))
    }
}
