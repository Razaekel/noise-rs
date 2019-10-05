use crate::noise_fns::{NoiseFn, Seedable};
use crate::permutationtable::PermutationTable;
use crate::{gradient, math};

use rayon::prelude::*;

/// Noise function that outputs 2/3/4-dimensional Perlin noise.
#[derive(Clone, Copy, Debug)]
pub struct Perlin {
    pub seed: u32,
    pub perm_table: PermutationTable,
}

impl Perlin {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new() -> Self {
        Self {
            seed: Self::DEFAULT_SEED,
            perm_table: PermutationTable::new(Self::DEFAULT_SEED),
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
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
impl NoiseFn<[f64; 2]> for Perlin {
    fn get(&self, point: [f64; 2]) -> f64 {
        #[inline]
        #[cfg_attr(rustfmt, rustfmt_skip)]
        fn gradient_dot_v(perm: usize, point: [f64; 2]) -> f64 {
            let x = point[0];
            let y = point[1];

            match perm & 0b11 {
                0 =>  x + y, // ( 1,  1)
                1 => -x + y, // (-1,  1)
                2 =>  x - y, // ( 1, -1)
                3 => -x - y, // (-1, -1)
                _ => unreachable!(),
            }
        }

        // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N/4), sqrt(N/4)).
        // Need to invert this value and multiply the unscaled result by the value to get a scaled
        // range of (-1, 1).
        // 1/sqrt(N/4), N=2 -> 1/sqrt(1/2) -> sqrt(2)
        const SCALE_FACTOR: f64 = std::f64::consts::SQRT_2;

        let floored = math::map2(point, f64::floor);
        let near_corner = math::to_isize2(floored);
        let far_corner = math::add2(near_corner, [1; 2]);
        let near_distance = math::sub2(point, floored);
        let far_distance = math::sub2(near_distance, [1.0; 2]);

        let u = interpolate::s_curve5(near_distance[0]);
        let v = interpolate::s_curve5(near_distance[1]);

        let a = gradient_dot_v(self.perm_table.get2(near_corner), near_distance);
        let b = gradient_dot_v(
            self.perm_table.get2([far_corner[0], near_corner[1]]),
            [far_distance[0], near_distance[1]],
        );
        let c = gradient_dot_v(
            self.perm_table.get2([near_corner[0], far_corner[1]]),
            [near_distance[0], far_distance[1]],
        );
        let d = gradient_dot_v(self.perm_table.get2(far_corner), far_distance);

        let k0 = a;
        let k1 = b - a;
        let k2 = c - a;
        let k3 = a + d - b - c;

        let unscaled_result = k0 + k1 * u + k2 * v + k3 * u * v;

        let scaled_result = unscaled_result * SCALE_FACTOR;

        // At this point, we should be really damn close to the (-1, 1) range, but some float errors
        // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
        // outliers and return it.

        math::clamp(scaled_result, -1.0, 1.0)
    }
}

pub fn perlin_2d_basic(x: f64, y: f64, perm_table: &PermutationTable) -> f64 {
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn gradient(hash: usize, x: f64, y: f64) -> f64 {
        match hash & 0b11 {
            0 =>  x + y, // ( 1,  1)
            1 => -x + y, // (-1,  1)
            2 =>  x - y, // ( 1, -1)
            3 => -x - y, // (-1, -1)
            _ => unreachable!(),
        }
    }

    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N/4), sqrt(N/4)).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    // 1/sqrt(N/4), N=2 -> 1/sqrt(1/2) -> sqrt(2)
    const SCALE_FACTOR: f64 = std::f64::consts::SQRT_2;

    let x_floor = x.floor();
    let y_floor = y.floor();
    let delta_x = x - x_floor;
    let delta_y = y - y_floor;

    let u = interpolate::s_curve5(delta_x);
    let v = interpolate::s_curve5(delta_y);

    let a = gradient(perm_table.get2([x_floor as isize, y_floor as isize]), delta_x, delta_y);
    let b = gradient(perm_table.get2([x_floor as isize + 1, y_floor as isize]), 1.0 - delta_x, delta_y);
    let c = gradient(perm_table.get2([x_floor as isize, y_floor as isize + 1]), delta_x, 1.0 - delta_y);
    let d = gradient(perm_table.get2([x_floor as isize + 1, y_floor as isize + 1]), 1.0 - delta_x, 1.0 - delta_y);

