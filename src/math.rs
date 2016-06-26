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

//! An ultra-light private math library to make our short lives easier as we
//! implement super-complex noise stuff.

use num_traits::{self, Float, NumCast};
use std::ops::{Add, Mul, Sub};

/// Cast a numeric type without having to unwrap - we don't expect any overflow
/// errors...
pub fn cast<T: NumCast, U: NumCast>(x: T) -> U {
    num_traits::cast(x).unwrap()
}

/// Raises the number to the power of `4`
pub fn pow4<T: Float>(x: T) -> T {
    x * x * x * x
}

/// A 2-dimensional point. This is a fixed sized array, so should be compatible
/// with most linear algebra libraries.
pub type Point2<T> = [T; 2];

/// A 3-dimensional point. This is a fixed sized array, so should be compatible
/// with most linear algebra libraries.
pub type Point3<T> = [T; 3];

/// A 4-dimensional point. This is a fixed sized array, so should be compatible
/// with most linear algebra libraries.
pub type Point4<T> = [T; 4];

/// A 2-dimensional vector, for internal use.
pub type Vector2<T> = [T; 2];
/// A 3-dimensional vector, for internal use.
pub type Vector3<T> = [T; 3];
/// A 4-dimensional vector, for internal use.
pub type Vector4<T> = [T; 4];

pub fn map2<T, U, F>(a: Vector2<T>, f: F) -> Vector2<U>
    where T: Copy,
          F: Fn(T) -> U,
{
    let (ax, ay) = (a[0], a[1]);
    [f(ax), f(ay)]
}
pub fn map3<T, U, F>(a: Vector3<T>, f: F) -> Vector3<U>
    where T: Copy,
          F: Fn(T) -> U,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    [f(ax), f(ay), f(az)]
}
pub fn map4<T, U, F>(a: Vector4<T>, f: F) -> Vector4<U>
    where T: Copy,
          F: Fn(T) -> U,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    [f(ax), f(ay), f(az), f(aw)]
}

pub fn zip_with2<T, U, V, F>(a: Vector2<T>, b: Vector2<U>, f: F) -> Vector2<V>
    where T: Copy,
          U: Copy,
          F: Fn(T, U) -> V,
{
    let (ax, ay) = (a[0], a[1]);
    let (bx, by) = (b[0], b[1]);
    [f(ax, bx), f(ay, by)]
}
pub fn zip_with3<T, U, V, F>(a: Vector3<T>, b: Vector3<U>, f: F) -> Vector3<V>
    where T: Copy,
          U: Copy,
          F: Fn(T, U) -> V,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    let (bx, by, bz) = (b[0], b[1], b[2]);
    [f(ax, bx), f(ay, by), f(az, bz)]
}
pub fn zip_with4<T, U, V, F>(a: Vector4<T>, b: Vector4<U>, f: F) -> Vector4<V>
    where T: Copy,
          U: Copy,
          F: Fn(T, U) -> V,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    let (bx, by, bz, bw) = (b[0], b[1], b[2], b[3]);
    [f(ax, bx), f(ay, by), f(az, bz), f(aw, bw)]
}

pub fn fold2<T, F>(a: Vector2<T>, f: F) -> T
    where T: Copy,
          F: Fn(T, T) -> T,
{
    let (ax, ay) = (a[0], a[1]);
    f(ax, ay)
}
pub fn fold3<T, F>(a: Vector3<T>, f: F) -> T
    where T: Copy,
          F: Fn(T, T) -> T,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    f(f(ax, ay), az)
}
pub fn fold4<T, F>(a: Vector4<T>, f: F) -> T
    where T: Copy,
          F: Fn(T, T) -> T,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    f(f(f(ax, ay), az), aw)
}

pub fn add2<T>(a: Point2<T>, b: Vector2<T>) -> Point2<T>
    where T: Copy + Add<T, Output = T>,
{
    zip_with2(a, b, Add::add)
}
pub fn add3<T>(a: Point3<T>, b: Vector3<T>) -> Point3<T>
    where T: Copy + Add<T, Output = T>,
{
    zip_with3(a, b, Add::add)
}
pub fn add4<T>(a: Point4<T>, b: Vector4<T>) -> Point4<T>
    where T: Copy + Add<T, Output = T>,
{
    zip_with4(a, b, Add::add)
}

pub fn sub2<T>(a: Point2<T>, b: Point2<T>) -> Vector2<T>
    where T: Copy + Sub<T, Output = T>,
{
    zip_with2(a, b, Sub::sub)
}
pub fn sub3<T>(a: Point3<T>, b: Point3<T>) -> Vector3<T>
    where T: Copy + Sub<T, Output = T>,
{
    zip_with3(a, b, Sub::sub)
}
pub fn sub4<T>(a: Point4<T>, b: Point4<T>) -> Vector4<T>
    where T: Copy + Sub<T, Output = T>,
{
    zip_with4(a, b, Sub::sub)
}

pub fn mul2<T>(a: Vector2<T>, b: T) -> Vector2<T>
    where T: Copy + Mul<T, Output = T>,
{
    zip_with2(a, const2(b), Mul::mul)
}
pub fn mul3<T>(a: Vector3<T>, b: T) -> Vector3<T>
    where T: Copy + Mul<T, Output = T>,
{
    zip_with3(a, const3(b), Mul::mul)
}
pub fn mul4<T>(a: Vector4<T>, b: T) -> Vector4<T>
    where T: Copy + Mul<T, Output = T>,
{
    zip_with4(a, const4(b), Mul::mul)
}

pub fn dot2<T: Float>(a: Vector2<T>, b: Vector2<T>) -> T {
    fold2(zip_with2(a, b, Mul::mul), Add::add)
}
pub fn dot3<T: Float>(a: Vector3<T>, b: Vector3<T>) -> T {
    fold3(zip_with3(a, b, Mul::mul), Add::add)
}
pub fn dot4<T: Float>(a: Vector4<T>, b: Vector4<T>) -> T {
    fold4(zip_with4(a, b, Mul::mul), Add::add)
}

pub fn const2<T: Copy>(x: T) -> Vector2<T> {
    [x, x]
}
pub fn const3<T: Copy>(x: T) -> Vector3<T> {
    [x, x, x]
}
pub fn const4<T: Copy>(x: T) -> Vector4<T> {
    [x, x, x, x]
}

pub fn one2<T: Copy + NumCast>() -> Vector2<T> {
    cast2(const2(1))
}
pub fn one3<T: Copy + NumCast>() -> Vector3<T> {
    cast3(const3(1))
}
pub fn one4<T: Copy + NumCast>() -> Vector4<T> {
    cast4(const4(1))
}

pub fn cast2<T, U>(x: Point2<T>) -> Point2<U>
    where T: NumCast + Copy,
          U: NumCast + Copy,
{
    map2(x, cast)
}
pub fn cast3<T, U>(x: Point3<T>) -> Point3<U>
    where T: NumCast + Copy,
          U: NumCast + Copy,
{
    map3(x, cast)
}
pub fn cast4<T, U>(x: Point4<T>) -> Point4<U>
    where T: NumCast + Copy,
          U: NumCast + Copy,
{
    map4(x, cast)
}
