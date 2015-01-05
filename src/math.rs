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

use std::num::{self, Float, NumCast};

pub fn cast<T: NumCast, U: NumCast>(x: T) -> U {
    num::cast(x).unwrap()
}

/// Linearly interpolates between the values `a` and `b` by the factor `t`
pub fn lerp<T: Float>(t: T, a: T, b: T) -> T {
    t * (b - a) + a
}

pub fn scurve5<T: Float>(t: T) -> T {
    t * t * t * (t * (t * cast(6u) - cast(15u)) + cast(10u))
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
