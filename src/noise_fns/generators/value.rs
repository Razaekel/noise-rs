use crate::{
    core::value::{value_2d, value_3d, value_4d},
    noise_fns::{NoiseFn, Seedable},
    permutationtable::PermutationTable,
};

/// Noise function that outputs 2/3/4-dimensional Value noise.
#[derive(Clone, Copy, Debug)]
pub struct Value {
    seed: u32,
    perm_table: PermutationTable,
}

impl Value {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            perm_table: PermutationTable::new(seed),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl Seedable for Value {
    /// Sets the seed value for Value noise
    fn set_seed(self, seed: u32) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Self {
            seed,
            perm_table: PermutationTable::new(seed),
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional value noise
impl NoiseFn<f64, 2> for Value {
    fn get(&self, point: [f64; 2]) -> f64 {
        value_2d(point, &self.perm_table)
    }
}

/// 3-dimensional value noise
impl NoiseFn<f64, 3> for Value {
    fn get(&self, point: [f64; 3]) -> f64 {
        value_3d(point, &self.perm_table)
    }
}

/// 4-dimensional value noise
impl NoiseFn<f64, 4> for Value {
    fn get(&self, point: [f64; 4]) -> f64 {
        value_4d(point, &self.perm_table)
    }
}
