use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    core::perlin::*,
    noise_fns::{NoiseFn, Seedable, DEFAULT_SEED},
    permutationtable::PermutationTable,
    Seed,
};

/// Noise function that outputs 1/2/3/4-dimensional Perlin noise.
#[derive(Clone, Copy, Debug)]
pub struct Perlin {
    seed: Seed,
    perm_table: PermutationTable,
}

impl Perlin {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new(seed: Seed) -> Self {
        let mut rng = XorShiftRng::from_seed(seed);

        Self {
            seed,
            perm_table: rng.gen(),
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new(DEFAULT_SEED)
    }
}

impl Seedable for Perlin {
    /// Sets the seed value for Perlin noise
    fn set_seed(self, seed: Seed) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        let mut rng = XorShiftRng::from_seed(seed);

        Self {
            seed,
            perm_table: rng.gen(),
        }
    }

    fn seed(&self) -> Seed {
        self.seed
    }
}

/// 1-dimensional perlin noise
impl NoiseFn<f64, 1> for Perlin {
    fn get(&self, point: [f64; 1]) -> f64 {
        perlin_1d(point[0], &self.perm_table)
    }
}

/// 2-dimensional perlin noise
impl NoiseFn<f64, 2> for Perlin {
    fn get(&self, point: [f64; 2]) -> f64 {
        perlin_2d(point.into(), &self.perm_table)
    }
}

/// 3-dimensional perlin noise
impl NoiseFn<f64, 3> for Perlin {
    fn get(&self, point: [f64; 3]) -> f64 {
        perlin_3d(point.into(), &self.perm_table)
    }
}

/// 4-dimensional perlin noise
impl NoiseFn<f64, 4> for Perlin {
    fn get(&self, point: [f64; 4]) -> f64 {
        perlin_4d(point.into(), &self.perm_table)
    }
}
