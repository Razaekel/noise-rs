// Copyright 2015 The noise-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

use std::num::Float;

use {gradient, math, Seed};

/// 2-dimensional perlin noise
pub fn perlin2<T: Float>(seed: &Seed, point: &math::Point2<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: &Seed, whole: math::Point2<isize>, frac: math::Vector2<T>) -> T {
        math::dot2(frac, gradient::get2(seed.get2(whole)))
    }

    let floored = math::map2(*point, Float::floor);
    let whole0  = math::map2(*point, math::cast);
    let whole1  = math::add2(whole0, math::one2());
    let frac0   = math::sub2(*point, floored);
    let frac1   = math::sub2(frac0, math::one2());
    let curve   = math::map2(frac0, math::scurve5);

    let f00 = gradient(seed, [whole0[0], whole0[1]], [frac0[0], frac0[1]]);
    let f10 = gradient(seed, [whole1[0], whole0[1]], [frac1[0], frac0[1]]);
    let f01 = gradient(seed, [whole0[0], whole1[1]], [frac0[0], frac1[1]]);
    let f11 = gradient(seed, [whole1[0], whole1[1]], [frac1[0], frac1[1]]);

    math::bilerp(curve, f00, f10, f01, f11)
}

/// 3-dimensional perlin noise
pub fn perlin3<T: Float>(seed: &Seed, point: &math::Point3<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: &Seed, whole: math::Point3<isize>, frac: math::Vector3<T>) -> T {
        math::dot3(frac, gradient::get3(seed.get3(whole)))
    }

    let floored = math::map3(*point, Float::floor);
    let whole0  = math::map3(*point, math::cast);
    let whole1  = math::add3(whole0, math::one3());
    let frac0   = math::sub3(*point, floored);
    let frac1   = math::sub3(frac0, math::one3());
    let curve   = math::map3(frac0, math::scurve5);

    let f000 = gradient(seed, [whole0[0], whole0[1], whole0[2]], [frac0[0], frac0[1], frac0[2]]);
    let f100 = gradient(seed, [whole1[0], whole0[1], whole0[2]], [frac1[0], frac0[1], frac0[2]]);
    let f010 = gradient(seed, [whole0[0], whole1[1], whole0[2]], [frac0[0], frac1[1], frac0[2]]);
    let f110 = gradient(seed, [whole1[0], whole1[1], whole0[2]], [frac1[0], frac1[1], frac0[2]]);
    let f001 = gradient(seed, [whole0[0], whole0[1], whole1[2]], [frac0[0], frac0[1], frac1[2]]);
    let f101 = gradient(seed, [whole1[0], whole0[1], whole1[2]], [frac1[0], frac0[1], frac1[2]]);
    let f011 = gradient(seed, [whole0[0], whole1[1], whole1[2]], [frac0[0], frac1[1], frac1[2]]);
    let f111 = gradient(seed, [whole1[0], whole1[1], whole1[2]], [frac1[0], frac1[1], frac1[2]]);

    math::trilerp(curve, f000, f100, f010, f110, f001, f101, f011, f111)
}

/// 4-dimensional perlin noise
pub fn perlin4<T: Float>(seed: &Seed, point: &math::Point4<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: &Seed, whole: math::Point4<isize>, frac: math::Vector4<T>) -> T {
        math::dot4(frac, gradient::get4(seed.get4(whole)))
    }

    let floored = math::map4(*point, Float::floor);
    let whole0  = math::map4(*point, math::cast);
    let whole1  = math::add4(whole0, math::one4());
    let frac0   = math::sub4(*point, floored);
    let frac1   = math::sub4(frac0, math::one4());
    let curve   = math::map4(frac0, math::scurve5);

    let f0000 = gradient(seed, [whole0[0], whole0[1], whole0[2], whole0[3]], [frac0[0], frac0[1], frac0[2], frac0[3]]);
    let f1000 = gradient(seed, [whole1[0], whole0[1], whole0[2], whole0[3]], [frac1[0], frac0[1], frac0[2], frac0[3]]);
    let f0001 = gradient(seed, [whole0[0], whole1[1], whole0[2], whole0[3]], [frac0[0], frac1[1], frac0[2], frac0[3]]);
    let f1001 = gradient(seed, [whole1[0], whole1[1], whole0[2], whole0[3]], [frac1[0], frac1[1], frac0[2], frac0[3]]);
    let f0010 = gradient(seed, [whole0[0], whole0[1], whole1[2], whole0[3]], [frac0[0], frac0[1], frac1[2], frac0[3]]);
    let f1010 = gradient(seed, [whole1[0], whole0[1], whole1[2], whole0[3]], [frac1[0], frac0[1], frac1[2], frac0[3]]);
    let f0011 = gradient(seed, [whole0[0], whole1[1], whole1[2], whole0[3]], [frac0[0], frac1[1], frac1[2], frac0[3]]);
    let f1011 = gradient(seed, [whole1[0], whole1[1], whole1[2], whole0[3]], [frac1[0], frac1[1], frac1[2], frac0[3]]);
    let f0100 = gradient(seed, [whole0[0], whole0[1], whole0[2], whole1[3]], [frac0[0], frac0[1], frac0[2], frac1[3]]);
    let f1100 = gradient(seed, [whole1[0], whole0[1], whole0[2], whole1[3]], [frac1[0], frac0[1], frac0[2], frac1[3]]);
    let f0101 = gradient(seed, [whole0[0], whole1[1], whole0[2], whole1[3]], [frac0[0], frac1[1], frac0[2], frac1[3]]);
    let f1101 = gradient(seed, [whole1[0], whole1[1], whole0[2], whole1[3]], [frac1[0], frac1[1], frac0[2], frac1[3]]);
    let f0110 = gradient(seed, [whole0[0], whole0[1], whole1[2], whole1[3]], [frac0[0], frac0[1], frac1[2], frac1[3]]);
    let f1110 = gradient(seed, [whole1[0], whole0[1], whole1[2], whole1[3]], [frac1[0], frac0[1], frac1[2], frac1[3]]);
    let f0111 = gradient(seed, [whole0[0], whole1[1], whole1[2], whole1[3]], [frac0[0], frac1[1], frac1[2], frac1[3]]);
    let f1111 = gradient(seed, [whole1[0], whole1[1], whole1[2], whole1[3]], [frac1[0], frac1[1], frac1[2], frac1[3]]);

    math::quadlerp(curve, f0000, f1000, f0001, f1001, f0010, f1010, f0011, f1011, f0100, f1100, f0101, f1101, f0110, f1110, f0111, f1111)
}