    let k0 = a;
    let k1 = b - a;
    let k2 = c - a;
    let k3 = a + d - b - c;

    let unscaled_result = k0 + k1 * u + k2 * v + k3 * u * v;

    math::clamp(unscaled_result * SCALE_FACTOR, -1.0, 1.0)
}

pub fn perlin_2d_point(point: [f64; 2], perm_table: &PermutationTable) -> f64 {
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn gradient(perm: usize, point: [f64; 2]) -> f64 {
        let x = point[0];
        let y = point[1];

        match perm & 0b11 {
            0 =>  x + y, // ( 1,  1)
            1 => -x + y, // (-1,  1)
            2 =>  x - y, // ( 1, -1)
            3 => -x - y, // (-1, -1)
            _ => unreachable!(),
        }
    }

    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N/4), sqrt(N/4)).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    // 1/sqrt(N/4), N=2 -> 1/sqrt(1/2) -> sqrt(2)
    const SCALE_FACTOR: f64 = std::f64::consts::SQRT_2;

    let floored = math::map2(point, f64::floor);
    let near_corner = math::to_isize2(floored);
    let far_corner = math::add2(near_corner, [1; 2]);
    let near_distance = math::sub2(point, floored);
    let far_distance = math::sub2(near_distance, [1.0; 2]);

    let u = interpolate::s_curve5(near_distance[0]);
    let v = interpolate::s_curve5(near_distance[1]);

    let a = gradient(perm_table.get2(near_corner), near_distance);
    let b = gradient(
        perm_table.get2([far_corner[0], near_corner[1]]),
        [far_distance[0], near_distance[1]],
    );
    let c = gradient(
        perm_table.get2([near_corner[0], far_corner[1]]),
        [near_distance[0], far_distance[1]],
    );
    let d = gradient(perm_table.get2(far_corner), far_distance);

    let k0 = a;
    let k1 = b - a;
    let k2 = c - a;
    let k3 = a + d - b - c;

    let unscaled_result = k0 + k1 * u + k2 * v + k3 * u * v;

    let scaled_result = unscaled_result * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.

    math::clamp(scaled_result, -1.0, 1.0)
}

pub fn perlin_2d_iter(points: &[[f64; 2]], perm_table: &PermutationTable) -> Vec<f64> {
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn gradient(perm: usize, point: [f64; 2]) -> f64 {
        let x = point[0];
        let y = point[1];

        match perm & 0b11 {
            0 =>  x + y, // ( 1,  1)
            1 => -x + y, // (-1,  1)
            2 =>  x - y, // ( 1, -1)
            3 => -x - y, // (-1, -1)
            _ => unreachable!(),
        }
    }

    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N/4), sqrt(N/4)).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    // 1/sqrt(N/4), N=2 -> 1/sqrt(1/2) -> sqrt(2)
    const SCALE_FACTOR: f64 = std::f64::consts::SQRT_2;

    let floored : Vec<[f64; 2]> = points.iter().map(|[x, y]| [x.floor(), y.floor()]).collect();
    let near_corner : Vec<[isize; 2]> = floored.iter().map(|point| math::to_isize2(*point)).collect();
    let far_corner : Vec<[isize; 2]> = near_corner.iter().map(|point| math::add2(*point, [1; 2])).collect();
    let near_distance : Vec<[f64; 2]> = points.iter().zip(floored.iter()).map(|(point, floored)| math::sub2(*point, *floored)).collect();
    let far_distance : Vec<[f64; 2]> = near_distance.iter().map(|point| math::sub2(*point, [1.0; 2])).collect();

    let u : Vec<f64> = near_distance.iter().map(|[x, _]| interpolate::s_curve5(*x)).collect();
    let v : Vec<f64> = near_distance.iter().map(|[_, y]| interpolate::s_curve5(*y)).collect();

    fn compute_gradients(perm_table: &PermutationTable, corner: &[[isize; 2]], distance: &[[f64; 2]]) -> Vec<f64> {
        corner.iter().zip(distance.iter()).map(|(corner, distance)| gradient(perm_table.get2(*corner), *distance)).collect()
    }

