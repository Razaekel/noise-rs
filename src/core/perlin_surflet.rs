use crate::{
    math,
    gradient,
    permutationtable::NoiseHasher,
};

#[inline(always)]
pub fn perlin_surflet_2d(point: [f64; 2], hasher: &dyn NoiseHasher) -> f64 {
    const SCALE_FACTOR: f64 = 3.160_493_827_160_493_7;

    #[inline(always)]
    fn surflet(hasher: &dyn NoiseHasher, corner: [isize; 2], distance: [f64; 2]) -> f64 {
        let attn: f64 = 1.0 - math::dot2(distance, distance);
        if attn > 0.0 {
            attn.powi(4) * math::dot2(distance, gradient::grad2(hasher.hash(&corner)))
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
        hasher,
        [near_corner[0], near_corner[1]],
        [near_distance[0], near_distance[1]],
    );
    let f10 = surflet(
        hasher,
        [far_corner[0], near_corner[1]],
        [far_distance[0], near_distance[1]],
    );
    let f01 = surflet(
        hasher,
        [near_corner[0], far_corner[1]],
        [near_distance[0], far_distance[1]],
    );
    let f11 = surflet(
        hasher,
        [far_corner[0], far_corner[1]],
        [far_distance[0], far_distance[1]],
    );

    // Multiply by arbitrary value to scale to -1..1
    ((f00 + f10 + f01 + f11) * SCALE_FACTOR).clamp(-1.0, 1.0)
}

pub fn perlin_surflet_3d(point: [f64; 3], hasher: &dyn NoiseHasher) -> f64 {
    const SCALE_FACTOR: f64 = 3.889_855_325_553_107_4;

    #[inline(always)]
    fn surflet(hasher: &dyn NoiseHasher, corner: [isize; 3], distance: [f64; 3]) -> f64 {
        let attn: f64 = 1.0 - math::dot3(distance, distance);
        if attn > 0.0 {
            attn.powi(4) * math::dot3(distance, gradient::grad3(hasher.hash(&corner)))
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
        hasher,
        [near_corner[0], near_corner[1], near_corner[2]],
        [near_distance[0], near_distance[1], near_distance[2]],
    );
    let f100 = surflet(
        hasher,
        [far_corner[0], near_corner[1], near_corner[2]],
        [far_distance[0], near_distance[1], near_distance[2]],
    );
    let f010 = surflet(
        hasher,
        [near_corner[0], far_corner[1], near_corner[2]],
        [near_distance[0], far_distance[1], near_distance[2]],
    );
    let f110 = surflet(
        hasher,
        [far_corner[0], far_corner[1], near_corner[2]],
        [far_distance[0], far_distance[1], near_distance[2]],
    );
    let f001 = surflet(
        hasher,
        [near_corner[0], near_corner[1], far_corner[2]],
        [near_distance[0], near_distance[1], far_distance[2]],
    );
    let f101 = surflet(
        hasher,
        [far_corner[0], near_corner[1], far_corner[2]],
        [far_distance[0], near_distance[1], far_distance[2]],
    );
    let f011 = surflet(
        hasher,
        [near_corner[0], far_corner[1], far_corner[2]],
        [near_distance[0], far_distance[1], far_distance[2]],
    );
    let f111 = surflet(
        hasher,
        [far_corner[0], far_corner[1], far_corner[2]],
        [far_distance[0], far_distance[1], far_distance[2]],
    );

    // Multiply by arbitrary value to scale to -1..1
    ((f000 + f100 + f010 + f110 + f001 + f101 + f011 + f111) * SCALE_FACTOR).clamp(-1.0, 1.0)
}

pub fn perlin_surflet_4d(point: [f64; 4], hasher: &dyn NoiseHasher) -> f64 {
    const SCALE_FACTOR: f64 = 4.424_369_240_215_691;

    #[inline(always)]
    fn surflet(hasher: &dyn NoiseHasher, corner: [isize; 4], distance: [f64; 4]) -> f64 {
        let attn: f64 = 1.0 - math::dot4(distance, distance);
        if attn > 0.0 {
            attn.powi(4) * math::dot4(distance, gradient::grad4(hasher.hash(&corner)))
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
        hasher,
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
        hasher,
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
        hasher,
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
        hasher,
        [far_corner[0], far_corner[1], near_corner[2], near_corner[3]],
        [
            far_distance[0],
            far_distance[1],
            near_distance[2],
            near_distance[3],
        ],
    );
    let f0010 = surflet(
        hasher,
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
        hasher,
        [far_corner[0], near_corner[1], far_corner[2], near_corner[3]],
        [
            far_distance[0],
            near_distance[1],
            far_distance[2],
            near_distance[3],
        ],
    );
    let f0110 = surflet(
        hasher,
        [near_corner[0], far_corner[1], far_corner[2], near_corner[3]],
        [
            near_distance[0],
            far_distance[1],
            far_distance[2],
            near_distance[3],
        ],
    );
    let f1110 = surflet(
        hasher,
        [far_corner[0], far_corner[1], far_corner[2], near_corner[3]],
        [
            far_distance[0],
            far_distance[1],
            far_distance[2],
            near_distance[3],
        ],
    );
    let f0001 = surflet(
        hasher,
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
        hasher,
        [far_corner[0], near_corner[1], near_corner[2], far_corner[3]],
        [
            far_distance[0],
            near_distance[1],
            near_distance[2],
            far_distance[3],
        ],
    );
    let f0101 = surflet(
        hasher,
        [near_corner[0], far_corner[1], near_corner[2], far_corner[3]],
        [
            near_distance[0],
            far_distance[1],
            near_distance[2],
            far_distance[3],
        ],
    );
    let f1101 = surflet(
        hasher,
        [far_corner[0], far_corner[1], near_corner[2], far_corner[3]],
        [
            far_distance[0],
            far_distance[1],
            near_distance[2],
            far_distance[3],
        ],
    );
    let f0011 = surflet(
        hasher,
        [near_corner[0], near_corner[1], far_corner[2], far_corner[3]],
        [
            near_distance[0],
            near_distance[1],
            far_distance[2],
            far_distance[3],
        ],
    );
    let f1011 = surflet(
        hasher,
        [far_corner[0], near_corner[1], far_corner[2], far_corner[3]],
        [
            far_distance[0],
            near_distance[1],
            far_distance[2],
            far_distance[3],
        ],
    );
    let f0111 = surflet(
        hasher,
        [near_corner[0], far_corner[1], far_corner[2], far_corner[3]],
        [
            near_distance[0],
            far_distance[1],
            far_distance[2],
            far_distance[3],
        ],
    );
    let f1111 = surflet(
        hasher,
        [far_corner[0], far_corner[1], far_corner[2], far_corner[3]],
        [
            far_distance[0],
            far_distance[1],
            far_distance[2],
            far_distance[3],
        ],
    );

    // Multiply by arbitrary value to scale to -1..1
    ((f0000
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
        * SCALE_FACTOR)
        .clamp(-1.0, 1.0)
}