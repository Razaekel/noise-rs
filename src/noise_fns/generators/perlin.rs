use crate::{
    core::perlin::*,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::PermutationTable,
};

/// Noise function that outputs 2/3/4-dimensional Perlin noise.
#[derive(Clone, Copy, Debug)]
pub struct Perlin {
    seed: u32,
    perm_table: PermutationTable,
}

impl Perlin {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            perm_table: PermutationTable::new(seed),
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl Seedable for Perlin {
    /// Sets the seed value for Perlin noise
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

/// 2-dimensional perlin noise
impl NoiseFn<f32, 2> for Perlin {
    fn get(&self, point: [f32; 2]) -> f32 {
        perlin_2d_f32(point, &self.perm_table)
    }
}

/// 3-dimensional perlin noise
impl NoiseFn<f32, 3> for Perlin {
    fn get(&self, point: [f32; 3]) -> f32 {
        perlin_3d_f32(point, &self.perm_table)
    }
}

/// 4-dimensional perlin noise
impl NoiseFn<f32, 4> for Perlin {
    fn get(&self, point: [f32; 4]) -> f32 {
        perlin_4d_f32(point, &self.perm_table)
    }
}

/// 2-dimensional perlin noise
impl NoiseFn<f64, 2> for Perlin {
    fn get(&self, point: [f64; 2]) -> f64 {
        perlin_2d_f64(point, &self.perm_table)
    }
}

/// 3-dimensional perlin noise
impl NoiseFn<f64, 3> for Perlin {
    fn get(&self, point: [f64; 3]) -> f64 {
        perlin_3d_f64(point, &self.perm_table)
    }
}

/// 4-dimensional perlin noise
impl NoiseFn<f64, 4> for Perlin {
    fn get(&self, point: [f64; 4]) -> f64 {
        perlin_4d_f64(point, &self.perm_table)
    }
}
