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

use std::num::{self, Float, NumCast};
use std::ops::{Add, Sub, Mul};

pub fn cast<T: NumCast, U: NumCast>(x: T) -> U {
    num::cast(x).unwrap()
}

/// Linearly interpolates between the values `f0` and `f1` by the factor `u`
pub fn lerp<T: Float>(u: T, f0: T, f1: T) -> T {
    u * (f1 - f0) + f0
}

pub fn bilerp<T: Float>([u, v]: Vector2<T>, f00: T, f10: T, f01: T, f11: T) -> T {
    lerp(v, lerp(u, f00, f10),
            lerp(u, f01, f11))
}

pub fn trilerp<T: Float>([u, v, w]: Vector3<T>, f000: T, f100: T, f010: T, f110: T, f001: T, f101: T, f011: T, f111: T) -> T {
    lerp(w, bilerp([u, v], f000, f100, f010, f110),
            bilerp([u, v], f001, f101, f011, f111))
}

pub fn quadlerp<T: Float>([u, v, w, x]: Vector4<T>, f0000: T, f1000: T, f0001: T, f1001: T, f0010: T, f1010: T, f0011: T, f1011: T, f0100: T, f1100: T, f0101: T, f1101: T, f0110: T, f1110: T, f0111: T, f1111: T) -> T {
    lerp(x, trilerp([u, v, w], f0000, f1000, f0001, f1001, f0010, f1010, f0011, f1011),
            trilerp([u, v, w], f0100, f1100, f0101, f1101, f0110, f1110, f0111, f1111))
}

pub fn scurve5<T: Float>(t: T) -> T {
    t * t * t * (t * (t * cast(6) - cast(15)) + cast(10))
}

/// Raises the number to the power of `4`
pub fn pow4<T: Float>(x: T) -> T { x * x * x * x }

/// A 2-dimensional point
pub type Point2<T> = [T; 2];
/// A 3-dimensional point
pub type Point3<T> = [T; 3];
/// A 4-dimensional point
pub type Point4<T> = [T; 4];

/// A 2-dimensional vector
pub type Vector2<T> = [T; 2];
/// A 3-dimensional vector
pub type Vector3<T> = [T; 3];
/// A 4-dimensional vector
pub type Vector4<T> = [T; 4];

pub fn map2<T, U, F: Fn(T) -> U>([ax, ay        ]: Vector2<T>, f: F) -> Vector2<U> { [f(ax), f(ay)] }
pub fn map3<T, U, F: Fn(T) -> U>([ax, ay, az    ]: Vector3<T>, f: F) -> Vector3<U> { [f(ax), f(ay), f(az)] }
pub fn map4<T, U, F: Fn(T) -> U>([ax, ay, az, aw]: Vector4<T>, f: F) -> Vector4<U> { [f(ax), f(ay), f(az), f(aw)] }

pub fn zip_with2<T, U, V, F: Fn(T, U) -> V>([ax, ay        ]: Vector2<T>, [bx, by        ]: Vector2<U>, f: F) -> Vector2<V> { [f(ax, bx), f(ay, by)] }
pub fn zip_with3<T, U, V, F: Fn(T, U) -> V>([ax, ay, az    ]: Vector3<T>, [bx, by, bz    ]: Vector3<U>, f: F) -> Vector3<V> { [f(ax, bx), f(ay, by), f(az, bz)] }
pub fn zip_with4<T, U, V, F: Fn(T, U) -> V>([ax, ay, az, aw]: Vector4<T>, [bx, by, bz, bw]: Vector4<U>, f: F) -> Vector4<V> { [f(ax, bx), f(ay, by), f(az, bz), f(aw, bw)] }

pub fn fold2<T, F: Fn(T, T) -> T>([ax, ay        ]: Vector2<T>, f: F) -> T { f(ax, ay) }
pub fn fold3<T, F: Fn(T, T) -> T>([ax, ay, az    ]: Vector3<T>, f: F) -> T { f(f(ax, ay), az) }
pub fn fold4<T, F: Fn(T, T) -> T>([ax, ay, az, aw]: Vector4<T>, f: F) -> T { f(f(f(ax, ay), az), aw) }

pub fn add2<T: Add<T, Output = T>>(a: Point2<T>, b: Vector2<T>) -> Point2<T> { zip_with2(a, b, Add::add) }
pub fn add3<T: Add<T, Output = T>>(a: Point3<T>, b: Vector3<T>) -> Point3<T> { zip_with3(a, b, Add::add) }
pub fn add4<T: Add<T, Output = T>>(a: Point4<T>, b: Vector4<T>) -> Point4<T> { zip_with4(a, b, Add::add) }

pub fn sub2<T: Sub<T, Output = T>>(a: Point2<T>, b: Point2<T>) -> Vector2<T> { zip_with2(a, b, Sub::sub) }
pub fn sub3<T: Sub<T, Output = T>>(a: Point3<T>, b: Point3<T>) -> Vector3<T> { zip_with3(a, b, Sub::sub) }
pub fn sub4<T: Sub<T, Output = T>>(a: Point4<T>, b: Point4<T>) -> Vector4<T> { zip_with4(a, b, Sub::sub) }

pub fn mul2<T: Mul<T, Output = T> + Copy>(a: Vector2<T>, b: T) -> Vector2<T> { zip_with2(a, const2(b), Mul::mul) }
pub fn mul3<T: Mul<T, Output = T> + Copy>(a: Vector3<T>, b: T) -> Vector3<T> { zip_with3(a, const3(b), Mul::mul) }
pub fn mul4<T: Mul<T, Output = T> + Copy>(a: Vector4<T>, b: T) -> Vector4<T> { zip_with4(a, const4(b), Mul::mul) }

pub fn dot2<T: Float>(a: Vector2<T>, b: Vector2<T>) -> T { fold2(zip_with2(a, b, Mul::mul), Add::add) }
pub fn dot3<T: Float>(a: Vector3<T>, b: Vector3<T>) -> T { fold3(zip_with3(a, b, Mul::mul), Add::add) }
pub fn dot4<T: Float>(a: Vector4<T>, b: Vector4<T>) -> T { fold4(zip_with4(a, b, Mul::mul), Add::add) }

pub fn const2<T: Copy>(x: T) -> Vector2<T> { [x, x] }
pub fn const3<T: Copy>(x: T) -> Vector3<T> { [x, x, x] }
pub fn const4<T: Copy>(x: T) -> Vector4<T> { [x, x, x, x] }

pub fn one2<T: Copy + NumCast>() -> Vector2<T> { cast2(const2(1)) }
pub fn one3<T: Copy + NumCast>() -> Vector3<T> { cast3(const3(1)) }
pub fn one4<T: Copy + NumCast>() -> Vector4<T> { cast4(const4(1)) }

pub fn cast2<T: NumCast + Copy, U: NumCast + Copy>(x: Point2<T>) -> Point2<U> { map2(x, cast) }
pub fn cast3<T: NumCast + Copy, U: NumCast + Copy>(x: Point3<T>) -> Point3<U> { map3(x, cast) }
pub fn cast4<T: NumCast + Copy, U: NumCast + Copy>(x: Point4<T>) -> Point4<U> { map4(x, cast) }