//    let a : Vec<f64> = near_corner.iter().zip(near_distance.iter()).map(|(corner, distance)| gradient(perm_table.get2(*corner), *distance)).collect();
    let a = compute_gradients(perm_table, &near_corner, &near_distance);

    let b_corner : Vec<[isize; 2]> = near_corner.iter().zip(far_corner.iter()).map(|(near_corner, far_corner)| [far_corner[0], near_corner[1]]).collect();
    let b_distance : Vec<[f64; 2]> = near_distance.iter().zip(far_distance.iter()).map(|(near_distance, far_distance)| [far_distance[0], near_distance[1]]).collect();

//    let b : Vec<f64> = b0.iter().zip(b1.iter()).map(|(corner, distance)| gradient(perm_table.get2(*corner), *distance)).collect();
    let b = compute_gradients(perm_table, &b_corner, &b_distance);

    let c_corner : Vec<[isize; 2]> = near_corner.iter().zip(far_corner.iter()).map(|(near_corner, far_corner)| [near_corner[0], far_corner[1]]).collect();
    let c_distance : Vec<[f64; 2]> = near_distance.iter().zip(far_distance.iter()).map(|(near_distance, far_distance)| [near_distance[0], far_distance[1]]).collect();

    let c = compute_gradients(perm_table, &c_corner, &c_distance);

    let d = compute_gradients(perm_table, &far_corner, &far_distance);

//    let a = gradient(perm_table.get2(near_corner), near_distance);
//    let b = gradient(perm_table.get2([far_corner[0], near_corner[1]]), [far_distance[0], near_distance[1]]);
//    let c = gradient(
//        perm_table.get2([near_corner[0], far_corner[1]]),
//        [near_distance[0], far_distance[1]],
//    );
//    let d = gradient(perm_table.get2(far_corner), far_distance);

    let k0 = &a;
    let k1 : Vec<f64> = b.iter().zip(a.iter()).map(|(b, a)| b - a).collect();
    let k2 : Vec<f64> = c.iter().zip(a.iter()).map(|(c, a)| c - a).collect();
    let k3 : Vec<f64> = a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter()).map(|(((a, b), c), d)| a + d - b - c).collect();

    let unscaled_result = k0.iter().zip(k1.iter()).zip(k2.iter()).zip(k3.iter()).zip(u.iter()).zip(v.iter()).map(|(((((k0, k1), k2), k3), u), v)| k0 + k1 * u + k2 * v + k3 * u * v);

    let scaled_result = unscaled_result.map(|x| x * SCALE_FACTOR).collect();

    return scaled_result;
/*
    let k0 = a;
    let k1 = b - a;
    let k2 = c - a;
    let k3 = a + d - b - c;

    let unscaled_result = k0 + k1 * u + k2 * v + k3 * u * v;

    let scaled_result = unscaled_result * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.

    math::clamp(scaled_result, -1.0, 1.0)
    */
}

pub fn perlin_2d_alt_iter(points: &[[f64; 2]], perm_table: &PermutationTable) -> Vec<f64> {
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn gradient(perm: usize, point: [f64; 2]) -> f64 {
        let x = point[0];
        let y = point[1];

        match perm & 0b11 {
            0 =>  x + y, // ( 1,  1)
            1 => -x + y, // (-1,  1)
            2 =>  x - y, // ( 1, -1)
            3 => -x - y, // (-1, -1)
            _ => unreachable!(),
        }
    }

    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N/4), sqrt(N/4)).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    // 1/sqrt(N/4), N=2 -> 1/sqrt(1/2) -> sqrt(2)
    const SCALE_FACTOR: f64 = std::f64::consts::SQRT_2;
    let results : Vec<f64> = points.iter().map(|point| {
        let floored = math::map2(*point, f64::floor);
        let near_corner = math::to_isize2(floored);
        let far_corner = math::add2(near_corner, [1; 2]);
        let near_distance = math::sub2(*point, floored);
        let far_distance = math::sub2(near_distance, [1.0; 2]);

        let u = interpolate::s_curve5(near_distance[0]);
        let v = interpolate::s_curve5(near_distance[1]);

        let a = gradient(perm_table.get2(near_corner), near_distance);
        let b = gradient(
            perm_table.get2([far_corner[0], near_corner[1]]),
            [far_distance[0], near_distance[1]],
        );
        let c = gradient(
            perm_table.get2([near_corner[0], far_corner[1]]),
            [near_distance[0], far_distance[1]],
        );
        let d = gradient(perm_table.get2(far_corner), far_distance);

        let k0 = a;
        let k1 = b - a;
        let k2 = c - a;
        let k3 = a + d - b - c;

        let unscaled_result = k0 + k1 * u + k2 * v + k3 * u * v;

        let scaled_result = unscaled_result * SCALE_FACTOR;

        // At this point, we should be really damn close to the (-1, 1) range, but some float errors
        // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
        // outliers and return it.

        math::clamp(scaled_result, -1.0, 1.0)
    }).collect();

    results
}

