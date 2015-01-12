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
    fn gradient<T: Float>(seed: math::Point2<usize>, frac: math::Vector2<T>) -> T {
        math::dot2(frac, gradient::get2((seed[0] ^ seed[1]) as usize))
    }

    let one: T = math::cast(1);

    let xfloor = point[0].floor();
    let yfloor = point[1].floor();

    let x0_whole: isize = math::cast(xfloor);
    let y0_whole: isize = math::cast(yfloor);

    let x1_whole = x0_whole + 1;
    let y1_whole = y0_whole + 1;

    let x0_frac = point[0] - xfloor;
    let y0_frac = point[1] - yfloor;

    let x1_frac = x0_frac - one;
    let y1_frac = y0_frac - one;

    let x_curve = math::scurve5(x0_frac);
    let y_curve = math::scurve5(y0_frac);

    let x0_cache = seed.getx(x0_whole);
    let y0_cache = seed.gety(y0_whole);
    let x1_cache = seed.getx(x1_whole);
    let y1_cache = seed.gety(y1_whole);

    let f00 = gradient([x0_cache, y0_cache], [x0_frac, y0_frac]);
    let f10 = gradient([x1_cache, y0_cache], [x1_frac, y0_frac]);
    let f01 = gradient([x0_cache, y1_cache], [x0_frac, y1_frac]);
    let f11 = gradient([x1_cache, y1_cache], [x1_frac, y1_frac]);

    math::bilerp(x_curve, y_curve, f00, f10, f01, f11)
}

/// 3-dimensional perlin noise
pub fn perlin3<T: Float>(seed: &Seed, point: &math::Point3<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: math::Point3<usize>, frac: math::Vector3<T>) -> T {
        math::dot3(frac, gradient::get3(seed[0] ^ seed[1] ^ seed[2]))
    }

    let one: T = math::cast(1);

    let xfloor = point[0].floor();
    let yfloor = point[1].floor();
    let zfloor = point[2].floor();

    let x0_whole: isize = math::cast(xfloor);
    let y0_whole: isize = math::cast(yfloor);
    let z0_whole: isize = math::cast(zfloor);

    let x1_whole = x0_whole + 1;
    let y1_whole = y0_whole + 1;
    let z1_whole = z0_whole + 1;

    let x0_frac = point[0] - xfloor;
    let y0_frac = point[1] - yfloor;
    let z0_frac = point[2] - zfloor;

    let x1_frac = x0_frac - one;
    let y1_frac = y0_frac - one;
    let z1_frac = z0_frac - one;

    let x_curve = math::scurve5(x0_frac);
    let y_curve = math::scurve5(y0_frac);
    let z_curve = math::scurve5(z0_frac);

    let x0_cache = seed.getx(x0_whole);
    let y0_cache = seed.gety(y0_whole);
    let z0_cache = seed.getz(z0_whole);
    let x1_cache = seed.getx(x1_whole);
    let y1_cache = seed.gety(y1_whole);
    let z1_cache = seed.getz(z1_whole);

    let f000 = gradient([x0_cache, y0_cache, z0_cache], [x0_frac, y0_frac, z0_frac]);
    let f100 = gradient([x1_cache, y0_cache, z0_cache], [x1_frac, y0_frac, z0_frac]);
    let f010 = gradient([x0_cache, y1_cache, z0_cache], [x0_frac, y1_frac, z0_frac]);
    let f110 = gradient([x1_cache, y1_cache, z0_cache], [x1_frac, y1_frac, z0_frac]);
    let f001 = gradient([x0_cache, y0_cache, z1_cache], [x0_frac, y0_frac, z1_frac]);
    let f101 = gradient([x1_cache, y0_cache, z1_cache], [x1_frac, y0_frac, z1_frac]);
    let f011 = gradient([x0_cache, y1_cache, z1_cache], [x0_frac, y1_frac, z1_frac]);
    let f111 = gradient([x1_cache, y1_cache, z1_cache], [x1_frac, y1_frac, z1_frac]);

    math::trilerp(x_curve, y_curve, z_curve, f000, f100, f010, f110, f001, f101, f011, f111)
}

