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

use std::num::{self, Float, NumCast, SignedInt};

pub fn cast<T: NumCast, U: NumCast>(x: T) -> U {
    num::cast(x).unwrap()
}

pub fn lerp<T: Float>(t: T, a: T, b: T) -> T {
    t * (b - a) + a
}

pub fn scurve3<F: Float>(t: F) -> F {
    t * t * (cast::<_, F>(3u) - (t * cast(2u)))
}

pub fn scurve5<T: Float>(t: T) -> T {
    t * t * t * (t * (t * cast(6u) - cast(15u)) + cast(10u))
}

pub fn clamp<F: Float>(val: F, min: F, max: F) -> F {
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _              => val,
    }
}

pub fn pow4<T: Float>(x: T) -> T { x * x * x * x }

pub fn signed_modulus<T: SignedInt>(a: T, b: T) -> T {
    if a.is_negative() { b - (a.abs() % b) } else { a % b }
}

pub type Point2<T> = [T; 2];
pub type Point3<T> = [T; 3];
pub type Point4<T> = [T; 4];

pub type Vector2<T> = [T; 2];
pub type Vector3<T> = [T; 3];
pub type Vector4<T> = [T; 4];

pub fn dot2<T: Float>(a: Vector2<T>, b: Vector2<T>) -> T {
    a[0] * b[0] + a[1] * b[1]
}

pub fn dot3<T: Float>(a: Vector3<T>, b: Vector3<T>) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn dot4<T: Float>(a: Vector4<T>, b: Vector4<T>) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}
