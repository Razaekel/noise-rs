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

use std::rand::{Rand, Rng, SeedableRng, XorShiftRng};
use std::num::SignedInt;

use math;

const TABLE_SIZE: uint = 256;

#[allow(missing_copy_implementations)]
pub struct Seed {
    values: [uint; TABLE_SIZE * 2],
}

impl Rand for Seed {
    fn rand<R: Rng>(rng: &mut R) -> Seed {
        let mut seq: Vec<uint> = range(0u, TABLE_SIZE).collect();
        for i in range(0, TABLE_SIZE) {
            let mut swap_i: uint = rng.gen();
            swap_i = swap_i % TABLE_SIZE;
            let swap = seq[swap_i];
            seq[swap_i] = seq[i];
            seq[i] = swap;
        }

        // It's unfortunate that this double-initializes the array, but Rust doesn't currently provide a
        // clean way to do this in one pass. Hopefully won't matter, as Seed creation will usually be a
        // one-time event.
        let mut new_seed = Seed {
            values: [0; TABLE_SIZE * 2],
        };
        for i in range(0, TABLE_SIZE * 2) {
            new_seed.values[i] = seq[i % TABLE_SIZE];
        }
        new_seed
    }
}

impl Seed {
    pub fn new(seed: u32) -> Seed {
        let mut rng: XorShiftRng = SeedableRng::from_seed([1, seed, seed, seed]);
        Rand::rand(&mut rng)
    }

    #[inline(always)]
    pub fn get1<T: SignedInt>(&self, x: T) -> uint {
        self.values[math::cast::<T, uint>(x) & (TABLE_SIZE - 1)]
    }

    #[inline(always)]
    pub fn get2<T: SignedInt>(&self, pos: math::Point2<T>) -> uint {
        self.values[self.get1(pos[0]) + (math::cast::<T, uint>(pos[1]) & (TABLE_SIZE - 1))]
    }

    #[inline(always)]
    pub fn get3<T: SignedInt>(&self, pos: math::Point3<T>) -> uint {
        self.values[self.get2([pos[0], pos[1]]) + (math::cast::<T, uint>(pos[2]) & (TABLE_SIZE - 1))]
    }

    #[inline(always)]
    pub fn get4<T: SignedInt>(&self, pos: math::Point4<T>) -> uint {
        self.values[self.get3([pos[0], pos[1], pos[2]]) + (math::cast::<T, uint>(pos[3]) & (TABLE_SIZE - 1))]
    }
}

#[cfg(test)]
mod tests {
    use std::rand::random;
    use perlin::perlin3;

    #[test]
    fn test_random_seed() {
        let _ = perlin3::<f32>(&random(), &[1.0, 2.0, 3.0]);
    }
}
