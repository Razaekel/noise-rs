use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    core::super_simplex::*,
    noise_fns::{NoiseFn, Seedable, DEFAULT_SEED},
    permutationtable::PermutationTable,
    Seed,
};

/// Noise function that outputs 2/3-dimensional Super Simplex noise.
#[derive(Clone, Copy, Debug)]
pub struct SuperSimplex {
    seed: Seed,
    perm_table: PermutationTable,
}

impl SuperSimplex {
    pub fn new(seed: Seed) -> Self {
        let mut rng = XorShiftRng::from_seed(seed);
        Self {
            seed,
            perm_table: rng.gen(),
        }
    }
}

impl Default for SuperSimplex {
    fn default() -> Self {
        Self::new(DEFAULT_SEED)
    }
}

impl Seedable for SuperSimplex {
    /// Sets the seed value for Super Simplex noise
    fn set_seed(self, seed: Seed) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Self::new(seed)
    }

    fn seed(&self) -> Seed {
        self.seed
    }
}

/// 2-dimensional Super Simplex noise
impl NoiseFn<f64, 2> for SuperSimplex {
    fn get(&self, point: [f64; 2]) -> f64 {
        super_simplex_2d(point.into(), &self.perm_table)
    }
}

/// 3-dimensional Super Simplex noise
impl NoiseFn<f64, 3> for SuperSimplex {
    fn get(&self, point: [f64; 3]) -> f64 {
        super_simplex_3d(point.into(), &self.perm_table)
    }
}
