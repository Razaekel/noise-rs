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

use std;
use std::num::{zero,one,abs,cast};
use std::rand::Rng;

use util::clamp;
use super::Source;
use Quality;

static DEFAULT_SEED : int = 0;
static DEFAULT_OCTAVE_COUNT : uint = 6;
static DEFAULT_FREQUENCY : f64 = 1.0;
static DEFAULT_LACUNARITY : f64 = 2.0;
static DEFAULT_EXPONENT : f64 = 1.0;
static DEFAULT_OFFSET : f64 = 1.0;
static DEFAULT_GAIN : f64 = 2.0;

#[deriving(Clone)]
pub struct RidgedMulti {
    pub seed: int,
    pub frequency: f64,
    pub quality: Quality,
    pub offset: f64,
    pub gain: f64,
    octaves: uint,
    lacunarity: f64,
    exponent: f64,
    spectral_weights: Vec<f64>,
}

impl RidgedMulti {
    pub fn new() -> RidgedMulti {
        let mut r = RidgedMulti {
            seed: DEFAULT_SEED,
            octaves: DEFAULT_OCTAVE_COUNT,
            frequency: DEFAULT_FREQUENCY,
            quality: ::Standard,
            offset: DEFAULT_OFFSET,
            gain: DEFAULT_GAIN,
            lacunarity: DEFAULT_LACUNARITY,
            exponent: DEFAULT_EXPONENT,
            spectral_weights: Vec::new(),
        };

        r.calc_spectral_weights();
        return r;
    }

    pub fn lacunarity(&self) -> f64 {
        self.lacunarity
    }

    // Set a random seed
    pub fn random_seed(&mut self) {
        let mut rng = std::rand::weak_rng();
        self.seed = rng.gen();
    }

    pub fn set_lacunarity(&mut self, l: f64) {
        self.lacunarity = l;
        self.calc_spectral_weights();
    }

    pub fn exponent(&self) -> f64 {
        self.exponent
    }

    pub fn set_exponent(&mut self, e: f64) {
        self.exponent = e;
        self.calc_spectral_weights();
    }

    pub fn octaves(&self) -> uint {
        self.octaves
    }

    pub fn set_octaves(&mut self, o: uint) {
        self.octaves = o;
        self.calc_spectral_weights();
    }

    fn calc_spectral_weights(&mut self) {
        let mut freq = 1.0f64;
        self.spectral_weights = Vec::from_fn(self.octaves, |_| {
            let w = freq.powf(-self.exponent);
            freq *= self.lacunarity;
            w
        });
    }
}

impl Source for RidgedMulti {

    fn get<F:Float>(&self, x: F, y: F, z: F) -> F {
        let mut value : F = zero();
        let mut weight : F = one();

        let offset : F = cast(self.offset).unwrap();
        let gain = cast(self.gain).unwrap();
        let frequency = cast(self.frequency).unwrap();
        let lacunarity = cast(self.lacunarity).unwrap();

        let mut x = x * frequency;
        let mut y = y * frequency;
        let mut z = z * frequency;

        for i in range(0, self.octaves) {
            let seed = self.seed + i as int;

            let signal = ::gen::gradient_coherent_noise_3d(
                x.clone(),
                y.clone(),
                z.clone(), seed, self.quality);

            let signal = abs(signal);
            let signal = offset - signal;
            let signal = signal * signal;
            let signal = signal * weight;

            weight = clamp(signal * gain, zero(), one());

            value = value + (signal * cast(self.spectral_weights[i]).unwrap());

            x = x * lacunarity;
            y = y * lacunarity;
            z = z * lacunarity;

        }

        return value - offset;
    }
}
