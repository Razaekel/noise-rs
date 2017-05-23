// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use {PermutationTable, gradient, math};
use math::{Point2, Point3, Point4};
use modules::{NoiseModule, Seedable};
use num_traits::Float;
use std::ops::Add;

/// Default Seed for the Perlin noise module.
pub const DEFAULT_SUPER_SIMPLEX_SEED: u32 = 0;

const TO_REAL_CONSTANT_2D: f64 = -0.211324865405187; // (1 / sqrt(2 + 1) - 1) / 2;
const TO_SIMPLEX_CONSTANT_2D: f64 = 0.366025403784439; // (sqrt(2 + 1) - 1) / 2;
const STRETCH_CONSTANT_3D: f64 = -1.0 / 6.0; // (1 / sqrt(3 + 1) - 1) / 3;
const SQUISH_CONSTANT_3D: f64 = 1.0 / 3.0; // (sqrt(3 + 1) - 1) / 3;
const STRETCH_CONSTANT_4D: f64 = -0.138196601125011; // (sqrt(4 + 1) - 1) / 4;
const SQUISH_CONSTANT_4D: f64 = 0.309016994374947; // (sqrt(4 + 1) - 1) / 4;

const NORM_CONSTANT_2D: f64 = 18.518518518518519;
const NORM_CONSTANT_3D: f64 = 14.0;
const NORM_CONSTANT_4D: f64 = 6.8699090070956625;

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

/// Noise module that outputs 2/3/4-dimensional Super Simplex noise.
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
            seed: seed,
            perm_table: PermutationTable::new(seed),
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
        let two: T = math::cast(2.0);
        let two_thirds: T = math::cast(2.0 / 3.0);
        let to_real_constant: T = math::cast(TO_REAL_CONSTANT_2D);
        let to_simplex_constant: T = math::cast(TO_SIMPLEX_CONSTANT_2D);

        let mut value = zero;

        // Transform point from real space to simplex space
        let to_simplex_offset = math::fold2(point, Add::add) * to_simplex_constant;
        let simplex_point = math::map2(point, |v| v + to_simplex_offset);

        // Get base point of simplex and barycentric coordinates in simplex space
        let simplex_base_point = math::map2(simplex_point, Float::floor);
        let simplex_rel_coords = math::sub2(simplex_point, simplex_base_point);

        // Create index to lookup table from barycentric coordinates
        let region_sum = math::cast::<_, T>(math::cast::<_, usize>(math::fold2(simplex_rel_coords, Add::add)));
        let index = (math::cast::<_, usize>(region_sum) << 2) |
        math::cast::<_, usize>(simplex_rel_coords[0] - simplex_rel_coords[1] / two + one - region_sum / two) << 3 |
        math::cast::<_, usize>(simplex_rel_coords[1] - simplex_rel_coords[0] / two + one - region_sum / two) << 4;

        // Transform barycentric coordinates to real space
        let to_real_offset = math::fold2(simplex_rel_coords, Add::add) * to_real_constant;
        let real_rel_coords = math::map2(simplex_rel_coords, |v| v + to_real_offset);

        for i in 0..4 {
            let lattice_lookup = LATTICE_LOOKUP_2D[index + i];

            let dpos = math::add2(real_rel_coords, math::cast2(lattice_lookup.1));
            let attn = two_thirds - math::dot2(dpos, dpos);
            if attn <= zero {
                continue;
            }

            let lattice_point = math::cast2::<_, isize>(math::add2(simplex_base_point, math::cast2(lattice_lookup.0)));
            let gradient = math::mul2(gradient::get2::<T>(self.perm_table.get2::<isize>(lattice_point)), math::cast(NORM_CONSTANT_2D));
            value = value + math::pow4(attn) * math::dot2(gradient, dpos);
        }

        value
    }
}

