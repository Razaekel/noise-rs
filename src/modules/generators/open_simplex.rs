// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

//! Note that this is NOT Ken Perlin's simplex noise, as that is patent encumbered.
//! Instead, these functions use the OpenSimplex algorithm, as detailed here:
//! http://uniblock.tumblr.com/post/97868843242/noise

use {PermutationTable, gradient, math};
use math::{Point2, Point3, Point4};
use modules::{NoiseModule, Seedable};
use num_traits::Float;
use std::ops::Add;

const STRETCH_CONSTANT_2D: f64 = -0.211324865405187; //(1/sqrt(2+1)-1)/2;
const SQUISH_CONSTANT_2D: f64 = 0.366025403784439; //(sqrt(2+1)-1)/2;
const STRETCH_CONSTANT_3D: f64 = -1.0 / 6.0; //(1/Math.sqrt(3+1)-1)/3;
const SQUISH_CONSTANT_3D: f64 = 1.0 / 3.0; //(Math.sqrt(3+1)-1)/3;
const STRETCH_CONSTANT_4D: f64 = -0.138196601125011; //(Math.sqrt(4+1)-1)/4;
const SQUISH_CONSTANT_4D: f64 = 0.309016994374947; //(Math.sqrt(4+1)-1)/4;

const NORM_CONSTANT_2D: f32 = 1.0 / 14.0;
const NORM_CONSTANT_3D: f32 = 1.0 / 14.0;
const NORM_CONSTANT_4D: f32 = 1.0 / 6.8699090070956625;

pub const DEFAULT_OPENSIMPLEX_SEED: usize = 0;

/// Noise module that outputs 2/3/4-dimensional Open Simplex noise.
#[derive(Clone, Copy, Debug)]
pub struct OpenSimplex {
    seed: usize,
    perm_table: PermutationTable,
}

impl OpenSimplex {
    pub fn new() -> OpenSimplex {
        OpenSimplex {
            seed: DEFAULT_OPENSIMPLEX_SEED,
            perm_table: PermutationTable::new(DEFAULT_OPENSIMPLEX_SEED as u32),
        }
    }
}

impl Seedable for OpenSimplex {
    /// Sets the seed value for Open Simplex noise
    fn set_seed(self, seed: usize) -> OpenSimplex {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }
        // Otherwise, regenerate the permutation table based on the new seed.
        OpenSimplex {
            seed: seed,
            perm_table: PermutationTable::new(seed as u32),
        }
    }
}

/// 2-dimensional [OpenSimplex Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than Perlin 2D.
impl<T: Float> NoiseModule<Point2<T>> for OpenSimplex {
    type Output = T;