pub fn perlin_2d_par_iter(points: &[[f64; 2]], perm_table: &PermutationTable) -> Vec<f64> {
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn gradient(perm: usize, point: [f64; 2]) -> f64 {
        let x = point[0];
        let y = point[1];

        match perm & 0b11 {
            0 =>  x + y, // ( 1,  1)
            1 => -x + y, // (-1,  1)
            2 =>  x - y, // ( 1, -1)
            3 => -x - y, // (-1, -1)
            _ => unreachable!(),
        }
    }

    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N/4), sqrt(N/4)).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    // 1/sqrt(N/4), N=2 -> 1/sqrt(1/2) -> sqrt(2)
    const SCALE_FACTOR: f64 = std::f64::consts::SQRT_2;
    let results : Vec<f64> = points.par_iter().map(|point| {
        let floored = math::map2(*point, f64::floor);
        let near_corner = math::to_isize2(floored);
        let far_corner = math::add2(near_corner, [1; 2]);
        let near_distance = math::sub2(*point, floored);
        let far_distance = math::sub2(near_distance, [1.0; 2]);

        let u = interpolate::s_curve5(near_distance[0]);
        let v = interpolate::s_curve5(near_distance[1]);

        let a = gradient(perm_table.get2(near_corner), near_distance);
        let b = gradient(
            perm_table.get2([far_corner[0], near_corner[1]]),
            [far_distance[0], near_distance[1]],
        );
        let c = gradient(
            perm_table.get2([near_corner[0], far_corner[1]]),
            [near_distance[0], far_distance[1]],
        );
        let d = gradient(perm_table.get2(far_corner), far_distance);

        let k0 = a;
        let k1 = b - a;
        let k2 = c - a;
        let k3 = a + d - b - c;

        let unscaled_result = k0 + k1 * u + k2 * v + k3 * u * v;

        let scaled_result = unscaled_result * SCALE_FACTOR;

        // At this point, we should be really damn close to the (-1, 1) range, but some float errors
        // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
        // outliers and return it.

        math::clamp(scaled_result, -1.0, 1.0)
    }).collect();

    results
}

