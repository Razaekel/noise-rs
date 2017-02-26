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
#[deprecated(since="0.3.0", note="will be made private by 1.0")]
pub struct PermutationTable {
    values: [u8; TABLE_SIZE],
}

impl Rand for PermutationTable {
    /// Generates a PermutationTable using a random seed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate noise;
    /// extern crate rand;
    ///
    /// use noise::PermutationTable;
    ///
    /// # fn main() {
    /// let perm_table = rand::random::<PermutationTable>();
    /// # }
    /// ```
    ///
    /// ```rust
    /// extern crate noise;
    /// extern crate rand;
    ///
    /// use noise::PermutationTable;
    /// use rand::{SeedableRng, Rng, XorShiftRng};
    ///
    /// # fn main() {
    /// let mut rng: XorShiftRng = SeedableRng::from_seed([1, 2, 3, 4]);
    /// let perm_table = rng.gen::<PermutationTable>();
    /// # }
    /// ```
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use noise::PermutationTable;
    ///
    /// let perm_table = PermutationTable::new(12);
    /// ```
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
    use super::PermutationTable;
    use perlin::perlin3;
    use rand::random;

    #[test]
    fn test_random_seed() {
        let _ = perlin3::<f32>(&random(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_negative_params() {
        let _ = perlin3::<f32>(&PermutationTable::new(0), &[-1.0, 2.0, 3.0]);
    }
}
