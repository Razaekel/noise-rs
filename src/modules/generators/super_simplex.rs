// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use {PermutationTable, gradient, math};
use math::{Point2, Point3};
use modules::{NoiseModule, Seedable};
use num_traits::Float;
use std::ops::Add;

/// Default Seed for the Super Simplex noise module.
pub const DEFAULT_SUPER_SIMPLEX_SEED: u32 = 0;

const TO_REAL_CONSTANT_2D: f64 = -0.211324865405187; // (1 / sqrt(2 + 1) - 1) / 2
const TO_SIMPLEX_CONSTANT_2D: f64 = 0.366025403784439; // (sqrt(2 + 1) - 1) / 2
const TO_SIMPLEX_CONSTANT_3D: f64 = -2.0 / 3.0;

// Determined using the Mathematica code listed in the super_simplex example and find_maximum_super_simplex.nb
const NORM_CONSTANT_2D: f64 = 1.0 / 0.05428295288661623;
const NORM_CONSTANT_3D: f64 = 1.0 / 0.0867664001655369;

// Points taken into account for 2D:
//              (-1,  0)
//                 |    \
//                 |      \
//                 |        \
// ( 0, -1) --- ( 0,  0) --- ( 1,  0)
//         \       |    \       |    \
//           \     |      \     |      \
//             \   |        \   |        \
//              ( 0,  1) --- ( 1,  1) --- ( 2,  1)
//                      \       |
//                        \     |
//                          \   |
//                           ( 1,  2)

const LATTICE_LOOKUP_2D: [([i8; 2], [f64; 2]); 4 * 8] =
    [([0, 0], [0f64, 0f64]),
     ([1, 1], [-0.577350269189626f64, -0.577350269189626f64]),
     ([-1, 0], [0.788675134594813f64, -0.211324865405187f64]),
     ([0, -1], [-0.211324865405187f64, 0.788675134594813f64]),

     ([0, 0], [0f64, 0f64]),
     ([1, 1], [-0.577350269189626f64, -0.577350269189626f64]),
     ([0, 1], [0.211324865405187f64, -0.788675134594813f64]),
     ([1, 0], [-0.788675134594813f64, 0.211324865405187f64]),

     ([0, 0], [0f64, 0f64]),
     ([1, 1], [-0.577350269189626f64, -0.577350269189626f64]),
     ([1, 0], [-0.788675134594813f64, 0.211324865405187f64]),
     ([0, -1], [-0.211324865405187f64, 0.788675134594813f64]),

     ([0, 0], [0f64, 0f64]),
     ([1, 1], [-0.577350269189626f64, -0.577350269189626f64]),
     ([2, 1], [-1.366025403784439f64, -0.36602540378443904f64]),
     ([1, 0], [-0.788675134594813f64, 0.211324865405187f64]),

     ([0, 0], [0f64, 0f64]),
     ([1, 1], [-0.577350269189626f64, -0.577350269189626f64]),
     ([-1, 0], [0.788675134594813f64, -0.211324865405187f64]),
     ([0, 1], [0.211324865405187f64, -0.788675134594813f64]),

     ([0, 0], [0f64, 0f64]),
     ([1, 1], [-0.577350269189626f64, -0.577350269189626f64]),
     ([0, 1], [0.211324865405187f64, -0.788675134594813f64]),
     ([1, 2], [-0.36602540378443904f64, -1.366025403784439f64]),

     ([0, 0], [0f64, 0f64]),
     ([1, 1], [-0.577350269189626f64, -0.577350269189626f64]),
     ([1, 0], [-0.788675134594813f64, 0.211324865405187f64]),
     ([0, 1], [0.211324865405187f64, -0.788675134594813f64]),

     ([0, 0], [0f64, 0f64]),
     ([1, 1], [-0.577350269189626f64, -0.577350269189626f64]),
     ([2, 1], [-1.366025403784439f64, -0.36602540378443904f64]),
     ([1, 2], [-0.36602540378443904f64, -1.366025403784439f64])];