/// 3-dimensional perlin noise
impl NoiseFn<[f64; 3]> for Perlin {
    fn get(&self, point: [f64; 3]) -> f64 {
        #[inline]
        #[cfg_attr(rustfmt, rustfmt_skip)]
        fn gradient_dot_v(perm: usize, point: [f64; 3]) -> f64 {
            let x = point[0];
            let y = point[1];
            let z = point[2];

            match perm & 0b1111 {
                 0 =>  x + y, // ( 1,  1,  0)
                 1 => -x + y, // (-1,  1,  0)
                 2 =>  x - y, // ( 1, -1,  0)
                 3 => -x - y, // (-1, -1,  0)
                 4 =>  x + z, // ( 1,  0,  1)
                 5 => -x + z, // (-1,  0,  1)
                 6 =>  x - z, // ( 1,  0, -1)
                 7 => -x - z, // (-1,  0, -1)
                 8 =>  y + z, // ( 0,  1,  1)
                 9 => -y + z, // ( 0, -1,  1)
                10 =>  y - z, // ( 0,  1, -1)
                11 => -y - z, // ( 0, -1, -1)
                12 =>  x + y, // ( 1,  1,  0)
                13 => -x + y, // (-1,  1,  0)
                14 => -y + z, // ( 0, -1,  1)
                15 => -y - z, // ( 0, -1, -1)
                _ => unreachable!(),
            }
        }

        // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N/4), sqrt(N/4)).
        // Need to invert this value and multiply the unscaled result by the value to get a scaled
        // range of (-1, 1).
        // 1/sqrt(N/4), N=3 -> 1/sqrt(3/4) -> 2/sqrt(3)
//        const SCALE_FACTOR: f64 = 2.0_f64/((3.0_f64).sqrt());
        const SCALE_FACTOR: f64 = 1.154_700_538_379_251_529_018;

        let floored = math::map3(point, f64::floor);
        let near_corner = math::to_isize3(floored);
        let far_corner = math::add3(near_corner, [1; 3]);
        let near_distance = math::sub3(point, floored);
        let far_distance = math::sub3(near_distance, [1.0; 3]);

        let u = interpolate::s_curve5(near_distance[0]);
        let v = interpolate::s_curve5(near_distance[1]);
        let w = interpolate::s_curve5(near_distance[2]);

        let a = gradient_dot_v(self.perm_table.get3(near_corner), near_distance);
        let b = gradient_dot_v(
            self.perm_table
                .get3([far_corner[0], near_corner[1], near_corner[2]]),
            [far_distance[0], near_distance[1], near_distance[2]],
        );
        let c = gradient_dot_v(
            self.perm_table
                .get3([near_corner[0], far_corner[1], near_corner[2]]),
            [near_distance[0], far_distance[1], near_distance[2]],
        );
        let d = gradient_dot_v(
            self.perm_table
                .get3([far_corner[0], far_corner[1], near_corner[2]]),
            [far_distance[0], far_distance[1], near_distance[2]],
        );
        let e = gradient_dot_v(
            self.perm_table
                .get3([near_corner[0], near_corner[1], far_corner[2]]),
            [near_distance[0], near_distance[1], far_distance[2]],
        );
        let f = gradient_dot_v(
            self.perm_table
                .get3([far_corner[0], near_corner[1], far_corner[2]]),
            [far_distance[0], near_distance[1], far_distance[2]],
        );
        let g = gradient_dot_v(
            self.perm_table
                .get3([near_corner[0], far_corner[1], far_corner[2]]),
            [near_distance[0], far_distance[1], far_distance[2]],
        );
        let h = gradient_dot_v(self.perm_table.get3(far_corner), far_distance);

        let k0 = a;
        let k1 = b - a;
        let k2 = c - a;
        let k3 = e - a;
        let k4 = a + d - b - c;
        let k5 = a + f - b - e;
        let k6 = a + g - c - e;
        let k7 = b + c + e + h - a - d - f - g;

        let unscaled_result = k0 + k1 * u + k2 * v + k3 * w + k4 * u * v + k5 * u * w + k6 * v * w + k7 * u * v * w;

        let scaled_result = unscaled_result * SCALE_FACTOR;

        // At this point, we should be really damn close to the (-1, 1) range, but some float errors
        // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
        // outliers and return it.

        math::clamp(scaled_result, -1.0, 1.0)
    }
}

