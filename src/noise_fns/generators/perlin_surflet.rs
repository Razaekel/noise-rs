use {gradient, math};
use math::{Point2, Point3, Point4, Vector2, Vector3, Vector4};
use noise_fns::{NoiseFn, Seedable};
use permutationtable::PermutationTable;

/// Noise function that outputs 2/3/4-dimensional PerlinSurflet noise.
#[derive(Clone, Copy, Debug)]
pub struct PerlinSurflet {
    seed: u32,
    perm_table: PermutationTable,
}

impl PerlinSurflet {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new() -> Self {
        PerlinSurflet {
            seed: Self::DEFAULT_SEED,
            perm_table: PermutationTable::new(Self::DEFAULT_SEED),
        }
    }
}

impl Default for PerlinSurflet {
    fn default() -> Self {
        Self::new()
    }
}

impl Seedable for PerlinSurflet {
    /// Sets the seed value for PerlinSurflet noise
    fn set_seed(self, seed: u32) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        PerlinSurflet {
            seed,
            perm_table: PermutationTable::new(seed),
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional PerlinSurflet noise
impl NoiseFn<Point2<f64>> for PerlinSurflet {
    fn get(&self, point: Point2<f64>) -> f64 {
        #[inline(always)]
        fn surflet(
            perm_table: &PermutationTable,
            corner: Point2<isize>,
            distance: Vector2<f64>,
        ) -> f64 {
            let attn = 1.0 - math::dot2(distance, distance);
            if attn > 0.0 {
                attn.powi(4) * math::dot2(distance, gradient::get2(perm_table.get2(corner)))
            } else {
                0.0
            }
        }

        let floored = math::map2(point, f64::floor);
        let near_corner = math::to_isize2(floored);
        let far_corner = math::add2(near_corner, math::one2());
        let near_distance = math::sub2(point, floored);
        let far_distance = math::sub2(near_distance, math::one2());

        let f00 = surflet(
            &self.perm_table,
            [near_corner[0], near_corner[1]],
            [near_distance[0], near_distance[1]],
        );
        let f10 = surflet(
            &self.perm_table,
            [far_corner[0], near_corner[1]],
            [far_distance[0], near_distance[1]],
        );
        let f01 = surflet(
            &self.perm_table,
            [near_corner[0], far_corner[1]],
            [near_distance[0], far_distance[1]],
        );
        let f11 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1]],
            [far_distance[0], far_distance[1]],
        );

        // Multiply by arbitrary value to scale to -1..1
        math::clamp((f00 + f10 + f01 + f11) * 3.1604938271604937, -1.0, 1.0)
    }
}

/// 3-dimensional PerlinSurflet noise
impl NoiseFn<Point3<f64>> for PerlinSurflet {
    fn get(&self, point: Point3<f64>) -> f64 {
        #[inline(always)]
        fn surflet(
            perm_table: &PermutationTable,
            corner: Point3<isize>,
            distance: Vector3<f64>,
        ) -> f64 {
            let attn = 1.0 - math::dot3(distance, distance);
            if attn > 0.0 {
                attn.powi(4) * math::dot3(distance, gradient::get3(perm_table.get3(corner)))
            } else {
                0.0
            }
        }

        let floored = math::map3(point, f64::floor);
        let near_corner = math::to_isize3(floored);
        let far_corner = math::add3(near_corner, math::one3());
        let near_distance = math::sub3(point, floored);
        let far_distance = math::sub3(near_distance, math::one3());

        let f000 = surflet(
            &self.perm_table,
            [near_corner[0], near_corner[1], near_corner[2]],
            [near_distance[0], near_distance[1], near_distance[2]],
        );
        let f100 = surflet(
            &self.perm_table,
            [far_corner[0], near_corner[1], near_corner[2]],
            [far_distance[0], near_distance[1], near_distance[2]],
        );
        let f010 = surflet(
            &self.perm_table,
            [near_corner[0], far_corner[1], near_corner[2]],
            [near_distance[0], far_distance[1], near_distance[2]],
        );
        let f110 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1], near_corner[2]],
            [far_distance[0], far_distance[1], near_distance[2]],
        );
        let f001 = surflet(
            &self.perm_table,
            [near_corner[0], near_corner[1], far_corner[2]],
            [near_distance[0], near_distance[1], far_distance[2]],
        );
        let f101 = surflet(
            &self.perm_table,
            [far_corner[0], near_corner[1], far_corner[2]],
            [far_distance[0], near_distance[1], far_distance[2]],
        );
        let f011 = surflet(
            &self.perm_table,
            [near_corner[0], far_corner[1], far_corner[2]],
            [near_distance[0], far_distance[1], far_distance[2]],
        );
        let f111 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1], far_corner[2]],
            [far_distance[0], far_distance[1], far_distance[2]],
        );

        // Multiply by arbitrary value to scale to -1..1
        math::clamp(
            (f000 + f100 + f010 + f110 + f001 + f101 + f011 + f111) * 3.8898553255531074,
            -1.0,
            1.0,
        )
    }
}

/// 4-dimensional PerlinSurflet noise
impl NoiseFn<Point4<f64>> for PerlinSurflet {
    fn get(&self, point: Point4<f64>) -> f64 {
        #[inline(always)]
        fn surflet(
            perm_table: &PermutationTable,
            corner: Point4<isize>,
            distance: Vector4<f64>,
        ) -> f64 {
            let attn = 1.0 - math::dot4(distance, distance);
            if attn > 0.0 {
                attn.powi(4) * math::dot4(distance, gradient::get4(perm_table.get4(corner)))
            } else {
                0.0
            }
        }

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
            (f0000 + f1000 + f0100 + f1100 + f0010 + f1010 + f0110 + f1110 + f0001 + f1001 + f0101
                + f1101 + f0011 + f1011 + f0111 + f1111) * 4.424369240215691,
            -1.0,
            1.0,
        )
    }
}
