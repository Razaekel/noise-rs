use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    core::perlin_surflet::*,
    noise_fns::{NoiseFn, Seedable, DEFAULT_SEED},
    permutationtable::PermutationTable,
    Seed,
};

/// Noise function that outputs 2/3/4-dimensional Perlin noise.
///
/// THis is a variant of original perlin noise, based on the principles of simplex noise to
/// calculate the values at a point using wavelets instead of interpolated gradients.
#[derive(Clone, Copy, Debug)]
pub struct PerlinSurflet {
    seed: Seed,
    perm_table: PermutationTable,
}

impl PerlinSurflet {
    pub fn new(seed: Seed) -> Self {
        let mut rng = XorShiftRng::from_seed(seed);

        Self {
            seed,
            perm_table: rng.gen(),
        }
    }
}

impl Default for PerlinSurflet {
    fn default() -> Self {
        Self::new(DEFAULT_SEED)
    }
}

impl Seedable for PerlinSurflet {
    /// Sets the seed value for Perlin noise
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

/// 2-dimensional perlin noise
impl NoiseFn<f64, 2> for PerlinSurflet {
    fn get(&self, point: [f64; 2]) -> f64 {
        perlin_surflet_2d(point.into(), &self.perm_table)
    }
}

/// 3-dimensional perlin noise
impl NoiseFn<f64, 3> for PerlinSurflet {
    fn get(&self, point: [f64; 3]) -> f64 {
        perlin_surflet_3d(point.into(), &self.perm_table)
    }
}

/// 4-dimensional perlin noise
impl NoiseFn<f64, 4> for PerlinSurflet {
    fn get(&self, point: [f64; 4]) -> f64 {
        perlin_surflet_4d(point.into(), &self.perm_table)
    }
}
