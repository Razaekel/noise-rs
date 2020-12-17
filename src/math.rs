//! An ultra-light private math library to make our short lives easier as we
//! implement super-complex noise stuff.

use std::ops::{Add, Mul, Sub};

/// Cast a numeric type without having to unwrap - we don't expect any overflow
/// errors...
#[inline]
pub(crate) fn cast<T, U: From<T>>(x: T) -> U {
    From::from(x)
}

#[inline]
pub(crate) fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    assert!(max >= min);
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    }
}

#[inline]
pub(crate) fn map2<T, U, F>(a: [T; 2], f: F) -> [U; 2]
where
    T: Copy,
    F: Fn(T) -> U,
{
    let (ax, ay) = (a[0], a[1]);
    [f(ax), f(ay)]
}

#[inline]
pub(crate) fn map3<T, U, F>(a: [T; 3], f: F) -> [U; 3]
where
    T: Copy,
    F: Fn(T) -> U,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    [f(ax), f(ay), f(az)]
}

#[inline]
pub(crate) fn map4<T, U, F>(a: [T; 4], f: F) -> [U; 4]
where
    T: Copy,
    F: Fn(T) -> U,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    [f(ax), f(ay), f(az), f(aw)]
}

#[inline]
pub(crate) fn zip_with2<T, U, V, F>(a: [T; 2], b: [U; 2], f: F) -> [V; 2]
where
    T: Copy,
    U: Copy,
    F: Fn(T, U) -> V,
{
    let (ax, ay) = (a[0], a[1]);
    let (bx, by) = (b[0], b[1]);
    [f(ax, bx), f(ay, by)]
}

#[inline]
pub(crate) fn zip_with3<T, U, V, F>(a: [T; 3], b: [U; 3], f: F) -> [V; 3]
where
    T: Copy,
    U: Copy,
    F: Fn(T, U) -> V,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    let (bx, by, bz) = (b[0], b[1], b[2]);
    [f(ax, bx), f(ay, by), f(az, bz)]
}

#[inline]
pub(crate) fn zip_with4<T, U, V, F>(a: [T; 4], b: [U; 4], f: F) -> [V; 4]
where
    T: Copy,
    U: Copy,
    F: Fn(T, U) -> V,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    let (bx, by, bz, bw) = (b[0], b[1], b[2], b[3]);
    [f(ax, bx), f(ay, by), f(az, bz), f(aw, bw)]
}

#[inline]
pub(crate) fn fold2<T, F>(a: [T; 2], f: F) -> T
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    let (ax, ay) = (a[0], a[1]);
    f(ax, ay)
}

#[inline]
pub(crate) fn fold3<T, F>(a: [T; 3], f: F) -> T
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    f(f(ax, ay), az)
}

#[inline]
pub(crate) fn fold4<T, F>(a: [T; 4], f: F) -> T
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    f(f(f(ax, ay), az), aw)
}

#[inline]
pub(crate) fn add2<T>(a: [T; 2], b: [T; 2]) -> [T; 2]
where
    T: Copy + Add<T, Output = T>,
{
    zip_with2(a, b, Add::add)
}

#[inline]
pub(crate) fn add3<T>(a: [T; 3], b: [T; 3]) -> [T; 3]
where
    T: Copy + Add<T, Output = T>,
{
    zip_with3(a, b, Add::add)
}

#[inline]
pub(crate) fn add4<T>(a: [T; 4], b: [T; 4]) -> [T; 4]
where
    T: Copy + Add<T, Output = T>,
{
    zip_with4(a, b, Add::add)
}

#[inline]
pub(crate) fn sub2<T>(a: [T; 2], b: [T; 2]) -> [T; 2]
where
    T: Copy + Sub<T, Output = T>,
{
    zip_with2(a, b, Sub::sub)
}

#[inline]
pub(crate) fn sub3<T>(a: [T; 3], b: [T; 3]) -> [T; 3]
where
    T: Copy + Sub<T, Output = T>,
{
    zip_with3(a, b, Sub::sub)
}

#[inline]
pub(crate) fn sub4<T>(a: [T; 4], b: [T; 4]) -> [T; 4]
where
    T: Copy + Sub<T, Output = T>,
{
    zip_with4(a, b, Sub::sub)
}

#[inline]
pub(crate) fn mul2<T>(a: [T; 2], b: T) -> [T; 2]
where
    T: Copy + Mul<T, Output = T>,
{
    zip_with2(a, const2(b), Mul::mul)
}

#[inline]
pub(crate) fn mul3<T>(a: [T; 3], b: T) -> [T; 3]
where
    T: Copy + Mul<T, Output = T>,
{
    zip_with3(a, const3(b), Mul::mul)
}

#[inline]
pub(crate) fn mul4<T>(a: [T; 4], b: T) -> [T; 4]
where
    T: Copy + Mul<T, Output = T>,
{
    zip_with4(a, const4(b), Mul::mul)
}

#[inline]
pub(crate) fn dot2(a: [f64; 2], b: [f64; 2]) -> f64 {
    fold2(zip_with2(a, b, Mul::mul), Add::add)
}