/// 4-dimensional perlin noise
pub fn perlin4<T: Float>(seed: &Seed, point: &math::Point4<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: math::Point4<usize>, frac: math::Vector4<T>) -> T {
        math::dot4(frac, gradient::get4(seed[0] ^ seed[1] ^ seed[2] ^ seed[3]))
    }

    let one: T = math::cast(1);

    let xfloor = point[0].floor();
    let yfloor = point[1].floor();
    let zfloor = point[2].floor();
    let wfloor = point[3].floor();

    let x0_whole: isize = math::cast(xfloor);
    let y0_whole: isize = math::cast(yfloor);
    let z0_whole: isize = math::cast(zfloor);
    let w0_whole: isize = math::cast(wfloor);

    let x1_whole = x0_whole + 1;
    let y1_whole = y0_whole + 1;
    let z1_whole = z0_whole + 1;
    let w1_whole = w0_whole + 1;

    let x0_frac = point[0] - xfloor;
    let y0_frac = point[1] - yfloor;
    let z0_frac = point[2] - zfloor;
    let w0_frac = point[2] - wfloor;

    let x1_frac = x0_frac - one;
    let y1_frac = y0_frac - one;
    let z1_frac = z0_frac - one;
    let w1_frac = w0_frac - one;

    let x_curve = math::scurve5(x0_frac);
    let y_curve = math::scurve5(y0_frac);
    let z_curve = math::scurve5(z0_frac);
    let w_curve = math::scurve5(w0_frac);

    let x0_cache = seed.getx(x0_whole);
    let y0_cache = seed.gety(y0_whole);
    let z0_cache = seed.getz(z0_whole);
    let w0_cache = seed.getw(w0_whole);
    let x1_cache = seed.getx(x1_whole);
    let y1_cache = seed.gety(y1_whole);
    let z1_cache = seed.getz(z1_whole);
    let w1_cache = seed.getw(w1_whole);

    let f0000 = gradient([x0_cache, y0_cache, z0_cache, w0_cache], [x0_frac, y0_frac, z0_frac, w0_frac]);
    let f1000 = gradient([x1_cache, y0_cache, z0_cache, w0_cache], [x1_frac, y0_frac, z0_frac, w0_frac]);
    let f0001 = gradient([x0_cache, y1_cache, z0_cache, w0_cache], [x0_frac, y1_frac, z0_frac, w0_frac]);
    let f1001 = gradient([x1_cache, y1_cache, z0_cache, w0_cache], [x1_frac, y1_frac, z0_frac, w0_frac]);
    let f0010 = gradient([x0_cache, y0_cache, z1_cache, w0_cache], [x0_frac, y0_frac, z1_frac, w0_frac]);
    let f1010 = gradient([x1_cache, y0_cache, z1_cache, w0_cache], [x1_frac, y0_frac, z1_frac, w0_frac]);
    let f0011 = gradient([x0_cache, y1_cache, z1_cache, w0_cache], [x0_frac, y1_frac, z1_frac, w0_frac]);
    let f1011 = gradient([x1_cache, y1_cache, z1_cache, w0_cache], [x1_frac, y1_frac, z1_frac, w0_frac]);
    let f0100 = gradient([x0_cache, y0_cache, z0_cache, w1_cache], [x0_frac, y0_frac, z0_frac, w1_frac]);
    let f1100 = gradient([x1_cache, y0_cache, z0_cache, w1_cache], [x1_frac, y0_frac, z0_frac, w1_frac]);
    let f0101 = gradient([x0_cache, y1_cache, z0_cache, w1_cache], [x0_frac, y1_frac, z0_frac, w1_frac]);
    let f1101 = gradient([x1_cache, y1_cache, z0_cache, w1_cache], [x1_frac, y1_frac, z0_frac, w1_frac]);
    let f0110 = gradient([x0_cache, y0_cache, z1_cache, w1_cache], [x0_frac, y0_frac, z1_frac, w1_frac]);
    let f1110 = gradient([x1_cache, y0_cache, z1_cache, w1_cache], [x1_frac, y0_frac, z1_frac, w1_frac]);
    let f0111 = gradient([x0_cache, y1_cache, z1_cache, w1_cache], [x0_frac, y1_frac, z1_frac, w1_frac]);
    let f1111 = gradient([x1_cache, y1_cache, z1_cache, w1_cache], [x1_frac, y1_frac, z1_frac, w1_frac]);

    math::quadlerp(x_curve, y_curve, z_curve, w_curve, f0000, f1000, f0001, f1001, f0010, f1010, f0011, f1011, f0100, f1100, f0101, f1101, f0110, f1110, f0111, f1111)
}