    fn get(&self, point: Point2<T>) -> T {
        #[inline(always)]
        fn gradient<T: Float>(perm_table: &PermutationTable,
                              vertex: math::Point2<T>,
                              pos: math::Point2<T>)
                              -> T {
            let zero = T::zero();
            let attn = math::cast::<_, T>(2.0_f64) - math::dot2(pos, pos);
            if attn > zero {
                let index = perm_table.get2::<isize>(math::cast2::<_, isize>(vertex));
                let vec = gradient::get2::<T>(index);
                math::pow4(attn) * math::dot2(pos, vec)
            } else {
                zero
            }
        }

        let zero = T::zero();
        let one = T::one();
        let stretch_constant: T = math::cast(STRETCH_CONSTANT_2D);
        let squish_constant: T = math::cast(SQUISH_CONSTANT_2D);

        // Place input coordinates onto grid.
        let stretch_offset = math::fold2(point, Add::add) * stretch_constant;
        let stretched = math::map2(point, |v| v + stretch_offset);

        // Floor to get grid coordinates of rhombus (stretched square) cell origin.
        let stretched_floor = math::map2(stretched, Float::floor);

        // Skew out to get actual coordinates of rhombus origin. We'll need these later.
        let squish_offset = math::fold2(stretched_floor, Add::add) * squish_constant;
        let skewed_floor = math::map2(stretched_floor, |v| v + squish_offset);

        // Compute grid coordinates relative to rhombus origin.
        let rel_coords = math::sub2(stretched, stretched_floor);

        // Sum those together to get a value that determines which region we're in.
        let region_sum = math::fold2(rel_coords, Add::add);

        // Positions relative to origin point (0, 0).
        let pos0 = math::sub2(point, skewed_floor);

        let mut value: T = zero;

        let mut vertex;
        let mut dpos;

        // (0, 0) --- (1, 0)
        // |   A     /     |
        // |       /       |
        // |     /     B   |
        // (0, 1) --- (1, 1)

        let t0 = squish_constant;
        let t1 = squish_constant + one;
        let t2 = squish_constant + t1;

        // Contribution (1, 0)
        vertex = math::add2(stretched_floor, [one, zero]);
        dpos = math::sub2(pos0, [t1, t0]);
        value = value + gradient(&self.perm_table, vertex, dpos);

        // Contribution (0, 1)
        vertex = math::add2(stretched_floor, [zero, one]);
        dpos = math::sub2(pos0, [t0, t1]);
        value = value + gradient(&self.perm_table, vertex, dpos);

        // See the graph for an intuitive explanation; the sum of `x` and `y` is
        // only greater than `1` if we're on Region B.
        if region_sum > one {
            // Contribution (1, 1)
            vertex = math::add2(stretched_floor, [one, one]);
            // We are moving across the diagonal `/`, so we'll need to add by the
            // squish constant
            dpos = math::sub2(pos0, [t2, t2]);
        } else {
            vertex = math::add2(stretched_floor, [zero, zero]);
            dpos = math::sub2(pos0, [zero, zero]);
        }

        // Point (0, 0) or (1, 1)
        value = value + gradient(&self.perm_table, vertex, dpos);

        value * math::cast(NORM_CONSTANT_2D)
    }
}

/// 3-dimensional [OpenSimplex Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than Perlin 3D.
impl<T: Float> NoiseModule<Point3<T>> for OpenSimplex {
    type Output = T;