#[inline]
pub(crate) fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    fold3(zip_with3(a, b, Mul::mul), Add::add)
}

#[inline]
pub(crate) fn dot4(a: [f64; 4], b: [f64; 4]) -> f64 {
    fold4(zip_with4(a, b, Mul::mul), Add::add)
}

#[inline]
pub(crate) fn const2<T: Copy>(x: T) -> [T; 2] {
    [x, x]
}

#[inline]
pub(crate) fn const3<T: Copy>(x: T) -> [T; 3] {
    [x, x, x]
}

#[inline]
pub(crate) fn const4<T: Copy>(x: T) -> [T; 4] {
    [x, x, x, x]
}

#[inline]
pub(crate) fn one2<T: Copy + From<i8>>() -> [T; 2] {
    cast2(const2(1))
}

#[inline]
pub(crate) fn one3<T: Copy + From<i8>>() -> [T; 3] {
    cast3(const3(1))
}

#[inline]
pub(crate) fn one4<T: Copy + From<i8>>() -> [T; 4] {
    cast4(const4(1))
}

#[inline]
pub(crate) fn cast2<T, U>(x: [T; 2]) -> [U; 2]
where
    T: Copy,
    U: Copy + From<T>,
{
    map2(x, cast)
}

#[inline]
pub(crate) fn cast3<T, U>(x: [T; 3]) -> [U; 3]
where
    T: Copy,
    U: Copy + From<T>,
{
    map3(x, cast)
}

#[inline]
pub(crate) fn cast4<T, U>(x: [T; 4]) -> [U; 4]
where
    T: Copy,
    U: Copy + From<T>,
{
    map4(x, cast)
}

/// f64 doesn't implement From<isize>
#[inline]
pub(crate) fn to_f64_2(x: [isize; 2]) -> [f64; 2] {
    [x[0] as f64, x[1] as f64]
}

/// f64 doesn't implement From<isize>
#[inline]
pub(crate) fn to_f64_3(x: [isize; 3]) -> [f64; 3] {
    [x[0] as f64, x[1] as f64, x[2] as f64]
}

/// f64 doesn't implement From<isize>
#[inline]
pub(crate) fn to_f64_4(x: [isize; 4]) -> [f64; 4] {
    [x[0] as f64, x[1] as f64, x[2] as f64, x[3] as f64]
}

// isize doesn't implement From<f64>
#[inline]
pub(crate) fn to_isize2(x: [f64; 2]) -> [isize; 2] {
    [x[0] as isize, x[1] as isize]
}

#[cfg(not(target_os = "emscripten"))]
#[inline]
pub(crate) fn scale_shift(value: f64, n: f64) -> f64 {
    value.abs().mul_add(n, -1.0_f64)
}

#[cfg(target_os = "emscripten")]
#[inline]
pub(crate) fn scale_shift(value: f64, n: f64) -> f64 {
    (value.abs() * n) + -1.0_f64
}

#[inline]
pub(crate) fn to_isize3(x: [f64; 3]) -> [isize; 3] {
    [x[0] as isize, x[1] as isize, x[2] as isize]
}

#[inline]
pub(crate) fn to_isize4(x: [f64; 4]) -> [isize; 4] {
    [x[0] as isize, x[1] as isize, x[2] as isize, x[3] as isize]
}

pub mod interpolate {
    /// Performs linear interpolation between two values.
    #[cfg(not(target_os = "emscripten"))]
    #[inline]
    pub(crate) fn linear(a: f64, b: f64, x: f64) -> f64 {
        x.mul_add(b - a, a)
    }

    /// Performs linear interpolation between two values.
    #[cfg(target_os = "emscripten")]
    #[inline]
    pub(crate) fn linear(a: f64, b: f64, x: f64) -> f64 {
        (x * (b - a)) + a
    }

    /// Performs cubic interpolation between two values bound between two other
    /// values.
    ///
    /// - n0 - The value before the first value.
    /// - n1 - The first value.
    /// - n2 - The second value.
    /// - n3 - The value after the second value.
    /// - alpha - The alpha value.
    ///
    /// The alpha value should range from 0.0 to 1.0. If the alpha value is
    /// 0.0, this function returns _n1_. If the alpha value is 1.0, this
    /// function returns _n2_.
    #[inline]
    pub(crate) fn cubic(n0: f64, n1: f64, n2: f64, n3: f64, alpha: f64) -> f64 {
        let p = (n3 - n2) - (n0 - n1);
        let q = (n0 - n1) - p;
        let r = n2 - n0;
        let s = n1;
        p * alpha * alpha * alpha + q * alpha * alpha + r * alpha + s
    }

    /// Maps a value onto a cubic S-curve.
    #[inline]
    pub(crate) fn s_curve3(x: f64) -> f64 {
        x * x * (3.0 - (x * 2.0))
    }

    /// Maps a value onto a quintic S-curve.
    #[inline]
    pub(crate) fn s_curve5(x: f64) -> f64 {
        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    }
}
