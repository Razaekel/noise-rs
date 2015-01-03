// Copyright 2013 The noise-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

use {math, Seed, Point2, Point3, Point4};

macro_rules! brownian {
    { $Brownian:ident, $Point:ident } => {
        pub struct $Brownian<T, F: Fn(&Seed, &$Point<T>) -> T> {
            /// The underlying noise function
            pub function: F,
            /// The number of octaves to use
            pub octaves: uint,
            /// The base frequency of the noise
            pub frequency: T,
            /// How quickly the amplitude of each octave decreases
            pub persistence: T,
            /// How quickly the frequency changes for each octave
            pub lacunarity: T,
        }

        impl<T: Float, F> $Brownian<T, F> where
            F: Fn(&Seed, &$Point<T>) -> T,
        {
            /// Consructs a new brownian noise function, defaulting to:
            ///
            /// - frequency: `1.0`
            /// - lacunarity: `2.0`
            /// - persistence: `0.5`
            #[inline]
            pub fn new(function: F, octaves: uint) -> $Brownian<T, F> {
                $Brownian {
                    function: function,
                    octaves: octaves,
                    frequency: math::cast(1.0f32),
                    lacunarity: math::cast(2.0f32),
                    persistence: math::cast(0.5f32),
                }
            }

            #[inline]
            pub fn function<Q>(self, function: Q) -> $Brownian<T, Q> where
                Q: Fn(&Seed, &$Point<T>) -> T,
            {
                let $Brownian { octaves, frequency, lacunarity, persistence, .. } = self;
                $Brownian {
                    function: function,
                    octaves: octaves,
                    frequency: frequency,
                    persistence: persistence,
                    lacunarity: lacunarity,
                }
            }

            #[inline]
            pub fn octaves(self, octaves: uint) -> $Brownian<T, F> {
                $Brownian { octaves: octaves, ..self }
            }

            #[inline]
            pub fn wavelength(self, wavelength: T) -> $Brownian<T, F> {
                $Brownian { frequency: wavelength.recip(), ..self }
            }

            #[inline]
            pub fn frequency(self, frequency: T) -> $Brownian<T, F> {
                $Brownian { frequency: frequency, ..self }
            }

            #[inline]
            pub fn persistence(self, persistence: T) -> $Brownian<T, F> {
                $Brownian { persistence: persistence, ..self }
            }

            #[inline]
            pub fn lacunarity(self, lacunarity: T) -> $Brownian<T, F> {
                $Brownian { lacunarity: lacunarity, ..self }
            }
        }
    }
}

brownian! { Brownian2, Point2 }
brownian! { Brownian3, Point3 }
brownian! { Brownian4, Point4 }

impl<'a, 'b, T, F> Fn(&'a Seed, &'b Point2<T>) -> T for Brownian2<T, F> where
    T: Float,
    F: Fn(&Seed, &Point2<T>) -> T,
{
    extern "rust-call" fn call(&self, (seed, point): (&'a Seed, &'b Point2<T>)) -> T {
        let mut frequency: T = self.frequency;
        let mut amplitude: T = Float::one();
        let mut result: T = Float::zero();
        for _ in range(0, self.octaves) {
            let scaled_point = [point[0] * frequency,
                                point[1] * frequency];
            result = result + ((self.function)(seed, &scaled_point) * amplitude);
            amplitude = amplitude * self.persistence;
            frequency = frequency * self.lacunarity;
        }
        result
    }
}

impl<'a, 'b, T, F> Fn(&'a Seed, &'b ::Point3<T>) -> T for Brownian3<T, F> where
    T: Float,
    F: Fn(&Seed, &::Point3<T>) -> T,
{
    extern "rust-call" fn call(&self, (seed, point): (&'a Seed, &'b ::Point3<T>)) -> T {
        let mut frequency: T = self.frequency;
        let mut amplitude: T = Float::one();
        let mut result: T = Float::zero();
        for _ in range(0, self.octaves) {
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

impl<'a, 'b, T, F> Fn(&'a Seed, &'b ::Point4<T>) -> T for Brownian4<T, F> where
    T: Float,
    F: Fn(&Seed, &::Point4<T>) -> T,
{
    extern "rust-call" fn call(&self, (seed, point): (&'a Seed, &'b ::Point4<T>)) -> T {
        let mut frequency: T = self.frequency;
        let mut amplitude: T = Float::one();
        let mut result: T = Float::zero();
        for _ in range(0, self.octaves) {
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
