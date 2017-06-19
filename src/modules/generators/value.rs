// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.


use math;
use math::{interp, Point2, Point3, Point4};
use modules::{NoiseModule, Seedable};
use num_traits::Float;
use permutationtable::PermutationTable;

/// Default Seed for the Value noise module.
pub const DEFAULT_VALUE_SEED: u32 = 0;

/// Noise module that outputs 2/3/4-dimensional Value noise.
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
impl<T: Float> NoiseModule<Point2<T>, T> for Value {
    fn get(&self, point: Point2<T>) -> T {
        #[inline(always)]
        fn get<T: Float>(perm_table: &PermutationTable, corner: math::Point2<isize>) -> T {
            math::cast::<_, T>(perm_table.get2(corner)) * math::cast(1.0 / 255.0)
        }

        let floored = math::map2(point, T::floor);
        let near_corner = math::map2(floored, math::cast);
        let far_corner = math::add2(near_corner, math::one2());
        let weight = math::map2(math::sub2(point, floored), interp::s_curve5);

        let f00 = get(&self.perm_table, [near_corner[0], near_corner[1]]);
        let f10 = get(&self.perm_table, [far_corner[0], near_corner[1]]);
        let f01 = get(&self.perm_table, [near_corner[0], far_corner[1]]);
        let f11 = get(&self.perm_table, [far_corner[0], far_corner[1]]);

        let d0 = interp::linear(f00, f10, weight[0]);
        let d1 = interp::linear(f01, f11, weight[0]);
        let d = interp::linear(d0, d1, weight[1]);

        d * math::cast(2) - math::cast(1)
    }
}

/// 3-dimensional value noise
impl<T: Float> NoiseModule<Point3<T>, T> for Value {
    fn get(&self, point: Point3<T>) -> T {
        #[inline(always)]
        fn get<T: Float>(perm_table: &PermutationTable, corner: math::Point3<isize>) -> T {
            math::cast::<_, T>(perm_table.get3(corner)) * math::cast(1.0 / 255.0)
        }

        let floored = math::map3(point, T::floor);
        let near_corner = math::map3(floored, math::cast);
        let far_corner = math::add3(near_corner, math::one3());
        let weight = math::map3(math::sub3(point, floored), interp::s_curve5);

        let f000: T = get(&self.perm_table, [near_corner[0], near_corner[1], near_corner[2]]);
        let f100: T = get(&self.perm_table, [far_corner[0], near_corner[1], near_corner[2]]);
        let f010: T = get(&self.perm_table, [near_corner[0], far_corner[1], near_corner[2]]);
        let f110: T = get(&self.perm_table, [far_corner[0], far_corner[1], near_corner[2]]);
        let f001: T = get(&self.perm_table, [near_corner[0], near_corner[1], far_corner[2]]);
        let f101: T = get(&self.perm_table, [far_corner[0], near_corner[1], far_corner[2]]);
        let f011: T = get(&self.perm_table, [near_corner[0], far_corner[1], far_corner[2]]);
        let f111: T = get(&self.perm_table, [far_corner[0], far_corner[1], far_corner[2]]);

        let d00 = interp::linear(f000, f100, weight[0]);
        let d01 = interp::linear(f001, f101, weight[0]);
        let d10 = interp::linear(f010, f110, weight[0]);
        let d11 = interp::linear(f011, f111, weight[0]);
        let d0 = interp::linear(d00, d10, weight[1]);
        let d1 = interp::linear(d01, d11, weight[1]);
        let d = interp::linear(d0, d1, weight[2]);

        d * math::cast(2) - math::cast(1)
    }
}

/// 4-dimensional value noise
#[cfg_attr(rustfmt, rustfmt_skip)]
impl<T: Float> NoiseModule<Point4<T>, T> for Value {
    fn get(&self, point: Point4<T>) -> T {
        #[inline(always)]
        fn get<T: Float>(perm_table: &PermutationTable, corner: math::Point4<isize>) -> T {
            math::cast::<_, T>(perm_table.get4(corner)) * math::cast(1.0 / 255.0)
        }

        let floored = math::map4(point, T::floor);
        let near_corner = math::map4(floored, math::cast);
        let far_corner = math::add4(near_corner, math::one4());
        let weight = math::map4(math::sub4(point, floored), interp::s_curve5);

        let f0000: T = get(&self.perm_table, [near_corner[0], near_corner[1], near_corner[2], near_corner[3]]);
        let f1000: T = get(&self.perm_table, [far_corner[0], near_corner[1], near_corner[2], near_corner[3]]);
        let f0100: T = get(&self.perm_table, [near_corner[0], far_corner[1], near_corner[2], near_corner[3]]);
        let f1100: T = get(&self.perm_table, [far_corner[0], far_corner[1], near_corner[2], near_corner[3]]);
        let f0010: T = get(&self.perm_table, [near_corner[0], near_corner[1], far_corner[2], near_corner[3]]);
        let f1010: T = get(&self.perm_table, [far_corner[0], near_corner[1], far_corner[2], near_corner[3]]);
        let f0110: T = get(&self.perm_table, [near_corner[0], far_corner[1], far_corner[2], near_corner[3]]);
        let f1110: T = get(&self.perm_table, [far_corner[0], far_corner[1], far_corner[2], near_corner[3]]);
        let f0001: T = get(&self.perm_table, [near_corner[0], near_corner[1], near_corner[2], far_corner[3]]);
        let f1001: T = get(&self.perm_table, [far_corner[0], near_corner[1], near_corner[2], far_corner[3]]);
        let f0101: T = get(&self.perm_table, [near_corner[0], far_corner[1], near_corner[2], far_corner[3]]);
        let f1101: T = get(&self.perm_table, [far_corner[0], far_corner[1], near_corner[2], far_corner[3]]);
        let f0011: T = get(&self.perm_table, [near_corner[0], near_corner[1], far_corner[2], far_corner[3]]);
        let f1011: T = get(&self.perm_table, [far_corner[0], near_corner[1], far_corner[2], far_corner[3]]);
        let f0111: T = get(&self.perm_table, [near_corner[0], far_corner[1], far_corner[2], far_corner[3]]);
        let f1111: T = get(&self.perm_table, [far_corner[0], far_corner[1], far_corner[2], far_corner[3]]);

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

        d * math::cast(2) - math::cast(1)
    }
}
