use crate::math::s_curve::quintic::Quintic;
use crate::{
    math::{self, interpolate},
    noise_fns::{NoiseFn, Seedable},
    permutationtable::{NoiseHasher, PermutationTable},
};

/// Noise function that outputs 2/3/4-dimensional Value noise.
#[derive(Clone, Copy, Debug)]
pub struct Value {
    seed: u32,
    perm_table: PermutationTable,
}

impl Value {
    pub const DEFAULT_SEED: u32 = 0;
}

impl Default for Value {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl Seedable for Value {
    fn new(seed: u32) -> Self {
        Self {
            seed,
            perm_table: PermutationTable::new(seed),
        }
    }

    /// Sets the seed value for Value noise
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

/// 2-dimensional value noise
impl NoiseFn<2> for Value {
    fn get(&self, point: [f64; 2]) -> f64 {
        fn get(perm_table: &PermutationTable, corner: [isize; 2]) -> f64 {
            perm_table.hash(&corner) as f64 / 255.0
        }

        let floored = math::map2(point, f64::floor);
        let near_corner = math::to_isize2(floored);
        let far_corner = math::add2(near_corner, math::one2());
        let weight = math::sub2(point, floored).map_quintic();

        let f00 = get(&self.perm_table, [near_corner[0], near_corner[1]]);
        let f10 = get(&self.perm_table, [far_corner[0], near_corner[1]]);
        let f01 = get(&self.perm_table, [near_corner[0], far_corner[1]]);
        let f11 = get(&self.perm_table, [far_corner[0], far_corner[1]]);

        let d0 = interpolate::linear(f00, f10, weight[0]);
        let d1 = interpolate::linear(f01, f11, weight[0]);
        let d = interpolate::linear(d0, d1, weight[1]);

        d * 2.0 - 1.0
    }
}

/// 3-dimensional value noise
impl NoiseFn<3> for Value {
    fn get(&self, point: [f64; 3]) -> f64 {
        fn get(perm_table: &PermutationTable, corner: [isize; 3]) -> f64 {
            perm_table.hash(&corner) as f64 / 255.0
        }

        let floored = math::map3(point, f64::floor);
        let near_corner = math::to_isize3(floored);
        let far_corner = math::add3(near_corner, math::one3());
        let weight = math::sub3(point, floored).map_quintic();

        let f000 = get(
            &self.perm_table,
            [near_corner[0], near_corner[1], near_corner[2]],
        );
        let f100 = get(
            &self.perm_table,
            [far_corner[0], near_corner[1], near_corner[2]],
        );
        let f010 = get(
            &self.perm_table,
            [near_corner[0], far_corner[1], near_corner[2]],
        );
        let f110 = get(
            &self.perm_table,
            [far_corner[0], far_corner[1], near_corner[2]],
        );
        let f001 = get(
            &self.perm_table,
            [near_corner[0], near_corner[1], far_corner[2]],
        );
        let f101 = get(
            &self.perm_table,
            [far_corner[0], near_corner[1], far_corner[2]],
        );
        let f011 = get(
            &self.perm_table,
            [near_corner[0], far_corner[1], far_corner[2]],
        );
        let f111 = get(
            &self.perm_table,
            [far_corner[0], far_corner[1], far_corner[2]],
        );

        let d00 = interpolate::linear(f000, f100, weight[0]);
        let d01 = interpolate::linear(f001, f101, weight[0]);
        let d10 = interpolate::linear(f010, f110, weight[0]);
        let d11 = interpolate::linear(f011, f111, weight[0]);
        let d0 = interpolate::linear(d00, d10, weight[1]);
        let d1 = interpolate::linear(d01, d11, weight[1]);
        let d = interpolate::linear(d0, d1, weight[2]);

        d * 2.0 - 1.0
    }
}

/// 4-dimensional value noise
impl NoiseFn<4> for Value {
    fn get(&self, point: [f64; 4]) -> f64 {
        fn get(perm_table: &PermutationTable, corner: [isize; 4]) -> f64 {
            perm_table.hash(&corner) as f64 / 255.0
        }

        let floored = math::map4(point, f64::floor);
        let near_corner = math::to_isize4(floored);
        let far_corner = math::add4(near_corner, math::one4());
        let weight = math::sub4(point, floored).map_quintic();

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
            [far_corner[0], far_corner[1], near_corner[2], near_corner[3]],
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
            [far_corner[0], near_corner[1], far_corner[2], near_corner[3]],
        );
        let f0110 = get(
            &self.perm_table,
            [near_corner[0], far_corner[1], far_corner[2], near_corner[3]],
        );
        let f1110 = get(
            &self.perm_table,
            [far_corner[0], far_corner[1], far_corner[2], near_corner[3]],
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
            [far_corner[0], near_corner[1], near_corner[2], far_corner[3]],
        );
        let f0101 = get(
            &self.perm_table,
            [near_corner[0], far_corner[1], near_corner[2], far_corner[3]],
        );
        let f1101 = get(
            &self.perm_table,
            [far_corner[0], far_corner[1], near_corner[2], far_corner[3]],
        );
        let f0011 = get(
            &self.perm_table,
            [near_corner[0], near_corner[1], far_corner[2], far_corner[3]],
        );
        let f1011 = get(
            &self.perm_table,
            [far_corner[0], near_corner[1], far_corner[2], far_corner[3]],
        );
        let f0111 = get(
            &self.perm_table,
            [near_corner[0], far_corner[1], far_corner[2], far_corner[3]],
        );
        let f1111 = get(
            &self.perm_table,
            [far_corner[0], far_corner[1], far_corner[2], far_corner[3]],
        );

        let d000 = interpolate::linear(f0000, f1000, weight[0]);
        let d010 = interpolate::linear(f0010, f1010, weight[0]);
        let d100 = interpolate::linear(f0100, f1100, weight[0]);
        let d110 = interpolate::linear(f0110, f1110, weight[0]);
        let d001 = interpolate::linear(f0001, f1001, weight[0]);
        let d011 = interpolate::linear(f0011, f1011, weight[0]);
        let d101 = interpolate::linear(f0101, f1101, weight[0]);
        let d111 = interpolate::linear(f0111, f1111, weight[0]);
        let d00 = interpolate::linear(d000, d100, weight[1]);
        let d10 = interpolate::linear(d010, d110, weight[1]);
        let d01 = interpolate::linear(d001, d101, weight[1]);
        let d11 = interpolate::linear(d011, d111, weight[1]);
        let d0 = interpolate::linear(d00, d10, weight[2]);
        let d1 = interpolate::linear(d01, d11, weight[2]);
        let d = interpolate::linear(d0, d1, weight[3]);

        d * 2.0 - 1.0
    }
}