/// 4-dimensional perlin noise
impl NoiseFn<[f64; 4]> for Perlin {
    fn get(&self, point: [f64; 4]) -> f64 {
        #[inline(always)]
        fn surflet(perm_table: &PermutationTable, corner: [isize; 4], distance: [f64; 4]) -> f64 {
            let attn = 1.0 - math::dot4(distance, distance);
            if attn > 0.0 {
                attn.powi(4) * math::dot4(distance, gradient::get4(perm_table.get4(corner)))
            } else {
                0.0
            }
        }

        // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N/4), sqrt(N/4)).
        // Need to invert this value and multiply the unscaled result by the value to get a scaled
        // range of (-1, 1).
        const SCALE_FACTOR: f64 = 1.0; // 1/sqrt(N/4), N=4 -> 1/sqrt(1) -> sqrt(1) -> 1

        let floored = math::map4(point, f64::floor);
        let near_corner = math::to_isize4(floored);
        let far_corner = math::add4(near_corner, math::one4());
        let near_distance = math::sub4(point, floored);
        let far_distance = math::sub4(near_distance, math::one4());

        let f0000 = surflet(
            &self.perm_table,
            [
                near_corner[0],
                near_corner[1],
                near_corner[2],
                near_corner[3],
            ],
            [
                near_distance[0],
                near_distance[1],
                near_distance[2],
                near_distance[3],
            ],
        );
        let f1000 = surflet(
            &self.perm_table,
            [
                far_corner[0],
                near_corner[1],
                near_corner[2],
                near_corner[3],
            ],
            [
                far_distance[0],
                near_distance[1],
                near_distance[2],
                near_distance[3],
            ],
        );
        let f0100 = surflet(
            &self.perm_table,
            [
                near_corner[0],
                far_corner[1],
                near_corner[2],
                near_corner[3],
            ],
            [
                near_distance[0],
                far_distance[1],
                near_distance[2],
                near_distance[3],
            ],
        );
        let f1100 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1], near_corner[2], near_corner[3]],
            [
                far_distance[0],
                far_distance[1],
                near_distance[2],
                near_distance[3],
            ],
        );
        let f0010 = surflet(
            &self.perm_table,
            [
                near_corner[0],
                near_corner[1],
                far_corner[2],
                near_corner[3],
            ],
            [
                near_distance[0],
                near_distance[1],
                far_distance[2],
                near_distance[3],
            ],
        );
        let f1010 = surflet(
            &self.perm_table,
            [far_corner[0], near_corner[1], far_corner[2], near_corner[3]],
            [
                far_distance[0],
                near_distance[1],
                far_distance[2],
                near_distance[3],
            ],
        );
        let f0110 = surflet(
            &self.perm_table,
            [near_corner[0], far_corner[1], far_corner[2], near_corner[3]],
            [
                near_distance[0],
                far_distance[1],
                far_distance[2],
                near_distance[3],
            ],
        );
        let f1110 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1], far_corner[2], near_corner[3]],
            [
                far_distance[0],
                far_distance[1],
                far_distance[2],
                near_distance[3],
            ],
        );
        let f0001 = surflet(
            &self.perm_table,
            [
                near_corner[0],
                near_corner[1],
                near_corner[2],
                far_corner[3],
            ],
            [
                near_distance[0],
                near_distance[1],
                near_distance[2],
                far_distance[3],
            ],
        );
        let f1001 = surflet(
            &self.perm_table,
            [far_corner[0], near_corner[1], near_corner[2], far_corner[3]],
            [
                far_distance[0],
                near_distance[1],
                near_distance[2],
                far_distance[3],
            ],
        );
        let f0101 = surflet(
            &self.perm_table,
            [near_corner[0], far_corner[1], near_corner[2], far_corner[3]],
            [
                near_distance[0],
                far_distance[1],
                near_distance[2],
                far_distance[3],
            ],
        );
        let f1101 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1], near_corner[2], far_corner[3]],
            [
                far_distance[0],
                far_distance[1],
                near_distance[2],
                far_distance[3],
            ],
        );
        let f0011 = surflet(
            &self.perm_table,
            [near_corner[0], near_corner[1], far_corner[2], far_corner[3]],
            [
                near_distance[0],
                near_distance[1],
                far_distance[2],
                far_distance[3],
            ],
        );
        let f1011 = surflet(
            &self.perm_table,
            [far_corner[0], near_corner[1], far_corner[2], far_corner[3]],
            [
                far_distance[0],
                near_distance[1],
                far_distance[2],
                far_distance[3],
            ],
        );
        let f0111 = surflet(
            &self.perm_table,
            [near_corner[0], far_corner[1], far_corner[2], far_corner[3]],
            [
                near_distance[0],
                far_distance[1],
                far_distance[2],
                far_distance[3],
            ],
        );
        let f1111 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1], far_corner[2], far_corner[3]],
            [
                far_distance[0],
                far_distance[1],
                far_distance[2],
                far_distance[3],
            ],
        );

        // Multiply by arbitrary value to scale to -1..1
        math::clamp(
            (f0000
                + f1000
                + f0100
                + f1100
                + f0010
                + f1010
                + f0110
                + f1110
                + f0001
                + f1001
                + f0101
                + f1101
                + f0011
                + f1011
                + f0111
                + f1111)
                * SCALE_FACTOR,
            -1.0,
            1.0,
        )
    }
}
