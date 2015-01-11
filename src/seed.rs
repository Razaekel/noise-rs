// Copyright 2015 The noise-rs developers. For a full listing of the authors,
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

#![allow(unstable)]

use std::rand::{Rand, Rng, SeedableRng, XorShiftRng};
use std::num::SignedInt;

use math;

const TABLE_SIZE: usize = 256;

#[allow(missing_copy_implementations)]
pub struct Seed {
    x_values: [u8; TABLE_SIZE],
    y_values: [u8; TABLE_SIZE],
    z_values: [u8; TABLE_SIZE],
    w_values: [u8; TABLE_SIZE],
}

impl Rand for Seed {
    fn rand<R: Rng>(rng: &mut R) -> Seed {
        let mut x_values: Vec<u8> = ::std::iter::range_inclusive(0, (TABLE_SIZE - 1) as u8).collect();
        let mut y_values: Vec<u8> = x_values.clone();
        let mut z_values: Vec<u8> = x_values.clone();
        let mut w_values: Vec<u8> = x_values.clone();

        rng.shuffle(&mut *x_values);
        rng.shuffle(&mut *y_values);
        rng.shuffle(&mut *z_values);
        rng.shuffle(&mut *w_values);

        // It's unfortunate that this double-initializes the array, but Rust doesn't currently provide a
        // clean way to do this in one pass. Hopefully won't matter, as Seed creation will usually be a
        // one-time event.
        let mut seed = Seed {
            x_values: [0; TABLE_SIZE],
            y_values: [0; TABLE_SIZE],
            z_values: [0; TABLE_SIZE],
            w_values: [0; TABLE_SIZE],
        };

        let x_iter = x_values.iter().cycle();
        for (x, y) in seed.x_values.iter_mut().zip(x_iter) { *x = *y }
        let y_iter = y_values.iter().cycle();
        for (x, y) in seed.y_values.iter_mut().zip(y_iter) { *x = *y }
        let z_iter = z_values.iter().cycle();
        for (x, y) in seed.z_values.iter_mut().zip(z_iter) { *x = *y }
        let w_iter = w_values.iter().cycle();
        for (x, y) in seed.w_values.iter_mut().zip(w_iter) { *x = *y }
        seed
    }
}

impl Seed {
    pub fn new(seed: u32) -> Seed {
        let mut rng: XorShiftRng = SeedableRng::from_seed([1, seed, seed, seed]);
        rng.gen()
    }

    #[inline(always)]
    pub fn get1<T: SignedInt>(&self, x: T) -> usize {
        self.x_values[math::cast::<T, usize>(x & math::cast(TABLE_SIZE - 1))] as usize
    }

    #[inline(always)]
    pub fn get2<T: SignedInt>(&self, pos: math::Point2<T>) -> usize {
        self.get1(pos[0]) ^ self.y_values[math::cast::<T, usize>(pos[1] & math::cast(TABLE_SIZE - 1))] as usize
    }

    #[inline(always)]
    pub fn get3<T: SignedInt>(&self, pos: math::Point3<T>) -> usize {
        self.get2([pos[0], pos[1]]) ^ self.z_values[math::cast::<T, usize>(pos[2] & math::cast(TABLE_SIZE - 1))] as usize
    }

    #[inline(always)]
    pub fn get4<T: SignedInt>(&self, pos: math::Point4<T>) -> usize {
        self.get3([pos[0], pos[1], pos[2]]) ^ self.w_values[math::cast::<T, usize>(pos[3] & math::cast(TABLE_SIZE - 1))] as usize
    }
}

#[cfg(test)]
mod tests {
    use std::rand::random;
    use perlin::perlin3;
    use super::Seed;

    #[test]
    fn test_random_seed() {
        let _ = perlin3::<f32>(&random(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_negative_params() {
        let _ = perlin3::<f32>(&Seed::new(0), &[-1.0, 2.0, 3.0]);
    }
}
