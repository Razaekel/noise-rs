use rand_xorshift::XorShiftRng;
use rand::{
    distributions::{Distribution, Standard},
    Rng, SeedableRng,
};
use rand::seq::SliceRandom;
use std::fmt;

const TABLE_SIZE: usize = 256;

/// A seed table, required by all noise functions.
///
/// Table creation is expensive, so in most circumstances you'll only want to
/// create one of these per generator.
#[derive(Copy, Clone)]
pub struct PermutationTable {
    values: [u8; TABLE_SIZE],
}

impl Distribution<PermutationTable> for Standard {
    /// Generates a PermutationTable using a random seed.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PermutationTable {
        let mut seq: Vec<u8> = (0..TABLE_SIZE).map(|x| x as u8).collect();
        seq.shuffle(rng);

        // It's unfortunate that this double-initializes the array, but Rust
        // doesn't currently provide a clean way to do this in one pass. Hopefully
        // it won't matter, as Seed creation will usually be a one-time event.
        let mut perm_table = PermutationTable {
            values: [0; TABLE_SIZE],
        };
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
    pub fn new(seed: u32) -> Self {
        let mut real = [0; 16];
        real[0] = 1;
        for i in 1..4 {
            real[i * 4] = seed as u8;
            real[(i * 4) + 1] = (seed >> 8) as u8;
            real[(i * 4) + 2] = (seed >> 16) as u8;
            real[(i * 4) + 3] = (seed >> 24) as u8;
        }
        let mut rng: XorShiftRng = SeedableRng::from_seed(real);
        rng.gen()
    }

    pub fn get1(&self, x: isize) -> usize {
        let x = (x & 0xff) as usize;
        self.values[x] as usize
    }

    pub fn get2(&self, pos: [isize; 2]) -> usize {
        let y = (pos[1] & 0xff) as usize;
        self.values[self.get1(pos[0]) ^ y] as usize
    }

    pub fn get3(&self, pos: [isize; 3]) -> usize {
        let z = (pos[2] & 0xff) as usize;
        self.values[self.get2([pos[0], pos[1]]) ^ z] as usize
    }

    pub fn get4(&self, pos: [isize; 4]) -> usize {
        let w = (pos[3] & 0xff) as usize;
        self.values[self.get3([pos[0], pos[1], pos[2]]) ^ w] as usize
    }
}

impl fmt::Debug for PermutationTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PermutationTable {{ .. }}")
    }
}

#[cfg(test)]
mod tests {
    use rand::random;
    use crate::{NoiseFn, Perlin, Seedable};

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