const LATTICE_LOOKUP_3D: [[i8; 3]; 4 * 16] =
    [[0, 0, 0],[1, 0, 0],[0, 1, 0],[0, 0, 1],
     [1, 1, 1],[1, 0, 0],[0, 1, 0],[0, 0, 1],
     [0, 0, 0],[0, 1, 1],[0, 1, 0],[0, 0, 1],
     [1, 1, 1],[0, 1, 1],[0, 1, 0],[0, 0, 1],
     [0, 0, 0],[1, 0, 0],[1, 0, 1],[0, 0, 1],
     [1, 1, 1],[1, 0, 0],[1, 0, 1],[0, 0, 1],
     [0, 0, 0],[0, 1, 1],[1, 0, 1],[0, 0, 1],
     [1, 1, 1],[0, 1, 1],[1, 0, 1],[0, 0, 1],
     [0, 0, 0],[1, 0, 0],[0, 1, 0],[1, 1, 0],
     [1, 1, 1],[1, 0, 0],[0, 1, 0],[1, 1, 0],
     [0, 0, 0],[0, 1, 1],[0, 1, 0],[1, 1, 0],
     [1, 1, 1],[0, 1, 1],[0, 1, 0],[1, 1, 0],
     [0, 0, 0],[1, 0, 0],[1, 0, 1],[1, 1, 0],
     [1, 1, 1],[1, 0, 0],[1, 0, 1],[1, 1, 0],
     [0, 0, 0],[0, 1, 1],[1, 0, 1],[1, 1, 0],
     [1, 1, 1],[0, 1, 1],[1, 0, 1],[1, 1, 0]];

/// Noise module that outputs 2/3-dimensional Super Simplex noise.
#[derive(Clone, Copy, Debug)]
pub struct SuperSimplex {
    seed: u32,
    perm_table: PermutationTable,
}

impl SuperSimplex {
    pub fn new() -> SuperSimplex {
        SuperSimplex {
            seed: DEFAULT_SUPER_SIMPLEX_SEED,
            perm_table: PermutationTable::new(DEFAULT_SUPER_SIMPLEX_SEED),
        }
    }
}