    fn get(&self, point: Point3<T>) -> T {
        #[inline(always)]
        fn gradient<T: Float>(perm_table: &PermutationTable,
                              vertex: math::Point3<T>,
                              pos: math::Point3<T>)
                              -> T {
            let zero = T::zero();
            let attn = math::cast::<_, T>(2.0_f64) - math::dot3(pos, pos);
            if attn > zero {
                let index = perm_table.get3::<isize>(math::cast3::<_, isize>(vertex));
                let vec = gradient::get3::<T>(index);
                math::pow4(attn) * math::dot3(pos, vec)
            } else {
                zero
            }
        }

        let zero = T::zero();
        let one = T::one();
        let two: T = math::cast(2.0);
        let stretch_constant: T = math::cast(STRETCH_CONSTANT_3D);
        let squish_constant: T = math::cast(SQUISH_CONSTANT_3D);

        // Place input coordinates on simplectic honeycomb.
        let stretch_offset = math::fold3(point, Add::add) * stretch_constant;
        let stretched = math::map3(point, |v| v + stretch_offset);

        // Floor to get simplectic honeycomb coordinates of rhombohedron
        // (stretched cube) super-cell origin.
        let stretched_floor = math::map3(stretched, Float::floor);

        // Skew out to get actual coordinates of rhombohedron origin. We'll need
        // these later.
        let squish_offset = math::fold3(stretched_floor, Add::add) * squish_constant;
        let skewed_floor = math::map3(stretched_floor, |v| v + squish_offset);

        // Compute simplectic honeycomb coordinates relative to rhombohedral origin.
        let rel_coords = math::sub3(stretched, stretched_floor);

        // Sum those together to get a value that determines which region we're in.
        let region_sum = math::fold3(rel_coords, Add::add);

        // Positions relative to origin point.
        let pos0 = math::sub3(point, skewed_floor);

        let mut value = zero;

        let mut vertex;
        let mut dpos;

        if region_sum <= one {
            // We're inside the tetrahedron (3-Simplex) at (0, 0, 0)
            let t0 = squish_constant;
            let t1 = squish_constant + one;

            // Contribution at (0, 0, 0)
            vertex = math::add3(stretched_floor, [zero, zero, zero]);
            dpos = math::sub3(pos0, [zero, zero, zero]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (1, 0, 0)
            vertex = math::add3(stretched_floor, [one, zero, zero]);
            dpos = math::sub3(pos0, [t1, t0, t0]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 1, 0)
            vertex = math::add3(stretched_floor, [zero, one, zero]);
            dpos = math::sub3(pos0, [t0, t1, t0]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 0, 1)
            vertex = math::add3(stretched_floor, [zero, zero, one]);
            dpos = math::sub3(pos0, [t0, t0, t1]);
            value = value + gradient(&self.perm_table, vertex, dpos);
        } else if region_sum >= two {
            // We're inside the tetrahedron (3-Simplex) at (1, 1, 1)
            let t0 = two * squish_constant;
            let t1 = one + two * squish_constant;
            let t2 = t1 + squish_constant;

            // Contribution at (1, 1, 0)
            vertex = math::add3(stretched_floor, [one, one, zero]);
            dpos = math::sub3(pos0, [t1, t1, t0]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (1, 0, 1)
            vertex = math::add3(stretched_floor, [one, zero, one]);
            dpos = math::sub3(pos0, [t1, t0, t1]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 1, 1)
            vertex = math::add3(stretched_floor, [zero, one, one]);
            dpos = math::sub3(pos0, [t0, t1, t1]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (1, 1, 1)
            vertex = math::add3(stretched_floor, [one, one, one]);
            dpos = math::sub3(pos0, [t2, t2, t2]);
            value = value + gradient(&self.perm_table, vertex, dpos);
        } else {
            // We're inside the octahedron (Rectified 3-Simplex) inbetween.
            let t0 = squish_constant;
            let t1 = one + squish_constant;
            let t2 = two * squish_constant;
            let t3 = one + two * squish_constant;

            // Contribution at (1, 0, 0)
            vertex = math::add3(stretched_floor, [one, zero, zero]);
            dpos = math::sub3(pos0, [t1, t0, t0]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 1, 0)
            vertex = math::add3(stretched_floor, [zero, one, zero]);
            dpos = math::sub3(pos0, [t0, t1, t0]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 0, 1)
            vertex = math::add3(stretched_floor, [zero, zero, one]);
            dpos = math::sub3(pos0, [t0, t0, t1]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (1, 1, 0)
            vertex = math::add3(stretched_floor, [one, one, zero]);
            dpos = math::sub3(pos0, [t3, t3, t2]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (1, 0, 1)
            vertex = math::add3(stretched_floor, [one, zero, one]);
            dpos = math::sub3(pos0, [t3, t2, t3]);
            value = value + gradient(&self.perm_table, vertex, dpos);

            // Contribution at (0, 1, 1)
            vertex = math::add3(stretched_floor, [zero, one, one]);
            dpos = math::sub3(pos0, [t2, t3, t3]);
            value = value + gradient(&self.perm_table, vertex, dpos);
        }

        value * math::cast(NORM_CONSTANT_3D)
    }
}

/// 4-dimensional [OpenSimplex Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than Perlin 4D.
impl<T: Float> NoiseModule<Point4<T>> for OpenSimplex {
    type Output = T;

    fn get(&self, point: Point4<T>) -> T {
        #[inline(always)]
        fn gradient<T: Float>(perm_table: &PermutationTable,
                              vertex: math::Point4<T>,
                              pos: math::Point4<T>)
                              -> T {
            let zero = T::zero();
            let attn = math::cast::<_, T>(2.0_f64) - math::dot4(pos, pos);
            if attn > zero {
                let index = perm_table.get4::<isize>(math::cast4::<_, isize>(vertex));
                let vec = gradient::get4::<T>(index);
                math::pow4(attn) * math::dot4(pos, vec)
            } else {
                zero
            }
        }

        // Constants.
        let stretch_constant: T = math::cast(STRETCH_CONSTANT_4D);
        let squish_constant: T = math::cast(SQUISH_CONSTANT_4D);
        let zero = T::zero();
        let one = T::one();
        let two: T = math::cast(2.0);
        let three: T = math::cast(3.0);

        // Place input coordinates on simplectic honeycomb.
        let stretch_offset = math::fold4(point, Add::add) * stretch_constant;
        let stretched = math::map4(point, |v| v + stretch_offset);

        // Floor to get simplectic honeycomb coordinates of rhombo-hypercube
        // super-cell origin.
        let stretched_floor = math::map4(stretched, Float::floor);

        // Skew out to get actual coordinates of stretched rhombo-hypercube origin.
        // We'll need these later.
        let squish_offset = math::fold4(stretched_floor, Add::add) * squish_constant;
        let skewed_floor = math::map4(stretched_floor, |v| v + squish_offset);

        // Compute simplectic honeycomb coordinates relative to rhombo-hypercube
        // origin.
        let rel_coords = math::sub4(stretched, stretched_floor);

        // Sum those together to get a value that determines which region
        // we're in.
        let region_sum = math::fold4(rel_coords, Add::add);

        // Position relative to origin point.
        let mut pos0 = math::sub4(point, skewed_floor);

        let mut value = zero;
        if region_sum <= one {
            // We're inside the pentachoron (4-Simplex) at (0, 0, 0, 0)

            // Contribution at (0, 0, 0, 0)
            value = value + gradient(&self.perm_table, stretched_floor, pos0);

            // Contribution at (1, 0, 0, 0)
            let pos1;
            {
                let vertex = math::add4(stretched_floor, [one, zero, zero, zero]);
                pos1 = math::sub4(pos0,
                                  [one + squish_constant,
                                   squish_constant,
                                   squish_constant,
                                   squish_constant]);
                value = value + gradient(&self.perm_table, vertex, pos1);
            }

            // Contribution at (0, 1, 0, 0)
            let pos2;
            {
                let vertex = math::add4(stretched_floor, [zero, one, zero, zero]);
                pos2 = [pos1[0] + one, pos1[1] - one, pos1[2], pos1[3]];
                value = value + gradient(&self.perm_table, vertex, pos2);
            }

            // Contribution at (0, 0, 1, 0)
            let pos3;
            {
                let vertex = math::add4(stretched_floor, [zero, zero, one, zero]);
                pos3 = [pos2[0], pos1[1], pos1[2] - one, pos1[3]];
                value = value + gradient(&self.perm_table, vertex, pos3);
            }

            // Contribution at (0, 0, 0, 1)
            let pos4;
            {
                let vertex = math::add4(stretched_floor, [zero, zero, zero, one]);
                pos4 = [pos2[0], pos1[1], pos1[2], pos1[3] - one];
                value = value + gradient(&self.perm_table, vertex, pos4);
            }
        } else if region_sum >= three {
            // We're inside the pentachoron (4-Simplex) at (1, 1, 1, 1)
            let squish_constant_3 = three * squish_constant;

            // Contribution at (1, 1, 1, 0)
            let pos4;
            {
                let vertex = math::add4(stretched_floor, [one, one, one, zero]);
                pos4 = math::sub4(pos0,
                                  [one + squish_constant_3,
                                   one + squish_constant_3,
                                   one + squish_constant_3,
                                   squish_constant_3]);
                value = value + gradient(&self.perm_table, vertex, pos4);
            }

            // Contribution at (1, 1, 0, 1)
            let pos3;
            {
                let vertex = math::add4(stretched_floor, [one, one, zero, one]);
                pos3 = [pos4[0], pos4[1], pos4[2] + one, pos4[3] - one];
                value = value + gradient(&self.perm_table, vertex, pos3);
            }

            // Contribution at (1, 0, 1, 1)
            let pos2;
            {
                let vertex = math::add4(stretched_floor, [one, zero, one, one]);
                pos2 = [pos4[0], pos4[1] + one, pos4[2], pos3[3]];
                value = value + gradient(&self.perm_table, vertex, pos2);
            }

            // Contribution at (0, 1, 1, 1)
            let pos1;
            {
                let vertex = math::add4(stretched_floor, [zero, one, one, one]);
                pos1 = [pos0[0] - squish_constant_3, pos4[1], pos4[2], pos3[3]];
                value = value + gradient(&self.perm_table, vertex, pos1);
            }

            // Contribution at (1, 1, 1, 1)
            {
                let vertex = math::add4(stretched_floor, [one, one, one, one]);
                pos0[0] = pos4[0] - squish_constant;
                pos0[1] = pos4[1] - squish_constant;
                pos0[2] = pos4[2] - squish_constant;
                pos0[3] = pos3[3] - squish_constant;
                value = value + gradient(&self.perm_table, vertex, pos0);
            }
        } else if region_sum <= two {
            // We're inside the first dispentachoron (Rectified 4-Simplex)

            // Contribution at (1, 0, 0, 0)
            let pos1;
            {
                let vertex = math::add4(stretched_floor, [one, zero, zero, zero]);
                pos1 = math::sub4(pos0,
                                  [one + squish_constant,
                                   squish_constant,
                                   squish_constant,
                                   squish_constant]);
                value = value + gradient(&self.perm_table, vertex, pos1);
            }

            // Contribution at (0, 1, 0, 0)
            let pos2;
            {
                let vertex = math::add4(stretched_floor, [zero, one, zero, zero]);
                pos2 = [pos1[0] + one, pos1[1] - one, pos1[2], pos1[3]];
                value = value + gradient(&self.perm_table, vertex, pos2);
            }

            // Contribution at (0, 0, 1, 0)
            let pos3;
            {
                let vertex = math::add4(stretched_floor, [zero, zero, one, zero]);
                pos3 = [pos2[0], pos1[1], pos1[2] - one, pos1[3]];
                value = value + gradient(&self.perm_table, vertex, pos3);
            }

            // Contribution at (0, 0, 0, 1)
            let pos4;
            {
                let vertex = math::add4(stretched_floor, [zero, zero, zero, one]);
                pos4 = [pos2[0], pos1[1], pos1[2], pos1[3] - one];
                value = value + gradient(&self.perm_table, vertex, pos4);
            }

            // Contribution at (1, 1, 0, 0)
            let pos5;
            {
                let vertex = math::add4(stretched_floor, [one, one, zero, zero]);
                pos5 = [pos1[0] - squish_constant,
                        pos2[1] - squish_constant,
                        pos1[2] - squish_constant,
                        pos1[3] - squish_constant];
                value = value + gradient(&self.perm_table, vertex, pos5);
            }

            // Contribution at (1, 0, 1, 0)
            let pos6;
            {
                let vertex = math::add4(stretched_floor, [one, zero, one, zero]);
                pos6 = [pos5[0], pos5[1] + one, pos5[2] - one, pos5[3]];
                value = value + gradient(&self.perm_table, vertex, pos6);
            }

            // Contribution at (1, 0, 0, 1)
            let pos7;
            {
                let vertex = math::add4(stretched_floor, [one, zero, zero, one]);
                pos7 = [pos5[0], pos6[1], pos5[2], pos5[3] - one];
                value = value + gradient(&self.perm_table, vertex, pos7);
            }

            // Contribution at (0, 1, 1, 0)
            let pos8;
            {
                let vertex = math::add4(stretched_floor, [zero, one, one, zero]);
                pos8 = [pos5[0] + one, pos5[1], pos6[2], pos5[3]];
                value = value + gradient(&self.perm_table, vertex, pos8);
            }

            // Contribution at (0, 1, 0, 1)
            let pos9;
            {
                let vertex = math::add4(stretched_floor, [zero, one, zero, one]);
                pos9 = [pos8[0], pos5[1], pos5[2], pos7[3]];
                value = value + gradient(&self.perm_table, vertex, pos9);
            }

            // Contribution at (0, 0, 1, 1)
            let pos10;
            {
                let vertex = math::add4(stretched_floor, [zero, zero, one, one]);
                pos10 = [pos8[0], pos6[1], pos6[2], pos7[3]];
                value = value + gradient(&self.perm_table, vertex, pos10);
            }
        } else {
            // We're inside the second dispentachoron (Rectified 4-Simplex)
            let squish_constant_3 = three * squish_constant;

            // Contribution at (1, 1, 1, 0)
            let pos4;
            {
                let vertex = math::add4(stretched_floor, [one, one, one, zero]);
                pos4 = math::sub4(pos0,
                                  [one + squish_constant_3,
                                   one + squish_constant_3,
                                   one + squish_constant_3,
                                   squish_constant_3]);
                value = value + gradient(&self.perm_table, vertex, pos4);
            }

            // Contribution at (1, 1, 0, 1)
            let pos3;
            {
                let vertex = math::add4(stretched_floor, [one, one, zero, one]);
                pos3 = [pos4[0], pos4[1], pos4[2] + one, pos4[3] - one];
                value = value + gradient(&self.perm_table, vertex, pos3);
            }

            // Contribution at (1, 0, 1, 1)
            let pos2;
            {
                let vertex = math::add4(stretched_floor, [one, zero, one, one]);
                pos2 = [pos4[0], pos4[1] + one, pos4[2], pos3[3]];
                value = value + gradient(&self.perm_table, vertex, pos2);
            }

            // Contribution at (0, 1, 1, 1)
            let pos1;
            {
                let vertex = math::add4(stretched_floor, [zero, one, one, one]);
                pos1 = [pos4[0] + one, pos4[1], pos4[2], pos3[3]];
                value = value + gradient(&self.perm_table, vertex, pos1);
            }

            // Contribution at (1, 1, 0, 0)
            let pos5;
            {
                let vertex = math::add4(stretched_floor, [one, one, zero, zero]);
                pos5 = [pos4[0] + squish_constant,
                        pos4[1] + squish_constant,
                        pos3[2] + squish_constant,
                        pos4[3] + squish_constant];
                value = value + gradient(&self.perm_table, vertex, pos5);
            }

            // Contribution at (1, 0, 1, 0)
            let pos6;
            {
                let vertex = math::add4(stretched_floor, [one, zero, one, zero]);
                pos6 = [pos5[0], pos5[1] + one, pos5[2] - one, pos5[3]];
                value = value + gradient(&self.perm_table, vertex, pos6);
            }

            // Contribution at (1, 0, 0, 1)
            let pos7;
            {
                let vertex = math::add4(stretched_floor, [one, zero, zero, one]);
                pos7 = [pos5[0], pos6[1], pos5[2], pos5[3] - one];
                value = value + gradient(&self.perm_table, vertex, pos7);
            }

            // Contribution at (0, 1, 1, 0)
            let pos8;
            {
                let vertex = math::add4(stretched_floor, [zero, one, one, zero]);
                pos8 = [pos5[0] + one, pos5[1], pos6[2], pos5[3]];
                value = value + gradient(&self.perm_table, vertex, pos8);
            }

            // Contribution at (0, 1, 0, 1)
            let pos9;
            {
                let vertex = math::add4(stretched_floor, [zero, one, zero, one]);
                pos9 = [pos8[0], pos5[1], pos5[2], pos7[3]];
                value = value + gradient(&self.perm_table, vertex, pos9);
            }

            // Contribution at (0, 0, 1, 1)
            let pos10;
            {
                let vertex = math::add4(stretched_floor, [zero, zero, one, one]);
                pos10 = [pos8[0], pos6[1], pos6[2], pos7[3]];
                value = value + gradient(&self.perm_table, vertex, pos10);
            }
        }

        value * math::cast(NORM_CONSTANT_4D)
    }
}
