//! Note that this is NOT Ken Perlin's simplex noise, as that is patent encumbered.
//! Instead, these functions use the `OpenSimplex` algorithm, as detailed here:
//! <http://uniblock.tumblr.com/post/97868843242/noise>

use crate::{
    gradient, math,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::{NoiseHasher, PermutationTable},
};
use std::ops::Add;

const STRETCH_CONSTANT_2D: f64 = -0.211_324_865_405_187; //(1/sqrt(2+1)-1)/2;
const SQUISH_CONSTANT_2D: f64 = 0.366_025_403_784_439; //(sqrt(2+1)-1)/2;
const STRETCH_CONSTANT_3D: f64 = -1.0 / 6.0; //(1/Math.sqrt(3+1)-1)/3;
const SQUISH_CONSTANT_3D: f64 = 1.0 / 3.0; //(Math.sqrt(3+1)-1)/3;
const STRETCH_CONSTANT_4D: f64 = -0.138_196_601_125_011; //(Math.sqrt(4+1)-1)/4;
const SQUISH_CONSTANT_4D: f64 = 0.309_016_994_374_947; //(Math.sqrt(4+1)-1)/4;

const NORM_CONSTANT_2D: f64 = 1.0 / 14.0;
const NORM_CONSTANT_3D: f64 = 1.0 / 14.0;
const NORM_CONSTANT_4D: f64 = 1.0 / 6.869_909_007_095_662_5;

/// Noise function that outputs 2/3/4-dimensional Open Simplex noise.
#[derive(Clone, Copy, Debug)]
pub struct OpenSimplex {
    seed: u32,
    perm_table: PermutationTable,
}

impl OpenSimplex {
    const DEFAULT_SEED: u32 = 0;

    pub fn new() -> Self {
        Self {
            seed: Self::DEFAULT_SEED,
            perm_table: PermutationTable::new(Self::DEFAULT_SEED),
        }
    }
}

impl Default for OpenSimplex {
    fn default() -> Self {
        Self::new()
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
        fn gradient(perm_table: &PermutationTable, vertex: [f64; 2], pos: [f64; 2]) -> f64 {
            let attn = 2.0 - math::dot2(pos, pos);
            if attn > 0.0 {
                let index = perm_table.hash(&math::to_isize2(vertex));
                let vec = gradient::grad2(index);
                attn.powi(4) * math::dot2(pos, vec)
            } else {
                0.0
            }
        }

        // Place input coordinates onto grid.
        let stretch_offset = math::fold2(point, Add::add) * STRETCH_CONSTANT_2D;
        let stretched = math::map2(point, |v| v + stretch_offset);

        // Floor to get grid coordinates of rhombus (stretched square) cell origin.
        let stretched_floor = math::map2(stretched, f64::floor);

        // Skew out to get actual coordinates of rhombus origin. We'll need these later.
        let squish_offset = math::fold2(stretched_floor, Add::add) * SQUISH_CONSTANT_2D;
        let skewed_floor = math::map2(stretched_floor, |v| v + squish_offset);

        // Compute grid coordinates relative to rhombus origin.
        let rel_coords = math::sub2(stretched, stretched_floor);

        // Sum those together to get a value that determines which region we're in.
        let region_sum = math::fold2(rel_coords, Add::add);

        // Positions relative to origin point (0, 0).
        let pos0 = math::sub2(point, skewed_floor);

        let mut value = 0.0;

        let mut vertex;
        let mut dpos;

        // (0, 0) --- (1, 0)
        // |   A     /     |
        // |       /       |
        // |     /     B   |
        // (0, 1) --- (1, 1)

        let t0 = SQUISH_CONSTANT_2D;
        let t1 = SQUISH_CONSTANT_2D + 1.0;
        let t2 = SQUISH_CONSTANT_2D + t1;

        // Contribution (1, 0)
        vertex = math::add2(stretched_floor, [1.0, 0.0]);
        dpos = math::sub2(pos0, [t1, t0]);
        value += gradient(&self.perm_table, vertex, dpos);

        // Contribution (0, 1)
        vertex = math::add2(stretched_floor, [0.0, 1.0]);
        dpos = math::sub2(pos0, [t0, t1]);
        value += gradient(&self.perm_table, vertex, dpos);

        // See the graph for an intuitive explanation; the sum of `x` and `y` is
        // only greater than `1` if we're on Region B.
        if region_sum > 1.0 {
            // Contribution (1, 1)
            vertex = math::add2(stretched_floor, [1.0, 1.0]);
            // We are moving across the diagonal `/`, so we'll need to add by the
            // squish constant
            dpos = math::sub2(pos0, [t2, t2]);
        } else {
            vertex = math::add2(stretched_floor, [0.0, 0.0]);
            dpos = math::sub2(pos0, [0.0, 0.0]);
        }

        // Point (0, 0) or (1, 1)
        value += gradient(&self.perm_table, vertex, dpos);

        value * NORM_CONSTANT_2D
    }
}

