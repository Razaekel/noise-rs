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

//! An implementation of Ken Perlin's [Improved Noise]
//! (http://mrl.nyu.edu/~perlin/noise/) algorithm.

use std;
use std::rand::Rng;
use std::num::{zero, one, cast};

use Source;
use Quality;

pub static DEFAULT_SEED : int = 0;
pub static DEFAULT_OCTAVE_COUNT : uint = 6;
pub static DEFAULT_FREQUENCY : f64 = 1.0;
pub static DEFAULT_LACUNARITY : f64 = 2.0;
pub static DEFAULT_PERSISTENCE : f64 = 0.5;

/// A perlin noise source
pub struct Perlin {
    /// The seed for the noise
    seed: int,
    /// The number of octaves to use
    octaves: uint,
    /// The base frequency of the noise
    frequency: f64,
    /// How quickly the frequency changes for each octave
    lacunarity: f64,
    /// How quickly the amplitude of each octave decreases
    persistence: f64,
    /// The quality of the noise, with lower quality being faster
    quality: Quality
}

impl Perlin {
    #[inline]
    pub fn new() -> Perlin {
        Perlin {
            seed: DEFAULT_SEED,
            octaves: DEFAULT_OCTAVE_COUNT,
            frequency: DEFAULT_FREQUENCY,
            lacunarity: DEFAULT_LACUNARITY,
            persistence: DEFAULT_PERSISTENCE,
            quality: ::Standard
        }
    }

    // Set a random seed
    pub fn random_seed(&mut self) {
        let mut rng = std::rand::weak_rng();
        self.seed = rng.gen();
    }
}

impl Source for Perlin {
    fn get<F:Float>(&self, x: F, y: F, z: F) -> F {
        let mut value : F = zero();
        let mut cur_persistence = one();

        let frequency = cast(self.frequency).unwrap();
        let lacunarity = cast(self.lacunarity).unwrap();
        let persistence = cast(self.persistence).unwrap();

        let mut x = x * frequency;
        let mut y = y * frequency;
        let mut z = z * frequency;

        for i in range(0, self.octaves) {
            let seed = (self.seed + i as int);

            let signal = ::gen::gradient_coherent_noise_3d(
                x.clone(),
                y.clone(),
                z.clone(), seed, self.quality);
            value = value + signal * cur_persistence;

            x = x * lacunarity;
            y = y * lacunarity;
            z = z * lacunarity;
            cur_persistence = cur_persistence * persistence;

        }

        return value;
    }
}
