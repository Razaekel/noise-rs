// Copyright 2013 The noise-rs developers. For a full listing of the authors,
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

use {math, Seed};
use gradients::{GRADIENT2, GRADIENT3, GRADIENT4};

fn perlin2<T, F>(seed: &Seed, point: &::Point2<T>, scurve: F) -> T where
    T: Float,
    F: Fn(T) -> T,
{
    fn gradient<T: Float>(seed: &Seed, x_whole: int, y_whole: int, x_frac: T, y_frac: T) -> T {
        let [x, y] = GRADIENT2[seed.get2(x_whole, y_whole) % GRADIENT2.len()];
        x_frac * math::cast(x) + y_frac * math::cast(y)
    }

    let xfloor = point[0].floor();
    let yfloor = point[1].floor();

    let x0_whole = xfloor.to_int().unwrap();
    let y0_whole = yfloor.to_int().unwrap();

    let x1_whole = x0_whole + 1;
    let y1_whole = y0_whole + 1;

    let x0_frac = point[0] - xfloor;
    let y0_frac = point[1] - yfloor;

    let x1_frac = x0_frac - Float::one();
    let y1_frac = y0_frac - Float::one();

    let x_curve = scurve(x0_frac);
    let y_curve = scurve(y0_frac);

    let n0 = gradient(seed, x0_whole, y0_whole, x0_frac, y0_frac);
    let n1 = gradient(seed, x1_whole, y0_whole, x1_frac, y0_frac);
    let interpolated_x0 = math::lerp(x_curve, n0, n1);

    let n0 = gradient(seed, x0_whole, y1_whole, x0_frac, y1_frac);
    let n1 = gradient(seed, x1_whole, y1_whole, x1_frac, y1_frac);
    let interpolated_x1 = math::lerp(x_curve, n0, n1);

    math::lerp(y_curve, interpolated_x0, interpolated_x1)
}

pub fn perlin2_fast<T: Float>(seed: &Seed, point: &::Point2<T>) -> T {
    perlin2(seed, point, math::scurve3)
}

pub fn perlin2_best<T: Float>(seed: &Seed, point: &::Point2<T>) -> T {
    perlin2(seed, point, math::scurve5)
}

fn perlin3<T, F>(seed: &Seed, point: &::Point3<T>, scurve: F) -> T where
    T: Float,
    F: Fn(T) -> T,
{
    fn gradient<T: Float>(seed: &Seed, x_whole: int, y_whole: int, z_whole: int, x_frac: T, y_frac: T, z_frac: T) -> T {
        let [x, y, z] = GRADIENT3[seed.get3(x_whole, y_whole, z_whole) % GRADIENT3.len()];
        x_frac * math::cast(x) + y_frac * math::cast(y) + z_frac * math::cast(z)
    }

    let xfloor = point[0].floor();
    let yfloor = point[1].floor();
    let zfloor = point[2].floor();

    let x0_whole = xfloor.to_int().unwrap();
    let y0_whole = yfloor.to_int().unwrap();
    let z0_whole = zfloor.to_int().unwrap();

    let x1_whole = x0_whole + 1;
    let y1_whole = y0_whole + 1;
    let z1_whole = z0_whole + 1;

    let x0_frac = point[0] - xfloor;
    let y0_frac = point[1] - yfloor;
    let z0_frac = point[2] - zfloor;

    let x1_frac = x0_frac - Float::one();
    let y1_frac = y0_frac - Float::one();
    let z1_frac = z0_frac - Float::one();

    let x_curve = scurve(x0_frac);
    let y_curve = scurve(y0_frac);
    let z_curve = scurve(z0_frac);

    let n0 = gradient(seed, x0_whole, y0_whole, z0_whole, x0_frac, y0_frac, z0_frac);
    let n1 = gradient(seed, x1_whole, y0_whole, z0_whole, x1_frac, y0_frac, z0_frac);
    let interpolated_x0 = math::lerp(x_curve, n0, n1);

    let n0 = gradient(seed, x0_whole, y1_whole, z0_whole, x0_frac, y1_frac, z0_frac);
    let n1 = gradient(seed, x1_whole, y1_whole, z0_whole, x1_frac, y1_frac, z0_frac);
    let interpolated_x1 = math::lerp(x_curve, n0, n1);
    let interpolated_y0 = math::lerp(y_curve, interpolated_x0, interpolated_x1);

    let n0 = gradient(seed, x0_whole, y0_whole, z1_whole, x0_frac, y0_frac, z1_frac);
    let n1 = gradient(seed, x1_whole, y0_whole, z1_whole, x1_frac, y0_frac, z1_frac);
    let interpolated_x0 = math::lerp(x_curve, n0, n1);

    let n0 = gradient(seed, x0_whole, y1_whole, z1_whole, x0_frac, y1_frac, z1_frac);
    let n1 = gradient(seed, x1_whole, y1_whole, z1_whole, x1_frac, y1_frac, z1_frac);
    let interpolated_x1 = math::lerp(x_curve, n0, n1);
    let interpolated_y1 = math::lerp(y_curve, interpolated_x0, interpolated_x1);

    math::lerp(z_curve, interpolated_y0, interpolated_y1)
}

pub fn perlin3_fast<T: Float>(seed: &Seed, point: &::Point3<T>) -> T {
    perlin3(seed, point, math::scurve3)
}

pub fn perlin3_best<T: Float>(seed: &Seed, point: &::Point3<T>) -> T {
    perlin3(seed, point, math::scurve5)
}