/*/// 3-dimensional perlin noise
impl<T: Float> NoiseModule<Point3<T>> for Perlin {
    type Output = T;

    fn get(&self, point: Point3<T>) -> T {
        #[inline(always)]
        fn surflet<T: Float>(perm_table: &PermutationTable,
                             corner: math::Point3<isize>,
                             distance: math::Vector3<T>)
                             -> T {
            let attn = T::one() - math::dot3(distance, distance);
            if attn > T::zero() {
                math::pow4(attn) * math::dot3(distance, gradient::get3(perm_table.get3(corner)))
            } else {
                T::zero()
            }
        }

        let floored = math::map3(point, T::floor);
        let near_corner = math::map3(floored, math::cast);
        let far_corner = math::add3(near_corner, math::one3());
        let near_distance = math::sub3(point, floored);
        let far_distance = math::sub3(near_distance, math::one3());

        let f000 = surflet(&self.perm_table,
                           [near_corner[0], near_corner[1], near_corner[2]],
                           [near_distance[0], near_distance[1], near_distance[2]]);
        let f100 = surflet(&self.perm_table,
                           [far_corner[0], near_corner[1], near_corner[2]],
                           [far_distance[0], near_distance[1], near_distance[2]]);
        let f010 = surflet(&self.perm_table,
                           [near_corner[0], far_corner[1], near_corner[2]],
                           [near_distance[0], far_distance[1], near_distance[2]]);
        let f110 = surflet(&self.perm_table,
                           [far_corner[0], far_corner[1], near_corner[2]],
                           [far_distance[0], far_distance[1], near_distance[2]]);
        let f001 = surflet(&self.perm_table,
                           [near_corner[0], near_corner[1], far_corner[2]],
                           [near_distance[0], near_distance[1], far_distance[2]]);
        let f101 = surflet(&self.perm_table,
                           [far_corner[0], near_corner[1], far_corner[2]],
                           [far_distance[0], near_distance[1], far_distance[2]]);
        let f011 = surflet(&self.perm_table,
                           [near_corner[0], far_corner[1], far_corner[2]],
                           [near_distance[0], far_distance[1], far_distance[2]]);
        let f111 = surflet(&self.perm_table,
                           [far_corner[0], far_corner[1], far_corner[2]],
                           [far_distance[0], far_distance[1], far_distance[2]]);

        // Multiply by arbitrary value to scale to -1..1
        (f000 + f100 + f010 + f110 + f001 + f101 + f011 + f111) * math::cast(3.8898553255531074)
    }
}

/// 4-dimensional perlin noise
impl<T: Float> NoiseModule<Point4<T>> for Perlin {
    type Output = T;

    fn get(&self, point: Point4<T>) -> T {
        #[inline(always)]
        fn surflet<T: Float>(perm_table: &PermutationTable,
                             corner: math::Point4<isize>,
                             distance: math::Vector4<T>)
                             -> T {
            let attn = T::one() - math::dot4(distance, distance);
            if attn > T::zero() {
                math::pow4(attn) * math::dot4(distance, gradient::get4(perm_table.get4(corner)))
            } else {
                T::zero()
            }
        }

        let floored = math::map4(point, T::floor);
        let near_corner = math::map4(floored, math::cast);
        let far_corner = math::add4(near_corner, math::one4());
        let near_distance = math::sub4(point, floored);
        let far_distance = math::sub4(near_distance, math::one4());

        let f0000 =
            surflet(&self.perm_table,
                    [near_corner[0], near_corner[1], near_corner[2], near_corner[3]],
                    [near_distance[0], near_distance[1], near_distance[2], near_distance[3]]);
        let f1000 =
            surflet(&self.perm_table,
                    [far_corner[0], near_corner[1], near_corner[2], near_corner[3]],
                    [far_distance[0], near_distance[1], near_distance[2], near_distance[3]]);
        let f0100 =
            surflet(&self.perm_table,
                    [near_corner[0], far_corner[1], near_corner[2], near_corner[3]],
                    [near_distance[0], far_distance[1], near_distance[2], near_distance[3]]);
        let f1100 = surflet(&self.perm_table,
                            [far_corner[0], far_corner[1], near_corner[2], near_corner[3]],
                            [far_distance[0], far_distance[1], near_distance[2], near_distance[3]]);
        let f0010 =
            surflet(&self.perm_table,
                    [near_corner[0], near_corner[1], far_corner[2], near_corner[3]],
                    [near_distance[0], near_distance[1], far_distance[2], near_distance[3]]);
        let f1010 = surflet(&self.perm_table,
                            [far_corner[0], near_corner[1], far_corner[2], near_corner[3]],
                            [far_distance[0], near_distance[1], far_distance[2], near_distance[3]]);
        let f0110 = surflet(&self.perm_table,
                            [near_corner[0], far_corner[1], far_corner[2], near_corner[3]],
                            [near_distance[0], far_distance[1], far_distance[2], near_distance[3]]);
        let f1110 = surflet(&self.perm_table,
                            [far_corner[0], far_corner[1], far_corner[2], near_corner[3]],
                            [far_distance[0], far_distance[1], far_distance[2], near_distance[3]]);
        let f0001 =
            surflet(&self.perm_table,
                    [near_corner[0], near_corner[1], near_corner[2], far_corner[3]],
                    [near_distance[0], near_distance[1], near_distance[2], far_distance[3]]);
        let f1001 = surflet(&self.perm_table,
                            [far_corner[0], near_corner[1], near_corner[2], far_corner[3]],
                            [far_distance[0], near_distance[1], near_distance[2], far_distance[3]]);
        let f0101 = surflet(&self.perm_table,
                            [near_corner[0], far_corner[1], near_corner[2], far_corner[3]],
                            [near_distance[0], far_distance[1], near_distance[2], far_distance[3]]);
        let f1101 = surflet(&self.perm_table,
                            [far_corner[0], far_corner[1], near_corner[2], far_corner[3]],
                            [far_distance[0], far_distance[1], near_distance[2], far_distance[3]]);
        let f0011 = surflet(&self.perm_table,
                            [near_corner[0], near_corner[1], far_corner[2], far_corner[3]],
                            [near_distance[0], near_distance[1], far_distance[2], far_distance[3]]);
        let f1011 = surflet(&self.perm_table,
                            [far_corner[0], near_corner[1], far_corner[2], far_corner[3]],
                            [far_distance[0], near_distance[1], far_distance[2], far_distance[3]]);
        let f0111 = surflet(&self.perm_table,
                            [near_corner[0], far_corner[1], far_corner[2], far_corner[3]],
                            [near_distance[0], far_distance[1], far_distance[2], far_distance[3]]);
        let f1111 = surflet(&self.perm_table,
                            [far_corner[0], far_corner[1], far_corner[2], far_corner[3]],
                            [far_distance[0], far_distance[1], far_distance[2], far_distance[3]]);

        // Multiply by arbitrary value to scale to -1..1
        (f0000 + f1000 + f0100 + f1100 + f0010 + f1010 + f0110 + f1110 +
         f0001 + f1001 + f0101 + f1101 + f0011 + f1011 + f0111 + f1111) *
        math::cast(4.424369240215691)
    }
}*/
