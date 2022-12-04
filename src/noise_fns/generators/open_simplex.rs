//! Note that this is NOT Ken Perlin's simplex noise, as that is patent encumbered.
//! Instead, these functions use the `OpenSimplex` algorithm, as detailed here:
//! <http://uniblock.tumblr.com/post/97868843242/noise>

use crate::{
    core::open_simplex::{open_simplex_2d, open_simplex_3d, open_simplex_4d},
    noise_fns::{NoiseFn, Seedable},
    permutationtable::PermutationTable,
};

/// Noise function that outputs 2/3/4-dimensional Open Simplex noise.
#[derive(Clone, Copy, Debug)]
pub struct OpenSimplex {
    seed: u32,
    perm_table: PermutationTable,
}

impl OpenSimplex {
    const DEFAULT_SEED: u32 = 0;

    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            perm_table: PermutationTable::new(seed),
        }
    }
}

impl Default for OpenSimplex {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl Seedable for OpenSimplex {
    /// Sets the seed value for Open Simplex noise
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

/// 2-dimensional [`OpenSimplex` Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than `Perlin` 2D.
impl NoiseFn<f64, 2> for OpenSimplex {
    fn get(&self, point: [f64; 2]) -> f64 {
        open_simplex_2d(point, &self.perm_table)
    }
}

/// 3-dimensional [`OpenSimplex` Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than `Perlin` 3D.
impl NoiseFn<f64, 3> for OpenSimplex {
    fn get(&self, point: [f64; 3]) -> f64 {
        open_simplex_3d(point, &self.perm_table)
    }
}

/// 4-dimensional [`OpenSimplex` Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than `Perlin` 4D.
impl NoiseFn<f64, 4> for OpenSimplex {
    fn get(&self, point: [f64; 4]) -> f64 {
        open_simplex_4d(point, &self.perm_table)
    }
}