impl Seedable for SuperSimplex {
    /// Sets the seed value for Super Simplex noise
    fn set_seed(self, seed: u32) -> SuperSimplex {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }
        // Otherwise, regenerate the permutation table based on the new seed.
        SuperSimplex {
            perm_table: PermutationTable::new(seed),
            .. self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional Super Simplex noise
impl<T: Float> NoiseModule<Point2<T>> for SuperSimplex {
    type Output = T;

    fn get(&self, point: Point2<T>) -> T {
        let zero: T = math::cast(0.0);
        let one: T = math::cast(1.0);
        let one_half: T = math::cast(0.5);
        let two: T = math::cast(2.0);
        let two_thirds: T = math::cast(2.0 / 3.0);
        let to_real_constant: T = math::cast(TO_REAL_CONSTANT_2D);
        let to_simplex_constant: T = math::cast(TO_SIMPLEX_CONSTANT_2D);
        let norm: T = math::cast(NORM_CONSTANT_2D);

        let mut value = zero;

        // Transform point from real space to simplex space
        let to_simplex_offset = math::fold2(point, Add::add) * to_simplex_constant;
        let simplex_point = math::map2(point, |v| v + to_simplex_offset);

        // Get base point of simplex and barycentric coordinates in simplex space
        let simplex_base_point = math::map2(simplex_point, Float::floor);
        let simplex_base_point_i = math::cast2::<_, isize>(simplex_base_point);
        let simplex_rel_coords = math::sub2(simplex_point, simplex_base_point);

        // Create index to lookup table from barycentric coordinates
        let region_sum = math::fold2(simplex_rel_coords, Add::add).floor();
        let index = ((region_sum >= one) as usize) << 2 |
        ((simplex_rel_coords[0] - simplex_rel_coords[1] * one_half + one - region_sum * one_half >= one) as usize) << 3 |
        ((simplex_rel_coords[1] - simplex_rel_coords[0] * one_half + one - region_sum * one_half >= one) as usize) << 4;

        // Transform barycentric coordinates to real space
        let to_real_offset = math::fold2(simplex_rel_coords, Add::add) * to_real_constant;
        let real_rel_coords = math::map2(simplex_rel_coords, |v| v + to_real_offset);

        for lattice_lookup in &LATTICE_LOOKUP_2D[index..index + 4] {
            let dpos = math::add2(real_rel_coords, math::cast2(lattice_lookup.1));
            let attn = two_thirds - math::dot2(dpos, dpos);
            if attn <= zero {
                continue;
            }

            let lattice_point = math::add2(simplex_base_point_i, math::cast2(lattice_lookup.0));
            let gradient = gradient::get2(self.perm_table.get2(lattice_point));
            value = value + math::pow4(attn) * math::dot2(gradient, dpos);
        }

        value * norm
    }
}

/// 3-dimensional Super Simplex noise
impl<T: Float> NoiseModule<Point3<T>> for SuperSimplex {
    type Output = T;

    fn get(&self, point: Point3<T>) -> T {
        let zero: T = math::cast(0.0);
        let one: T = math::cast(1.0);
        let one_p_five: T = math::cast(1.5);
        let one_half: T = math::cast(0.5);
        let two: T = math::cast(2.0);
        let three_fourths: T = math::cast(0.75);
        let overlapping_offset: T = math::cast(512.5);
        let to_simplex_constant: T = math::cast(TO_SIMPLEX_CONSTANT_3D);
        let norm: T = math::cast(NORM_CONSTANT_3D);

        let mut value = zero;

        // Transform point from real space to simplex space
        let to_simplex_offset = math::fold3(point, Add::add) * to_simplex_constant;
        let simplex_point = math::map3(point, |v| -(v + to_simplex_offset));
        let second_simplex_point = math::map3(simplex_point, |v| v + overlapping_offset);

        // Get base point of simplex and barycentric coordinates in simplex space
        let simplex_base_point = math::map3(simplex_point, Float::floor);
        let simplex_base_point_i = math::cast3::<_, isize>(simplex_base_point);
        let simplex_rel_coords = math::sub3(simplex_point, simplex_base_point);
        let second_simplex_base_point = math::map3(second_simplex_point, Float::floor);
        let second_simplex_base_point_i = math::cast3::<_, isize>(second_simplex_base_point);
        let second_simplex_rel_coords = math::sub3(second_simplex_point, second_simplex_base_point);

        // Create indices to lookup table from barycentric coordinates
        let index = ((simplex_rel_coords[0] + simplex_rel_coords[1] + simplex_rel_coords[2] >= one_p_five) as usize) << 2 |
        ((-simplex_rel_coords[0] + simplex_rel_coords[1] + simplex_rel_coords[2] >= one_half) as usize) << 3 |
        ((simplex_rel_coords[0] - simplex_rel_coords[1] + simplex_rel_coords[2] >= one_half) as usize) << 4 |
        ((simplex_rel_coords[0] + simplex_rel_coords[1] - simplex_rel_coords[2] >= one_half) as usize) << 5;
        let second_index = ((second_simplex_rel_coords[0] + second_simplex_rel_coords[1] + second_simplex_rel_coords[2] >= one_p_five) as usize) << 2 |
        ((-second_simplex_rel_coords[0] + second_simplex_rel_coords[1] + second_simplex_rel_coords[2] >= one_half) as usize) << 3 |
        ((second_simplex_rel_coords[0] - second_simplex_rel_coords[1] + second_simplex_rel_coords[2] >= one_half) as usize) << 4 |
        ((second_simplex_rel_coords[0] + second_simplex_rel_coords[1] - second_simplex_rel_coords[2] >= one_half) as usize) << 5;

        // Sum contributions from first lattice
        for &lattice_lookup in &LATTICE_LOOKUP_3D[index..index + 4] {
            let dpos = math::sub3(simplex_rel_coords, math::cast3(lattice_lookup));
            let attn = three_fourths - math::dot3(dpos, dpos);
            if attn <= zero {
                continue;
            }

            let lattice_point = math::add3(simplex_base_point_i, math::cast3(lattice_lookup));
            let gradient = gradient::get3(self.perm_table.get3(lattice_point));
            value = value + math::pow4(attn) * math::dot3(gradient, dpos);
        }

        // Sum contributions from second lattice
        for &lattice_lookup in &LATTICE_LOOKUP_3D[second_index..second_index + 4] {
            let dpos = math::sub3(second_simplex_rel_coords, math::cast3(lattice_lookup));
            let attn = three_fourths - math::dot3(dpos, dpos);
            if attn <= zero {
                continue;
            }

            let lattice_point = math::add3(second_simplex_base_point_i, math::cast3(lattice_lookup));
            let gradient = gradient::get3(self.perm_table.get3(lattice_point));
            value = value + math::pow4(attn) * math::dot3(gradient, dpos);
        }

        value * norm
    }
}
