// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

//! An ultra-light private math library to make our short lives easier as we
//! implement super-complex noise stuff.

use num_traits::{self, Float, NumCast};
use std::ops::{Add, Mul, Sub};

/// Cast a numeric type without having to unwrap - we don't expect any overflow
/// errors...
#[inline]
pub fn cast<T: NumCast, U: NumCast>(x: T) -> U {
    num_traits::cast(x).unwrap()
}

/// Raises the number to the power of `4`
#[inline]
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

#[inline]
pub fn map2<T, U, F>(a: Vector2<T>, f: F) -> Vector2<U>
    where T: Copy,
          F: Fn(T) -> U,
{
    let (ax, ay) = (a[0], a[1]);
    [f(ax), f(ay)]
}

#[inline]
pub fn map3<T, U, F>(a: Vector3<T>, f: F) -> Vector3<U>
    where T: Copy,
          F: Fn(T) -> U,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    [f(ax), f(ay), f(az)]
}

#[inline]
pub fn map4<T, U, F>(a: Vector4<T>, f: F) -> Vector4<U>
    where T: Copy,
          F: Fn(T) -> U,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    [f(ax), f(ay), f(az), f(aw)]
}

#[inline]
pub fn zip_with2<T, U, V, F>(a: Vector2<T>, b: Vector2<U>, f: F) -> Vector2<V>
    where T: Copy,
          U: Copy,
          F: Fn(T, U) -> V,
{
    let (ax, ay) = (a[0], a[1]);
    let (bx, by) = (b[0], b[1]);
    [f(ax, bx), f(ay, by)]
}

#[inline]
pub fn zip_with3<T, U, V, F>(a: Vector3<T>, b: Vector3<U>, f: F) -> Vector3<V>
    where T: Copy,
          U: Copy,
          F: Fn(T, U) -> V,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    let (bx, by, bz) = (b[0], b[1], b[2]);
    [f(ax, bx), f(ay, by), f(az, bz)]
}

#[inline]
pub fn zip_with4<T, U, V, F>(a: Vector4<T>, b: Vector4<U>, f: F) -> Vector4<V>
    where T: Copy,
          U: Copy,
          F: Fn(T, U) -> V,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    let (bx, by, bz, bw) = (b[0], b[1], b[2], b[3]);
    [f(ax, bx), f(ay, by), f(az, bz), f(aw, bw)]
}

#[inline]
pub fn fold2<T, F>(a: Vector2<T>, f: F) -> T
    where T: Copy,
          F: Fn(T, T) -> T,
{
    let (ax, ay) = (a[0], a[1]);
    f(ax, ay)
}

#[inline]
pub fn fold3<T, F>(a: Vector3<T>, f: F) -> T
    where T: Copy,
          F: Fn(T, T) -> T,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    f(f(ax, ay), az)
}

#[inline]
pub fn fold4<T, F>(a: Vector4<T>, f: F) -> T
    where T: Copy,
          F: Fn(T, T) -> T,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    f(f(f(ax, ay), az), aw)
}

#[inline]
pub fn add2<T>(a: Point2<T>, b: Vector2<T>) -> Point2<T>
    where T: Copy + Add<T, Output = T>,
{
    zip_with2(a, b, Add::add)
}

#[inline]
pub fn add3<T>(a: Point3<T>, b: Vector3<T>) -> Point3<T>
    where T: Copy + Add<T, Output = T>,
{
    zip_with3(a, b, Add::add)
}

#[inline]
pub fn add4<T>(a: Point4<T>, b: Vector4<T>) -> Point4<T>
    where T: Copy + Add<T, Output = T>,
{
    zip_with4(a, b, Add::add)
}

#[inline]
pub fn sub2<T>(a: Point2<T>, b: Point2<T>) -> Vector2<T>
    where T: Copy + Sub<T, Output = T>,
{
    zip_with2(a, b, Sub::sub)
}

#[inline]
pub fn sub3<T>(a: Point3<T>, b: Point3<T>) -> Vector3<T>
    where T: Copy + Sub<T, Output = T>,
{
    zip_with3(a, b, Sub::sub)
}