fn perlin4<T, F>(seed: &Seed, point: &::Point4<T>, scurve: F) -> T where
    T: Float,
    F: Fn(T) -> T,
{
    fn gradient<T: Float>(seed: &Seed, x_whole: int, y_whole: int, z_whole: int, w_whole: int, x_frac: T, y_frac: T, z_frac: T, w_frac: T) -> T {
        let [x, y, z, w] = GRADIENT4[seed.get4(x_whole, y_whole, z_whole, w_whole) % GRADIENT4.len()];
        x_frac * math::cast(x) + y_frac * math::cast(y) + z_frac * math::cast(z) + w_frac * math::cast(w)
    }

    let xfloor = point[0].floor();
    let yfloor = point[1].floor();
    let zfloor = point[2].floor();
    let wfloor = point[3].floor();

    let x0_whole = xfloor.to_int().unwrap();
    let y0_whole = yfloor.to_int().unwrap();
    let z0_whole = zfloor.to_int().unwrap();
    let w0_whole = wfloor.to_int().unwrap();

    let x1_whole = x0_whole + 1;
    let y1_whole = y0_whole + 1;
    let z1_whole = z0_whole + 1;
    let w1_whole = w0_whole + 1;

    let x0_frac = point[0] - xfloor;
    let y0_frac = point[1] - yfloor;
    let z0_frac = point[2] - zfloor;
    let w0_frac = point[2] - wfloor;

    let x1_frac = x0_frac - Float::one();
    let y1_frac = y0_frac - Float::one();
    let z1_frac = z0_frac - Float::one();
    let w1_frac = w0_frac - Float::one();

    let x_curve = scurve(x0_frac);
    let y_curve = scurve(y0_frac);
    let z_curve = scurve(z0_frac);
    let w_curve = scurve(w0_frac);

    let n0 = gradient(seed, x0_whole, y0_whole, z0_whole, w0_whole, x0_frac, y0_frac, z0_frac, w0_frac);
    let n1 = gradient(seed, x1_whole, y0_whole, z0_whole, w0_whole, x1_frac, y0_frac, z0_frac, w0_frac);
    let interpolated_x0 = math::lerp(x_curve, n0, n1);

    let n0 = gradient(seed, x0_whole, y1_whole, z0_whole, w0_whole, x0_frac, y1_frac, z0_frac, w0_frac);
    let n1 = gradient(seed, x1_whole, y1_whole, z0_whole, w0_whole, x1_frac, y1_frac, z0_frac, w0_frac);
    let interpolated_x1 = math::lerp(x_curve, n0, n1);
    let interpolated_y0 = math::lerp(y_curve, interpolated_x0, interpolated_x1);

    let n0 = gradient(seed, x0_whole, y0_whole, z1_whole, w0_whole, x0_frac, y0_frac, z1_frac, w0_frac);
    let n1 = gradient(seed, x1_whole, y0_whole, z1_whole, w0_whole, x1_frac, y0_frac, z1_frac, w0_frac);
    let interpolated_x0 = math::lerp(x_curve, n0, n1);

    let n0 = gradient(seed, x0_whole, y1_whole, z1_whole, w0_whole, x0_frac, y1_frac, z1_frac, w0_frac);
    let n1 = gradient(seed, x1_whole, y1_whole, z1_whole, w0_whole, x1_frac, y1_frac, z1_frac, w0_frac);
    let interpolated_x1 = math::lerp(x_curve, n0, n1);
    let interpolated_y1 = math::lerp(y_curve, interpolated_x0, interpolated_x1);
    let interpolated_z0 = math::lerp(z_curve, interpolated_y0, interpolated_y1);

    let n0 = gradient(seed, x0_whole, y0_whole, z0_whole, w1_whole, x0_frac, y0_frac, z0_frac, w1_frac);
    let n1 = gradient(seed, x1_whole, y0_whole, z0_whole, w1_whole, x1_frac, y0_frac, z0_frac, w1_frac);
    let interpolated_x0 = math::lerp(x_curve, n0, n1);

    let n0 = gradient(seed, x0_whole, y1_whole, z0_whole, w1_whole, x0_frac, y1_frac, z0_frac, w1_frac);
    let n1 = gradient(seed, x1_whole, y1_whole, z0_whole, w1_whole, x1_frac, y1_frac, z0_frac, w1_frac);
    let interpolated_x1 = math::lerp(x_curve, n0, n1);
    let interpolated_y0 = math::lerp(y_curve, interpolated_x0, interpolated_x1);

    let n0 = gradient(seed, x0_whole, y0_whole, z1_whole, w1_whole, x0_frac, y0_frac, z1_frac, w1_frac);
    let n1 = gradient(seed, x1_whole, y0_whole, z1_whole, w1_whole, x1_frac, y0_frac, z1_frac, w1_frac);
    let interpolated_x0 = math::lerp(x_curve, n0, n1);

    let n0 = gradient(seed, x0_whole, y1_whole, z1_whole, w1_whole, x0_frac, y1_frac, z1_frac, w1_frac);
    let n1 = gradient(seed, x1_whole, y1_whole, z1_whole, w1_whole, x1_frac, y1_frac, z1_frac, w1_frac);
    let interpolated_x1 = math::lerp(x_curve, n0, n1);
    let interpolated_y1 = math::lerp(y_curve, interpolated_x0, interpolated_x1);
    let interpolated_z1 = math::lerp(z_curve, interpolated_y0, interpolated_y1);

    math::lerp(w_curve, interpolated_z0, interpolated_z1)
}

pub fn perlin4_fast<T: Float>(seed: &Seed, point: &::Point4<T>) -> T {
    perlin4(seed, point, math::scurve3)
}

pub fn perlin4_best<T: Float>(seed: &Seed, point: &::Point4<T>) -> T {
    perlin4(seed, point, math::scurve5)
}
