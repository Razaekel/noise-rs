use crate::{
    core::super_simplex::*,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::PermutationTable,
};

/// Noise function that outputs 2/3-dimensional Super Simplex noise.
#[derive(Clone, Copy, Debug)]
pub struct SuperSimplex {
    seed: u32,
    perm_table: PermutationTable,
}

impl SuperSimplex {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            perm_table: PermutationTable::new(seed),
        }
    }
}

impl Default for SuperSimplex {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl Seedable for SuperSimplex {
    /// Sets the seed value for Super Simplex noise
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

/// 2-dimensional Super Simplex noise
impl NoiseFn<f64, 2> for SuperSimplex {
    fn get(&self, point: [f64; 2]) -> f64 {
        super_simplex_2d(point, &self.perm_table)
    }
}

/// 3-dimensional Super Simplex noise
impl NoiseFn<f64, 3> for SuperSimplex {
    fn get(&self, point: [f64; 3]) -> f64 {
        super_simplex_3d(point, &self.perm_table)
    }
}
