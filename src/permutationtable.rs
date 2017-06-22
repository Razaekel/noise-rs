// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

// TODO: Use PrimInt + Signed instead of SignedInt + NumCast once num has
// PrimInt implementations

use math;
use num_traits::{NumCast, PrimInt, Signed};
use rand::{Rand, Rng, SeedableRng, XorShiftRng};
use std::fmt;

const TABLE_SIZE: usize = 256;

/// A seed table, required by all noise functions.
///
/// Table creation is expensive, so in most circumstances you'll only want to
/// create one of these per generator.
#[derive(Copy)]
pub struct PermutationTable {
    values: [u8; TABLE_SIZE],
}

impl Rand for PermutationTable {
    /// Generates a PermutationTable using a random seed.
    fn rand<R: Rng>(rng: &mut R) -> PermutationTable {
        let mut seq: Vec<u8> = (0..TABLE_SIZE).map(|x| x as u8).collect();
        rng.shuffle(&mut *seq);

        // It's unfortunate that this double-initializes the array, but Rust
        // doesn't currently provide a clean way to do this in one pass. Hopefully
        // it won't matter, as Seed creation will usually be a one-time event.
        let mut perm_table = PermutationTable { values: [0; TABLE_SIZE] };
        let seq_it = seq.iter();
        for (x, y) in perm_table.values.iter_mut().zip(seq_it) {
            *x = *y
        }
        perm_table
    }
}

impl PermutationTable {
    /// Deterministically generates a new permutation table based on a `u32` seed value.
    ///
    /// Internally this uses a `XorShiftRng`, but we don't really need to worry
    /// about cryptographic security when working with procedural noise.
    pub fn new(seed: u32) -> PermutationTable {
        let mut rng: XorShiftRng = SeedableRng::from_seed([1, seed, seed, seed]);
        rng.gen()
    }

    #[inline(always)]
    pub fn get1<T: Signed + PrimInt + NumCast>(&self, x: T) -> usize {
        let x: usize = math::cast(x & math::cast(0xff));
        self.values[x] as usize
    }

    #[inline(always)]
    pub fn get2<T: Signed + PrimInt + NumCast>(&self, pos: math::Point2<T>) -> usize {
        let y: usize = math::cast(pos[1] & math::cast(0xff));
        self.values[self.get1(pos[0]) ^ y] as usize
    }

    #[inline(always)]
    pub fn get3<T: Signed + PrimInt + NumCast>(&self, pos: math::Point3<T>) -> usize {
        let z: usize = math::cast(pos[2] & math::cast(0xff));
        self.values[self.get2([pos[0], pos[1]]) ^ z] as usize
    }

    #[inline(always)]
    pub fn get4<T: Signed + PrimInt + NumCast>(&self, pos: math::Point4<T>) -> usize {
        let w: usize = math::cast(pos[3] & math::cast(0xff));
        self.values[self.get3([pos[0], pos[1], pos[2]]) ^ w] as usize
    }
}

impl Clone for PermutationTable {
    fn clone(&self) -> PermutationTable {
        PermutationTable { values: self.values }
    }
}

impl fmt::Debug for PermutationTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PermutationTable {{ .. }}")
    }
}

#[cfg(test)]
mod tests {
    use {NoiseFn, Perlin, Seedable};
    use rand::random;

    #[test]
    fn test_random_seed() {
        let perlin = Perlin::new().set_seed(random());
        let _ = perlin.get([1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_negative_params() {
        let perlin = Perlin::new();
        let _ = perlin.get([-1.0, 2.0, 3.0]);
    }
}
