//! Note that this is NOT Ken Perlin's simplex noise, as that is patent encumbered.
//! Instead, these functions use the `OpenSimplex` algorithm, as detailed here:
//! <http://uniblock.tumblr.com/post/97868843242/noise>

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    core::open_simplex::{open_simplex_2d, open_simplex_3d, open_simplex_4d},
    noise_fns::{NoiseFn, Seedable, DEFAULT_SEED},
    permutationtable::PermutationTable,
    Seed,
};

/// Noise function that outputs 2/3/4-dimensional Open Simplex noise.
#[derive(Clone, Copy, Debug)]
pub struct OpenSimplex {
    seed: Seed,
    perm_table: PermutationTable,
}

impl OpenSimplex {
    pub fn new(seed: Seed) -> Self {
        let mut rng = XorShiftRng::from_seed(seed);

        Self {
            seed,
            perm_table: rng.gen(),
        }
    }
}

impl Default for OpenSimplex {
    fn default() -> Self {
        Self::new(DEFAULT_SEED)
    }
}

impl Seedable for OpenSimplex {
    /// Sets the seed value for Open Simplex noise
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

/// 2-dimensional [`OpenSimplex` Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than `Perlin` 2D.
impl NoiseFn<f64, 2> for OpenSimplex {
    fn get(&self, point: [f64; 2]) -> f64 {
        open_simplex_2d(point.into(), &self.perm_table)
    }
}

/// 3-dimensional [`OpenSimplex` Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than `Perlin` 3D.
impl NoiseFn<f64, 3> for OpenSimplex {
    fn get(&self, point: [f64; 3]) -> f64 {
        open_simplex_3d(point.into(), &self.perm_table)
    }
}

/// 4-dimensional [`OpenSimplex` Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than `Perlin` 4D.
impl NoiseFn<f64, 4> for OpenSimplex {
    fn get(&self, point: [f64; 4]) -> f64 {
        open_simplex_4d(point.into(), &self.perm_table)
    }
}
