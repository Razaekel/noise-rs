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
use std::num::{cast, Float};
use std::rand::Rng;

use {math, Quality};
use super::Source;

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
        RidgedMulti {
            seed: DEFAULT_SEED,
            octaves: DEFAULT_OCTAVE_COUNT,
            frequency: DEFAULT_FREQUENCY,
            quality: ::Quality::Fast,
            offset: DEFAULT_OFFSET,
            gain: DEFAULT_GAIN,
            lacunarity: DEFAULT_LACUNARITY,
            exponent: DEFAULT_EXPONENT,
            spectral_weights: Vec::new(),
        }.calc_spectral_weights()
    }

    pub fn seed(self, seed: int) -> RidgedMulti {
        RidgedMulti { seed: seed, ..self }
    }

    pub fn frequency(self, frequency: f64) -> RidgedMulti {
        RidgedMulti { frequency: frequency, ..self }
    }

    // Set a random seed
    pub fn random_seed(self) -> RidgedMulti {
        self.seed(std::rand::weak_rng().gen())
    }

    pub fn lacunarity(self, l: f64) -> RidgedMulti {
        RidgedMulti { lacunarity: l, ..self }.calc_spectral_weights()
    }

    pub fn exponent(self, e: f64) -> RidgedMulti {
        RidgedMulti{ exponent: e, ..self }.calc_spectral_weights()
    }

    pub fn octaves(self, o: uint) -> RidgedMulti {
        RidgedMulti{ octaves: o, ..self }.calc_spectral_weights()
    }

    fn calc_spectral_weights(self) -> RidgedMulti {
        let mut freq = 1.0f64;
        let spectral_weights = Vec::from_fn(self.octaves, |_| {
            let w = freq.powf(-self.exponent);
            freq *= self.lacunarity;
            w
        });
        RidgedMulti { spectral_weights: spectral_weights, ..self }
    }
}

impl Source for RidgedMulti {
    fn get<F:Float>(&self, x: F, y: F, z: F) -> F {
        let mut value : F = Float::zero();
        let mut weight : F = Float::one();

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

            let signal = signal.abs();
            let signal = offset - signal;
            let signal = signal * signal;
            let signal = signal * weight;

            weight = math::clamp(signal * gain, Float::zero(), Float::one());

            value = value + (signal * cast(self.spectral_weights[i]).unwrap());

            x = x * lacunarity;
            y = y * lacunarity;
            z = z * lacunarity;

        }

        value - offset
    }
}
