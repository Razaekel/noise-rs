// Copyright 2015 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num::Float;

use {gradient, math, Seed};

/// 2-dimensional perlin noise
pub fn perlin2<T: Float>(seed: &Seed, point: &math::Point2<T>) -> T {
    #[inline(always)]
    fn surflet<T: Float>(seed: &Seed, corner: math::Point2<isize>, distance: math::Vector2<T>) -> T {
        let attn = T::one() - math::dot2(distance, distance);
        if attn > T::zero() {
            math::pow4(attn) * math::dot2(distance, gradient::get2(seed.get2(corner)))
        } else {
            T::zero()
        }
    }

    let floored = math::map2(*point, Float::floor);
    let near_corner = math::map2(floored, math::cast);
    let far_corner = math::add2(near_corner, math::one2());
    let near_distance = math::sub2(*point, floored);
    let far_distance = math::sub2(near_distance, math::one2());

    let f00 = surflet(seed,
                      [near_corner[0], near_corner[1]],
                      [near_distance[0], near_distance[1]]);
    let f10 = surflet(seed,
                      [far_corner[0], near_corner[1]],
                      [far_distance[0], near_distance[1]]);
    let f01 = surflet(seed,
                      [near_corner[0], far_corner[1]],
                      [near_distance[0], far_distance[1]]);
    let f11 = surflet(seed,
                      [far_corner[0], far_corner[1]],
                      [far_distance[0], far_distance[1]]);

    // Multiply by arbitrary value to scale to -1..1
    (f00 + f10 + f01 + f11) * math::cast(3.1604938271604937)
}

/// 3-dimensional perlin noise
pub fn perlin3<T: Float>(seed: &Seed, point: &math::Point3<T>) -> T {
    #[inline(always)]
    fn surflet<T: Float>(seed: &Seed, corner: math::Point3<isize>, distance: math::Vector3<T>) -> T {
        let attn = T::one() - math::dot3(distance, distance);
        if attn > T::zero() {
            math::pow4(attn) * math::dot3(distance, gradient::get3(seed.get3(corner)))
        } else {
            T::zero()
        }
    }

    let floored = math::map3(*point, Float::floor);
    let near_corner = math::map3(floored, math::cast);
    let far_corner = math::add3(near_corner, math::one3());
    let near_distance = math::sub3(*point, floored);
    let far_distance = math::sub3(near_distance, math::one3());

    let f000 = surflet(seed,
                       [near_corner[0], near_corner[1], near_corner[2]],
                       [near_distance[0], near_distance[1], near_distance[2]]);
    let f100 = surflet(seed,
                       [far_corner[0], near_corner[1], near_corner[2]],
                       [far_distance[0], near_distance[1], near_distance[2]]);
    let f010 = surflet(seed,
                       [near_corner[0], far_corner[1], near_corner[2]],
                       [near_distance[0], far_distance[1], near_distance[2]]);
    let f110 = surflet(seed,
                       [far_corner[0], far_corner[1], near_corner[2]],
                       [far_distance[0], far_distance[1], near_distance[2]]);
    let f001 = surflet(seed,
                       [near_corner[0], near_corner[1], far_corner[2]],
                       [near_distance[0], near_distance[1], far_distance[2]]);
    let f101 = surflet(seed,
                       [far_corner[0], near_corner[1], far_corner[2]],
                       [far_distance[0], near_distance[1], far_distance[2]]);
    let f011 = surflet(seed,
                       [near_corner[0], far_corner[1], far_corner[2]],
                       [near_distance[0], far_distance[1], far_distance[2]]);
    let f111 = surflet(seed,
                       [far_corner[0], far_corner[1], far_corner[2]],
                       [far_distance[0], far_distance[1], far_distance[2]]);

    // Multiply by arbitrary value to scale to -1..1
    (f000 + f100 + f010 + f110 + f001 + f101 + f011 + f111) * math::cast(3.8898553255531074)
}

