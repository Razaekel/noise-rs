// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use {gradient, math};
use math::{Point2, Point3};
use noise_fns::{NoiseFn, Seedable};
use permutationtable::PermutationTable;
use std::ops::Add;

/// Default Seed for the `SuperSimplex` noise function.
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
#[cfg_attr(rustfmt, rustfmt_skip)]
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
#[cfg_attr(rustfmt, rustfmt_skip)]
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

/// Noise function that outputs 2/3-dimensional Super Simplex noise.
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

impl Default for SuperSimplex {
    fn default() -> Self {
        Self::new()
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
            seed: seed,
            perm_table: PermutationTable::new(seed),
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional Super Simplex noise
impl NoiseFn<Point2<f64>> for SuperSimplex {
    fn get(&self, point: Point2<f64>) -> f64 {
        let mut value = 0.0;

        // Transform point from real space to simplex space
        let to_simplex_offset = math::fold2(point, Add::add) * TO_SIMPLEX_CONSTANT_2D;
        let simplex_point = math::map2(point, |v| v + to_simplex_offset);

        // Get base point of simplex and barycentric coordinates in simplex space
        let simplex_base_point = math::map2(simplex_point, f64::floor);
        let simplex_base_point_i = math::to_isize2(simplex_base_point);
        let simplex_rel_coords = math::sub2(simplex_point, simplex_base_point);

        // Create index to lookup table from barycentric coordinates
        let region_sum = math::fold2(simplex_rel_coords, Add::add).floor();
        let index = ((region_sum >= 1.0) as usize) << 2 |
            ((simplex_rel_coords[0] - simplex_rel_coords[1] * 0.5 + 1.0 -
                  region_sum * 0.5 >= 1.0) as usize) << 3 |
            ((simplex_rel_coords[1] - simplex_rel_coords[0] * 0.5 + 1.0 -
                  region_sum * 0.5 >= 1.0) as usize) << 4;

        // Transform barycentric coordinates to real space
        let to_real_offset = math::fold2(simplex_rel_coords, Add::add) * TO_REAL_CONSTANT_2D;
        let real_rel_coords = math::map2(simplex_rel_coords, |v| v + to_real_offset);

        for lattice_lookup in &LATTICE_LOOKUP_2D[index..index + 4] {
            let dpos = math::add2(real_rel_coords, math::cast2(lattice_lookup.1));
            let attn = (2.0 / 3.0) - math::dot2(dpos, dpos);
            if attn > 0.0 {
                let lattice_point = math::add2(simplex_base_point_i, math::cast2(lattice_lookup.0));
                let gradient = gradient::get2(self.perm_table.get2(lattice_point));
                value += attn.powi(4) * math::dot2(gradient, dpos);
            }
        }

        value * NORM_CONSTANT_2D
    }
}

/// 3-dimensional Super Simplex noise
impl NoiseFn<Point3<f64>> for SuperSimplex {
    fn get(&self, point: Point3<f64>) -> f64 {
        let mut value = 0.0;

        // Transform point from real space to simplex space
        let to_simplex_offset = math::fold3(point, Add::add) * TO_SIMPLEX_CONSTANT_3D;
        let simplex_point = math::map3(point, |v| -(v + to_simplex_offset));
        let second_simplex_point = math::map3(simplex_point, |v| v + 512.5);

        // Get base point of simplex and barycentric coordinates in simplex space
        let simplex_base_point = math::map3(simplex_point, f64::floor);
        let simplex_base_point_i = math::to_isize3(simplex_base_point);
        let simplex_rel_coords = math::sub3(simplex_point, simplex_base_point);
        let second_simplex_base_point = math::map3(second_simplex_point, f64::floor);
        let second_simplex_base_point_i = math::to_isize3(second_simplex_base_point);
        let second_simplex_rel_coords = math::sub3(second_simplex_point, second_simplex_base_point);

        // Create indices to lookup table from barycentric coordinates
        let index = ((simplex_rel_coords[0] + simplex_rel_coords[1] +
                          simplex_rel_coords[2] >= 1.5) as usize) << 2 |
            ((-simplex_rel_coords[0] + simplex_rel_coords[1] + simplex_rel_coords[2] >=
                  0.5) as usize) << 3 |
            ((simplex_rel_coords[0] - simplex_rel_coords[1] + simplex_rel_coords[2] >=
                 0.5) as usize) << 4 |
            ((simplex_rel_coords[0] + simplex_rel_coords[1] - simplex_rel_coords[2] >= 0.5) as
                 usize) << 5;
        let second_index = ((second_simplex_rel_coords[0] + second_simplex_rel_coords[1] +
                                 second_simplex_rel_coords[2] >=
                                 1.5) as usize) << 2 |
            ((-second_simplex_rel_coords[0] + second_simplex_rel_coords[1] +
                  second_simplex_rel_coords[2] >= 0.5) as usize) << 3 |
            ((second_simplex_rel_coords[0] - second_simplex_rel_coords[1] +
                  second_simplex_rel_coords[2] >= 0.5) as usize) << 4 |
            ((second_simplex_rel_coords[0] + second_simplex_rel_coords[1] -
                  second_simplex_rel_coords[2] >= 0.5) as usize) << 5;

        // Sum contributions from first lattice
        for &lattice_lookup in &LATTICE_LOOKUP_3D[index..index + 4] {
            let dpos = math::sub3(simplex_rel_coords, math::cast3(lattice_lookup));
            let attn = 0.75 - math::dot3(dpos, dpos);
            if attn > 0.0 {
                let lattice_point = math::add3(simplex_base_point_i, math::cast3(lattice_lookup));
                let gradient = gradient::get3(self.perm_table.get3(lattice_point));
                value += attn.powi(4) * math::dot3(gradient, dpos);
            }
        }

        // Sum contributions from second lattice
        for &lattice_lookup in &LATTICE_LOOKUP_3D[second_index..second_index + 4] {
            let dpos = math::sub3(second_simplex_rel_coords, math::cast3(lattice_lookup));
            let attn = 0.75 - math::dot3(dpos, dpos);
            if attn > 0.0 {
                let lattice_point = math::add3(second_simplex_base_point_i,
                                               math::cast3(lattice_lookup));
                let gradient = gradient::get3(self.perm_table.get3(lattice_point));
                value += attn.powi(4) * math::dot3(gradient, dpos);
            }
        }

        value * NORM_CONSTANT_3D
    }
}
