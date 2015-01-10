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

#[allow(unstable)]
pub fn cast<T: NumCast, U: NumCast>(x: T) -> U {
    num::cast(x).unwrap()
}

/// Linearly interpolates between the values `f0` and `f1` by the factor `u`
pub fn lerp<T: Float>(u: T, f0: T, f1: T) -> T {
    u * (f1 - f0) + f0
}

pub fn bilerp<T: Float>(u: T, v: T, f00: T, f10: T, f01: T, f11: T) -> T {
    lerp(v, lerp(u, f00, f10),
            lerp(u, f01, f11))
}

pub fn trilerp<T: Float>(u: T, v: T, w: T, f000: T, f100: T, f010: T, f110: T, f001: T, f101: T, f011: T, f111: T) -> T {
    lerp(w, bilerp(u, v, f000, f100, f010, f110),
            bilerp(u, v, f001, f101, f011, f111))
}

pub fn quadlerp<T: Float>(u: T, v: T, w: T, x: T, f0000: T, f1000: T, f0001: T, f1001: T, f0010: T, f1010: T, f0011: T, f1011: T, f0100: T, f1100: T, f0101: T, f1101: T, f0110: T, f1110: T, f0111: T, f1111: T) -> T {
    lerp(x, trilerp(u, v, w, f0000, f1000, f0001, f1001, f0010, f1010, f0011, f1011),
            trilerp(u, v, w, f0100, f1100, f0101, f1101, f0110, f1110, f0111, f1111))
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

/// The dot product of two 2-dimensional vectors
pub fn dot2<T: Float>(a: Vector2<T>, b: Vector2<T>) -> T {
    a[0] * b[0] + a[1] * b[1]
}

/// The dot product of two 3-dimensional vectors
pub fn dot3<T: Float>(a: Vector3<T>, b: Vector3<T>) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// The dot product of two 4-dimensional vectors
pub fn dot4<T: Float>(a: Vector4<T>, b: Vector4<T>) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}
