use crate::{
    core::simplex::*,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::PermutationTable,
};

/// Noise function that outputs N-dimensional Simplex noise.
///
#[derive(Clone, Copy, Debug)]
pub struct Simplex {
    seed: u32,
    hasher: PermutationTable,
}

impl Simplex {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new(seed: u32) -> Self {
        Simplex {
            seed,
            hasher: PermutationTable::new(seed),
        }
    }
}

impl Default for Simplex {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl Seedable for Simplex {
    /// Sets the seed value for Simplex noise
    fn set_seed(self, seed: u32) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Simplex {
            seed,
            hasher: PermutationTable::new(seed),
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional Simplex noise
impl NoiseFn<f64, 2> for Simplex {
    fn get(&self, point: [f64; 2]) -> f64 {
        let (result, _) = simplex_2d(point, &self.hasher);

        result
    }
}

/// 3-dimensional Simplex noise
impl NoiseFn<f64, 3> for Simplex {
    fn get(&self, point: [f64; 3]) -> f64 {
        let (result, _) = simplex_3d(point, &self.hasher);

        result
    }
}

/// 4-dimensional Simplex noise
impl NoiseFn<f64, 4> for Simplex {
    fn get(&self, point: [f64; 4]) -> f64 {
        let (result, _) = simplex_4d(point, &self.hasher);

        result
    }
}