/// 4-dimensional perlin noise
pub fn perlin4<T: Float>(seed: &Seed, point: &math::Point4<T>) -> T {
    #[inline(always)]
    fn surflet<T: Float>(seed: &Seed, corner: math::Point4<isize>, distance: math::Vector4<T>) -> T {
        let attn = T::one() - math::dot4(distance, distance);
        if attn > T::zero() {
            math::pow4(attn) * math::dot4(distance, gradient::get4(seed.get4(corner)))
        } else {
            T::zero()
        }
    }

    let floored = math::map4(*point, Float::floor);
    let near_corner = math::map4(floored, math::cast);
    let far_corner = math::add4(near_corner, math::one4());
    let near_distance = math::sub4(*point, floored);
    let far_distance = math::sub4(near_distance, math::one4());

    let f0000 = surflet(seed,
                        [near_corner[0], near_corner[1], near_corner[2], near_corner[3]],
                        [near_distance[0], near_distance[1], near_distance[2], near_distance[3]]);
    let f1000 = surflet(seed,
                        [far_corner[0], near_corner[1], near_corner[2], near_corner[3]],
                        [far_distance[0], near_distance[1], near_distance[2], near_distance[3]]);
    let f0100 = surflet(seed,
                        [near_corner[0], far_corner[1], near_corner[2], near_corner[3]],
                        [near_distance[0], far_distance[1], near_distance[2], near_distance[3]]);
    let f1100 = surflet(seed,
                        [far_corner[0], far_corner[1], near_corner[2], near_corner[3]],
                        [far_distance[0], far_distance[1], near_distance[2], near_distance[3]]);
    let f0010 = surflet(seed,
                        [near_corner[0], near_corner[1], far_corner[2], near_corner[3]],
                        [near_distance[0], near_distance[1], far_distance[2], near_distance[3]]);
    let f1010 = surflet(seed,
                        [far_corner[0], near_corner[1], far_corner[2], near_corner[3]],
                        [far_distance[0], near_distance[1], far_distance[2], near_distance[3]]);
    let f0110 = surflet(seed,
                        [near_corner[0], far_corner[1], far_corner[2], near_corner[3]],
                        [near_distance[0], far_distance[1], far_distance[2], near_distance[3]]);
    let f1110 = surflet(seed,
                        [far_corner[0], far_corner[1], far_corner[2], near_corner[3]],
                        [far_distance[0], far_distance[1], far_distance[2], near_distance[3]]);
    let f0001 = surflet(seed,
                        [near_corner[0], near_corner[1], near_corner[2], far_corner[3]],
                        [near_distance[0], near_distance[1], near_distance[2], far_distance[3]]);
    let f1001 = surflet(seed,
                        [far_corner[0], near_corner[1], near_corner[2], far_corner[3]],
                        [far_distance[0], near_distance[1], near_distance[2], far_distance[3]]);
    let f0101 = surflet(seed,
                        [near_corner[0], far_corner[1], near_corner[2], far_corner[3]],
                        [near_distance[0], far_distance[1], near_distance[2], far_distance[3]]);
    let f1101 = surflet(seed,
                        [far_corner[0], far_corner[1], near_corner[2], far_corner[3]],
                        [far_distance[0], far_distance[1], near_distance[2], far_distance[3]]);
    let f0011 = surflet(seed,
                        [near_corner[0], near_corner[1], far_corner[2], far_corner[3]],
                        [near_distance[0], near_distance[1], far_distance[2], far_distance[3]]);
    let f1011 = surflet(seed,
                        [far_corner[0], near_corner[1], far_corner[2], far_corner[3]],
                        [far_distance[0], near_distance[1], far_distance[2], far_distance[3]]);
    let f0111 = surflet(seed,
                        [near_corner[0], far_corner[1], far_corner[2], far_corner[3]],
                        [near_distance[0], far_distance[1], far_distance[2], far_distance[3]]);
    let f1111 = surflet(seed,
                        [far_corner[0], far_corner[1], far_corner[2], far_corner[3]],
                        [far_distance[0], far_distance[1], far_distance[2], far_distance[3]]);

    // Multiply by arbitrary value to scale to -1..1
    (f0000 + f1000 + f0100 + f1100 + f0010 + f1010 + f0110 + f1110 + f0001 + f1001 +
     f0101 + f1101 + f0011 + f1011 + f0111 + f1111) * math::cast(4.424369240215691)
}