#[inline]
pub fn sub4<T>(a: Point4<T>, b: Point4<T>) -> Vector4<T>
    where T: Copy + Sub<T, Output = T>,
{
    zip_with4(a, b, Sub::sub)
}

#[inline]
pub fn mul2<T>(a: Vector2<T>, b: T) -> Vector2<T>
    where T: Copy + Mul<T, Output = T>,
{
    zip_with2(a, const2(b), Mul::mul)
}

#[inline]
pub fn mul3<T>(a: Vector3<T>, b: T) -> Vector3<T>
    where T: Copy + Mul<T, Output = T>,
{
    zip_with3(a, const3(b), Mul::mul)
}

#[inline]
pub fn mul4<T>(a: Vector4<T>, b: T) -> Vector4<T>
    where T: Copy + Mul<T, Output = T>,
{
    zip_with4(a, const4(b), Mul::mul)
}

#[inline]
pub fn dot2<T: Float>(a: Vector2<T>, b: Vector2<T>) -> T {
    fold2(zip_with2(a, b, Mul::mul), Add::add)
}

#[inline]
pub fn dot3<T: Float>(a: Vector3<T>, b: Vector3<T>) -> T {
    fold3(zip_with3(a, b, Mul::mul), Add::add)
}

#[inline]
pub fn dot4<T: Float>(a: Vector4<T>, b: Vector4<T>) -> T {
    fold4(zip_with4(a, b, Mul::mul), Add::add)
}

#[inline]
pub fn const2<T: Copy>(x: T) -> Vector2<T> {
    [x, x]
}

#[inline]
pub fn const3<T: Copy>(x: T) -> Vector3<T> {
    [x, x, x]
}

#[inline]
pub fn const4<T: Copy>(x: T) -> Vector4<T> {
    [x, x, x, x]
}

#[inline]
pub fn one2<T: Copy + NumCast>() -> Vector2<T> {
    cast2(const2(1))
}

#[inline]
pub fn one3<T: Copy + NumCast>() -> Vector3<T> {
    cast3(const3(1))
}

#[inline]
pub fn one4<T: Copy + NumCast>() -> Vector4<T> {
    cast4(const4(1))
}

#[inline]
pub fn cast2<T, U>(x: Point2<T>) -> Point2<U>
    where T: NumCast + Copy,
          U: NumCast + Copy,
{
    map2(x, cast)
}

#[inline]
pub fn cast3<T, U>(x: Point3<T>) -> Point3<U>
    where T: NumCast + Copy,
          U: NumCast + Copy,
{
    map3(x, cast)
}

#[inline]
pub fn cast4<T, U>(x: Point4<T>) -> Point4<U>
    where T: NumCast + Copy,
          U: NumCast + Copy,
{
    map4(x, cast)
}

pub mod interp {
    use math;
    use num_traits::Float;

    /// Performs linear interploation between two values.
    #[inline]
    pub fn linear<T: Float>(a: T, b: T, x: T) -> T {
        x.mul_add((b - a), a)
    }

    /// Performs cubic interpolation between two values bound between two other
    /// values.
    ///
    /// - n0 - The value before the first value.
    /// - n1 - The first value.
    /// - n2 - The second value.
    /// - n3 - The value after the second value.
    /// - x - The alpha value.
    ///
    /// The alpha value should range from 0.0 to 1.0. If the alpha value is
    /// 0.0, this function returns _n1_. If the alpha value is 1.0, this
    /// function returns _n2_.
    #[inline]
    pub fn cubic<T: Float>(n0: T, n1: T, n2: T, n3: T, x: T) -> T {
        let p = (n3 - n2) - (n0 - n1);
        let q = (n0 - n1) - p;
        let r = n2 - n0;
        let s = n1;
        p * x * x * x + q * x * x + r * x + s
    }

    /// Maps a value onto a cubic S-curve.
    #[inline]
    pub fn s_curve3<T: Float>(x: T) -> T {
        x * x * (math::cast::<_, T>(3.0) - (x * math::cast(2.0)))
    }

    /// Maps a value onto a quintic S-curve.
    #[inline]
    pub fn s_curve5<T: Float>(x: T) -> T {
        x * x * x * (x * (x * math::cast(6.0) - math::cast(15.0) + math::cast(10.0)))
    }
}
