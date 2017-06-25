// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.


use math;
use math::{Point2, Point3, Point4, interp};
use noise_fns::{NoiseFn, Seedable};
use permutationtable::PermutationTable;

/// Default Seed for the Value noise function.
pub const DEFAULT_VALUE_SEED: u32 = 0;

/// Noise function that outputs 2/3/4-dimensional Value noise.
#[derive(Clone, Copy, Debug)]
pub struct Value {
    seed: u32,
    perm_table: PermutationTable,
}

impl Value {
    pub fn new() -> Value {
        Value {
            seed: DEFAULT_VALUE_SEED,
            perm_table: PermutationTable::new(DEFAULT_VALUE_SEED),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::new()
    }
}

impl Seedable for Value {
    /// Sets the seed value for Value noise
    fn set_seed(self, seed: u32) -> Value {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Value {
            seed: seed,
            perm_table: PermutationTable::new(seed),
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional value noise
impl NoiseFn<Point2<f64>> for Value {
    fn get(&self, point: Point2<f64>) -> f64 {
        #[inline(always)]
        fn get(perm_table: &PermutationTable, corner: math::Point2<isize>) -> f64 {
            perm_table.get2(corner) as f64 / 255.0
        }

        let floored = math::map2(point, f64::floor);
        let near_corner = math::to_isize2(floored);
        let far_corner = math::add2(near_corner, math::one2());
        let weight = math::map2(math::sub2(point, floored), interp::s_curve5);

        let f00 = get(&self.perm_table, [near_corner[0], near_corner[1]]);
        let f10 = get(&self.perm_table, [far_corner[0], near_corner[1]]);
        let f01 = get(&self.perm_table, [near_corner[0], far_corner[1]]);
        let f11 = get(&self.perm_table, [far_corner[0], far_corner[1]]);

        let d0 = interp::linear(f00, f10, weight[0]);
        let d1 = interp::linear(f01, f11, weight[0]);
        let d = interp::linear(d0, d1, weight[1]);

        d * 2.0 - 1.0
    }
}

/// 3-dimensional value noise
impl NoiseFn<Point3<f64>> for Value {
    fn get(&self, point: Point3<f64>) -> f64 {
        #[inline(always)]
        fn get(perm_table: &PermutationTable, corner: math::Point3<isize>) -> f64 {
            perm_table.get3(corner) as f64 / 255.0
        }

        let floored = math::map3(point, f64::floor);
        let near_corner = math::to_isize3(floored);
        let far_corner = math::add3(near_corner, math::one3());
        let weight = math::map3(math::sub3(point, floored), interp::s_curve5);

        let f000 = get(&self.perm_table,
                       [near_corner[0], near_corner[1], near_corner[2]]);
        let f100 = get(&self.perm_table,
                       [far_corner[0], near_corner[1], near_corner[2]]);
        let f010 = get(&self.perm_table,
                       [near_corner[0], far_corner[1], near_corner[2]]);
        let f110 = get(&self.perm_table,
                       [far_corner[0], far_corner[1], near_corner[2]]);
        let f001 = get(&self.perm_table,
                       [near_corner[0], near_corner[1], far_corner[2]]);
        let f101 = get(&self.perm_table,
                       [far_corner[0], near_corner[1], far_corner[2]]);
        let f011 = get(&self.perm_table,
                       [near_corner[0], far_corner[1], far_corner[2]]);
        let f111 = get(&self.perm_table,
                       [far_corner[0], far_corner[1], far_corner[2]]);

        let d00 = interp::linear(f000, f100, weight[0]);
        let d01 = interp::linear(f001, f101, weight[0]);
        let d10 = interp::linear(f010, f110, weight[0]);
        let d11 = interp::linear(f011, f111, weight[0]);
        let d0 = interp::linear(d00, d10, weight[1]);
        let d1 = interp::linear(d01, d11, weight[1]);
        let d = interp::linear(d0, d1, weight[2]);

        d * 2.0 - 1.0
    }
}

/// 4-dimensional value noise
impl NoiseFn<Point4<f64>> for Value {
    fn get(&self, point: Point4<f64>) -> f64 {
        #[inline(always)]
        fn get(perm_table: &PermutationTable, corner: math::Point4<isize>) -> f64 {
            perm_table.get4(corner) as f64 / 255.0
        }

        let floored = math::map4(point, f64::floor);
        let near_corner = math::to_isize4(floored);
        let far_corner = math::add4(near_corner, math::one4());
        let weight = math::map4(math::sub4(point, floored), interp::s_curve5);

        let f0000 = get(
            &self.perm_table,
            [
                near_corner[0],
                near_corner[1],
                near_corner[2],
                near_corner[3],
            ],
        );
        let f1000 = get(
            &self.perm_table,
            [
                far_corner[0],
                near_corner[1],
                near_corner[2],
                near_corner[3],
            ],
        );
        let f0100 = get(
            &self.perm_table,
            [
                near_corner[0],
                far_corner[1],
                near_corner[2],
                near_corner[3],
            ],
        );
        let f1100 = get(
            &self.perm_table,
            [
                far_corner[0],
                far_corner[1],
                near_corner[2],
                near_corner[3],
            ],
        );
        let f0010 = get(
            &self.perm_table,
            [
                near_corner[0],
                near_corner[1],
                far_corner[2],
                near_corner[3],
            ],
        );
        let f1010 = get(
            &self.perm_table,
            [
                far_corner[0],
                near_corner[1],
                far_corner[2],
                near_corner[3],
            ],
        );
        let f0110 = get(
            &self.perm_table,
            [
                near_corner[0],
                far_corner[1],
                far_corner[2],
                near_corner[3],
            ],
        );
        let f1110 = get(
            &self.perm_table,
            [
                far_corner[0],
                far_corner[1],
                far_corner[2],
                near_corner[3],
            ],
        );
        let f0001 = get(
            &self.perm_table,
            [
                near_corner[0],
                near_corner[1],
                near_corner[2],
                far_corner[3],
            ],
        );
        let f1001 = get(
            &self.perm_table,
            [
                far_corner[0],
                near_corner[1],
                near_corner[2],
                far_corner[3],
            ],
        );
        let f0101 = get(
            &self.perm_table,
            [
                near_corner[0],
                far_corner[1],
                near_corner[2],
                far_corner[3],
            ],
        );
        let f1101 = get(
            &self.perm_table,
            [
                far_corner[0],
                far_corner[1],
                near_corner[2],
                far_corner[3],
            ],
        );
        let f0011 = get(
            &self.perm_table,
            [
                near_corner[0],
                near_corner[1],
                far_corner[2],
                far_corner[3],
            ],
        );
        let f1011 = get(
            &self.perm_table,
            [
                far_corner[0],
                near_corner[1],
                far_corner[2],
                far_corner[3],
            ],
        );
        let f0111 = get(
            &self.perm_table,
            [
                near_corner[0],
                far_corner[1],
                far_corner[2],
                far_corner[3],
            ],
        );
        let f1111 = get(
            &self.perm_table,
            [
                far_corner[0],
                far_corner[1],
                far_corner[2],
                far_corner[3],
            ],
        );

        let d000 = interp::linear(f0000, f1000, weight[0]);
        let d010 = interp::linear(f0010, f1010, weight[0]);
        let d100 = interp::linear(f0100, f1100, weight[0]);
        let d110 = interp::linear(f0110, f1110, weight[0]);
        let d001 = interp::linear(f0001, f1001, weight[0]);
        let d011 = interp::linear(f0011, f1011, weight[0]);
        let d101 = interp::linear(f0101, f1101, weight[0]);
        let d111 = interp::linear(f0111, f1111, weight[0]);
        let d00 = interp::linear(d000, d100, weight[1]);
        let d10 = interp::linear(d010, d110, weight[1]);
        let d01 = interp::linear(d001, d101, weight[1]);
        let d11 = interp::linear(d011, d111, weight[1]);
        let d0 = interp::linear(d00, d10, weight[2]);
        let d1 = interp::linear(d01, d11, weight[2]);
        let d = interp::linear(d0, d1, weight[3]);

        d * 2.0 - 1.0
    }
}
