use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    core::value::{value_2d, value_3d, value_4d},
    noise_fns::{NoiseFn, Seedable, DEFAULT_SEED},
    permutationtable::PermutationTable,
    Seed,
};

/// Noise function that outputs 2/3/4-dimensional Value noise.
#[derive(Clone, Copy, Debug)]
pub struct Value {
    seed: Seed,
    perm_table: PermutationTable,
}

impl Value {
    pub fn new(seed: Seed) -> Self {
        let mut rng = XorShiftRng::from_seed(seed);

        Self {
            seed,
            perm_table: rng.gen(),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::new(DEFAULT_SEED)
    }
}

impl Seedable for Value {
    /// Sets the seed value for Value noise
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

/// 2-dimensional value noise
impl NoiseFn<f64, 2> for Value {
    fn get(&self, point: [f64; 2]) -> f64 {
        value_2d(point.into(), &self.perm_table)
    }
}

/// 3-dimensional value noise
impl NoiseFn<f64, 3> for Value {
    fn get(&self, point: [f64; 3]) -> f64 {
        value_3d(point.into(), &self.perm_table)
    }
}

/// 4-dimensional value noise
impl NoiseFn<f64, 4> for Value {
    fn get(&self, point: [f64; 4]) -> f64 {
        value_4d(point.into(), &self.perm_table)
    }
}