/// 3-dimensional [`OpenSimplex` Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than `Perlin` 3D.
impl NoiseFn<f64, 3> for OpenSimplex {
    fn get(&self, point: [f64; 3]) -> f64 {
        fn gradient(perm_table: &PermutationTable, vertex: [f64; 3], pos: [f64; 3]) -> f64 {
            let attn = 2.0 - math::dot3(pos, pos);
            if attn > 0.0 {
                let index = perm_table.hash(&math::to_isize3(vertex));
                let vec = gradient::grad3(index);
                attn.powi(4) * math::dot3(pos, vec)
            } else {
                0.0
            }
        }

        // Place input coordinates on simplectic h1.0ycomb.
        let stretch_offset = math::fold3(point, Add::add) * STRETCH_CONSTANT_3D;
        let stretched = math::map3(point, |v| v + stretch_offset);

        // Floor to get simplectic h1.0ycomb coordinates of rhombohedron
        // (stretched cube) super-cell origin.
        let stretched_floor = math::map3(stretched, f64::floor);

        // Skew out to get actual coordinates of rhombohedron origin. We'll need
        // these later.
        let squish_offset = math::fold3(stretched_floor, Add::add) * SQUISH_CONSTANT_3D;
        let skewed_floor = math::map3(stretched_floor, |v| v + squish_offset);

        // Compute simplectic h1.0ycomb coordinates relative to rhombohedral origin.
        let rel_coords = math::sub3(stretched, stretched_floor);

        // Sum those together to get a value that determines which region we're in.
        let region_sum = math::fold3(rel_coords, Add::add);

        // Positions relative to origin point.
        let pos0 = math::sub3(point, skewed_floor);

        let mut value = 0.0;

        let mut vertex;
        let mut dpos;

        if region_sum <= 1.0 {
            // We're inside the tetrahedron (3-Simplex) at (0, 0, 0)
            let t0 = SQUISH_CONSTANT_3D;
            let t1 = SQUISH_CONSTANT_2D + 1.0;

            // Contribution at (0, 0, 0)
            vertex = math::add3(stretched_floor, [0.0, 0.0, 0.0]);
            dpos = math::sub3(pos0, [0.0, 0.0, 0.0]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (1, 0, 0)
            vertex = math::add3(stretched_floor, [1.0, 0.0, 0.0]);
            dpos = math::sub3(pos0, [t1, t0, t0]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 1, 0)
            vertex = math::add3(stretched_floor, [0.0, 1.0, 0.0]);
            dpos = math::sub3(pos0, [t0, t1, t0]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 0, 1)
            vertex = math::add3(stretched_floor, [0.0, 0.0, 1.0]);
            dpos = math::sub3(pos0, [t0, t0, t1]);
            value += gradient(&self.perm_table, vertex, dpos);
        } else if region_sum >= 2.0 {
            // We're inside the tetrahedron (3-Simplex) at (1, 1, 1)
            let t0 = 2.0 * SQUISH_CONSTANT_3D;
            let t1 = 1.0 + 2.0 * SQUISH_CONSTANT_3D;
            let t2 = t1 + SQUISH_CONSTANT_3D;

            // Contribution at (1, 1, 0)
            vertex = math::add3(stretched_floor, [1.0, 1.0, 0.0]);
            dpos = math::sub3(pos0, [t1, t1, t0]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (1, 0, 1)
            vertex = math::add3(stretched_floor, [1.0, 0.0, 1.0]);
            dpos = math::sub3(pos0, [t1, t0, t1]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 1, 1)
            vertex = math::add3(stretched_floor, [0.0, 1.0, 1.0]);
            dpos = math::sub3(pos0, [t0, t1, t1]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (1, 1, 1)
            vertex = math::add3(stretched_floor, [1.0, 1.0, 1.0]);
            dpos = math::sub3(pos0, [t2, t2, t2]);
            value += gradient(&self.perm_table, vertex, dpos);
        } else {
            // We're inside the octahedron (Rectified 3-Simplex) inbetween.
            let t0 = SQUISH_CONSTANT_3D;
            let t1 = 1.0 + SQUISH_CONSTANT_3D;
            let t2 = 2.0 * SQUISH_CONSTANT_3D;
            let t3 = 1.0 + 2.0 * SQUISH_CONSTANT_3D;

            // Contribution at (1, 0, 0)
            vertex = math::add3(stretched_floor, [1.0, 0.0, 0.0]);
            dpos = math::sub3(pos0, [t1, t0, t0]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 1, 0)
            vertex = math::add3(stretched_floor, [0.0, 1.0, 0.0]);
            dpos = math::sub3(pos0, [t0, t1, t0]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 0, 1)
            vertex = math::add3(stretched_floor, [0.0, 0.0, 1.0]);
            dpos = math::sub3(pos0, [t0, t0, t1]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (1, 1, 0)
            vertex = math::add3(stretched_floor, [1.0, 1.0, 0.0]);
            dpos = math::sub3(pos0, [t3, t3, t2]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (1, 0, 1)
            vertex = math::add3(stretched_floor, [1.0, 0.0, 1.0]);
            dpos = math::sub3(pos0, [t3, t2, t3]);
            value += gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 1, 1)
            vertex = math::add3(stretched_floor, [0.0, 1.0, 1.0]);
            dpos = math::sub3(pos0, [t2, t3, t3]);
            value += gradient(&self.perm_table, vertex, dpos);
        }

        value * NORM_CONSTANT_3D
    }
}

/// 4-dimensional [`OpenSimplex` Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than `Perlin` 4D.
impl NoiseFn<f64, 4> for OpenSimplex {
    fn get(&self, point: [f64; 4]) -> f64 {
        #[inline(always)]
        fn gradient(perm_table: &PermutationTable, vertex: [f64; 4], pos: [f64; 4]) -> f64 {
            let attn = 2.0 - math::dot4(pos, pos);
            if attn > 0.0 {
                let index = perm_table.hash(&math::to_isize4(vertex));
                let vec = gradient::grad4(index);
                attn.powi(4) * math::dot4(pos, vec)
            } else {
                0.0
            }
        }

        // Place input coordinates on simplectic h1.0ycomb.
        let stretch_offset = math::fold4(point, Add::add) * STRETCH_CONSTANT_4D;
        let stretched = math::map4(point, |v| v + stretch_offset);

        // Floor to get simplectic h1.0ycomb coordinates of rhombo-hypercube
        // super-cell origin.
        let stretched_floor = math::map4(stretched, f64::floor);

        // Skew out to get actual coordinates of stretched rhombo-hypercube origin.
        // We'll need these later.
        let squish_offset = math::fold4(stretched_floor, Add::add) * SQUISH_CONSTANT_4D;
        let skewed_floor = math::map4(stretched_floor, |v| v + squish_offset);

        // Compute simplectic h1.0ycomb coordinates relative to rhombo-hypercube
        // origin.
        let rel_coords = math::sub4(stretched, stretched_floor);

        // Sum those together to get a value that determines which region
        // we're in.
        let region_sum = math::fold4(rel_coords, Add::add);

        // Position relative to origin point.
        let mut pos0 = math::sub4(point, skewed_floor);

        let mut value = 0.0;
        if region_sum <= 1.0 {
            // We're inside the pentachoron (4-Simplex) at (0, 0, 0, 0)

            // Contribution at (0, 0, 0, 0)
            value += gradient(&self.perm_table, stretched_floor, pos0);

            // Contribution at (1, 0, 0, 0)
            let pos1;
            {
                let vertex = math::add4(stretched_floor, [1.0, 0.0, 0.0, 0.0]);
                pos1 = math::sub4(
                    pos0,
                    [
                        1.0 + SQUISH_CONSTANT_4D,
                        SQUISH_CONSTANT_4D,
                        SQUISH_CONSTANT_4D,
                        SQUISH_CONSTANT_4D,
                    ],
                );
                value += gradient(&self.perm_table, vertex, pos1);
            }

            // Contribution at (0, 1, 0, 0)
            let pos2;
            {
                let vertex = math::add4(stretched_floor, [0.0, 1.0, 0.0, 0.0]);
                pos2 = [pos1[0] + 1.0, pos1[1] - 1.0, pos1[2], pos1[3]];
                value += gradient(&self.perm_table, vertex, pos2);
            }

            // Contribution at (0, 0, 1, 0)
            let pos3;
            {
                let vertex = math::add4(stretched_floor, [0.0, 0.0, 1.0, 0.0]);
                pos3 = [pos2[0], pos1[1], pos1[2] - 1.0, pos1[3]];
                value += gradient(&self.perm_table, vertex, pos3);
            }

            // Contribution at (0, 0, 0, 1)
            let pos4;
            {
                let vertex = math::add4(stretched_floor, [0.0, 0.0, 0.0, 1.0]);
                pos4 = [pos2[0], pos1[1], pos1[2], pos1[3] - 1.0];
                value += gradient(&self.perm_table, vertex, pos4);
            }
        } else if region_sum >= 3.0 {
            // We're inside the pentachoron (4-Simplex) at (1, 1, 1, 1)
            let squish_constant_3 = 3.0 * SQUISH_CONSTANT_4D;

            // Contribution at (1, 1, 1, 0)
            let pos4;
            {
                let vertex = math::add4(stretched_floor, [1.0, 1.0, 1.0, 0.0]);
                pos4 = math::sub4(
                    pos0,
                    [
                        1.0 + squish_constant_3,
                        1.0 + squish_constant_3,
                        1.0 + squish_constant_3,
                        squish_constant_3,
                    ],
                );
                value += gradient(&self.perm_table, vertex, pos4);
            }

            // Contribution at (1, 1, 0, 1)
            let pos3;
            {
                let vertex = math::add4(stretched_floor, [1.0, 1.0, 0.0, 1.0]);
                pos3 = [pos4[0], pos4[1], pos4[2] + 1.0, pos4[3] - 1.0];
                value += gradient(&self.perm_table, vertex, pos3);
            }

            // Contribution at (1, 0, 1, 1)
            let pos2;
            {
                let vertex = math::add4(stretched_floor, [1.0, 0.0, 1.0, 1.0]);
                pos2 = [pos4[0], pos4[1] + 1.0, pos4[2], pos3[3]];
                value += gradient(&self.perm_table, vertex, pos2);
            }

            // Contribution at (0, 1, 1, 1)
            let pos1;
            {
                let vertex = math::add4(stretched_floor, [0.0, 1.0, 1.0, 1.0]);
                pos1 = [pos0[0] - squish_constant_3, pos4[1], pos4[2], pos3[3]];
                value += gradient(&self.perm_table, vertex, pos1);
            }

            // Contribution at (1, 1, 1, 1)
            {
                let vertex = math::add4(stretched_floor, [1.0, 1.0, 1.0, 1.0]);
                pos0[0] = pos4[0] - SQUISH_CONSTANT_4D;
                pos0[1] = pos4[1] - SQUISH_CONSTANT_4D;
                pos0[2] = pos4[2] - SQUISH_CONSTANT_4D;
                pos0[3] = pos3[3] - SQUISH_CONSTANT_4D;
                value += gradient(&self.perm_table, vertex, pos0);
            }
        } else if region_sum <= 2.0 {
            // We're inside the first dispentachoron (Rectified 4-Simplex)

            // Contribution at (1, 0, 0, 0)
            let pos1;
            {
                let vertex = math::add4(stretched_floor, [1.0, 0.0, 0.0, 0.0]);
                pos1 = math::sub4(
                    pos0,
                    [
                        1.0 + SQUISH_CONSTANT_4D,
                        SQUISH_CONSTANT_4D,
                        SQUISH_CONSTANT_4D,
                        SQUISH_CONSTANT_4D,
                    ],
                );
                value += gradient(&self.perm_table, vertex, pos1);
            }

            // Contribution at (0, 1, 0, 0)
            let pos2;
            {
                let vertex = math::add4(stretched_floor, [0.0, 1.0, 0.0, 0.0]);
                pos2 = [pos1[0] + 1.0, pos1[1] - 1.0, pos1[2], pos1[3]];
                value += gradient(&self.perm_table, vertex, pos2);
            }

            // Contribution at (0, 0, 1, 0)
            let pos3;
            {
                let vertex = math::add4(stretched_floor, [0.0, 0.0, 1.0, 0.0]);
                pos3 = [pos2[0], pos1[1], pos1[2] - 1.0, pos1[3]];
                value += gradient(&self.perm_table, vertex, pos3);
            }

            // Contribution at (0, 0, 0, 1)
            let pos4;
            {
                let vertex = math::add4(stretched_floor, [0.0, 0.0, 0.0, 1.0]);
                pos4 = [pos2[0], pos1[1], pos1[2], pos1[3] - 1.0];
                value += gradient(&self.perm_table, vertex, pos4);
            }

            // Contribution at (1, 1, 0, 0)
            let pos5;
            {
                let vertex = math::add4(stretched_floor, [1.0, 1.0, 0.0, 0.0]);
                pos5 = [
                    pos1[0] - SQUISH_CONSTANT_4D,
                    pos2[1] - SQUISH_CONSTANT_4D,
                    pos1[2] - SQUISH_CONSTANT_4D,
                    pos1[3] - SQUISH_CONSTANT_4D,
                ];
                value += gradient(&self.perm_table, vertex, pos5);
            }

            // Contribution at (1, 0, 1, 0)
            let pos6;
            {
                let vertex = math::add4(stretched_floor, [1.0, 0.0, 1.0, 0.0]);
                pos6 = [pos5[0], pos5[1] + 1.0, pos5[2] - 1.0, pos5[3]];
                value += gradient(&self.perm_table, vertex, pos6);
            }

            // Contribution at (1, 0, 0, 1)
            let pos7;
            {
                let vertex = math::add4(stretched_floor, [1.0, 0.0, 0.0, 1.0]);
                pos7 = [pos5[0], pos6[1], pos5[2], pos5[3] - 1.0];
                value += gradient(&self.perm_table, vertex, pos7);
            }

            // Contribution at (0, 1, 1, 0)
            let pos8;
            {
                let vertex = math::add4(stretched_floor, [0.0, 1.0, 1.0, 0.0]);
                pos8 = [pos5[0] + 1.0, pos5[1], pos6[2], pos5[3]];
                value += gradient(&self.perm_table, vertex, pos8);
            }

            // Contribution at (0, 1, 0, 1)
            let pos9;
            {
                let vertex = math::add4(stretched_floor, [0.0, 1.0, 0.0, 1.0]);
                pos9 = [pos8[0], pos5[1], pos5[2], pos7[3]];
                value += gradient(&self.perm_table, vertex, pos9);
            }

            // Contribution at (0, 0, 1, 1)
            let pos10;
            {
                let vertex = math::add4(stretched_floor, [0.0, 0.0, 1.0, 1.0]);
                pos10 = [pos8[0], pos6[1], pos6[2], pos7[3]];
                value += gradient(&self.perm_table, vertex, pos10);
            }
        } else {
            // We're inside the second dispentachoron (Rectified 4-Simplex)
            let squish_constant_3 = 3.0 * SQUISH_CONSTANT_4D;

            // Contribution at (1, 1, 1, 0)
            let pos4;
            {
                let vertex = math::add4(stretched_floor, [1.0, 1.0, 1.0, 0.0]);
                pos4 = math::sub4(
                    pos0,
                    [
                        1.0 + squish_constant_3,
                        1.0 + squish_constant_3,
                        1.0 + squish_constant_3,
                        squish_constant_3,
                    ],
                );
                value += gradient(&self.perm_table, vertex, pos4);
            }

            // Contribution at (1, 1, 0, 1)
            let pos3;
            {
                let vertex = math::add4(stretched_floor, [1.0, 1.0, 0.0, 1.0]);
                pos3 = [pos4[0], pos4[1], pos4[2] + 1.0, pos4[3] - 1.0];
                value += gradient(&self.perm_table, vertex, pos3);
            }

            // Contribution at (1, 0, 1, 1)
            let pos2;
            {
                let vertex = math::add4(stretched_floor, [1.0, 0.0, 1.0, 1.0]);
                pos2 = [pos4[0], pos4[1] + 1.0, pos4[2], pos3[3]];
                value += gradient(&self.perm_table, vertex, pos2);
            }

            // Contribution at (0, 1, 1, 1)
            let pos1;
            {
                let vertex = math::add4(stretched_floor, [0.0, 1.0, 1.0, 1.0]);
                pos1 = [pos4[0] + 1.0, pos4[1], pos4[2], pos3[3]];
                value += gradient(&self.perm_table, vertex, pos1);
            }

            // Contribution at (1, 1, 0, 0)
            let pos5;
            {
                let vertex = math::add4(stretched_floor, [1.0, 1.0, 0.0, 0.0]);
                pos5 = [
                    pos4[0] + SQUISH_CONSTANT_4D,
                    pos4[1] + SQUISH_CONSTANT_4D,
                    pos3[2] + SQUISH_CONSTANT_4D,
                    pos4[3] + SQUISH_CONSTANT_4D,
                ];
                value += gradient(&self.perm_table, vertex, pos5);
            }

            // Contribution at (1, 0, 1, 0)
            let pos6;
            {
                let vertex = math::add4(stretched_floor, [1.0, 0.0, 1.0, 0.0]);
                pos6 = [pos5[0], pos5[1] + 1.0, pos5[2] - 1.0, pos5[3]];
                value += gradient(&self.perm_table, vertex, pos6);
            }

            // Contribution at (1, 0, 0, 1)
            let pos7;
            {
                let vertex = math::add4(stretched_floor, [1.0, 0.0, 0.0, 1.0]);
                pos7 = [pos5[0], pos6[1], pos5[2], pos5[3] - 1.0];
                value += gradient(&self.perm_table, vertex, pos7);
            }

            // Contribution at (0, 1, 1, 0)
            let pos8;
            {
                let vertex = math::add4(stretched_floor, [0.0, 1.0, 1.0, 0.0]);
                pos8 = [pos5[0] + 1.0, pos5[1], pos6[2], pos5[3]];
                value += gradient(&self.perm_table, vertex, pos8);
            }

            // Contribution at (0, 1, 0, 1)
            let pos9;
            {
                let vertex = math::add4(stretched_floor, [0.0, 1.0, 0.0, 1.0]);
                pos9 = [pos8[0], pos5[1], pos5[2], pos7[3]];
                value += gradient(&self.perm_table, vertex, pos9);
            }

            // Contribution at (0, 0, 1, 1)
            let pos10;
            {
                let vertex = math::add4(stretched_floor, [0.0, 0.0, 1.0, 1.0]);
                pos10 = [pos8[0], pos6[1], pos6[2], pos7[3]];
                value += gradient(&self.perm_table, vertex, pos10);
            }
        }

        value * NORM_CONSTANT_4D
    }
}
