// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math;
use num_traits::Float;

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get2<T: Float>(index: usize) -> math::Vector2<T> {
    let zero = T::zero();
    let one = T::one();
    // Vectors are combinations of -1, 0, and 1, precompute the normalized element
    let norm = math::cast(0.7071067811865475);

    match index % 8 {
        0 => [ one,   zero],
        1 => [-one,   zero],
        2 => [ zero,  one],
        3 => [ zero, -one],
        4 => [ norm,  norm],
        5 => [-norm,  norm],
        6 => [ norm, -norm],
        7 => [-norm, -norm],
        _ => panic!("Attempt to access gradient {} of 8", index % 8),
    }
}

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get3<T: Float>(index: usize) -> math::Vector3<T> {
    let zero = T::zero();
    // Vectors are combinations of -1, 0, and 1, precompute the normalized elements
    let norm = math::cast(0.7071067811865475);
    let norm2 = math::cast(0.5773502691896258);

    match index % 32 {
        // 12 edges repeated twice then 8 corners
        0 | 12  => [ norm,  norm,  zero],
        1 | 13  => [-norm,  norm,  zero],
        2 | 14  => [ norm, -norm,  zero],
        3 | 15  => [-norm, -norm,  zero],
        4 | 16  => [ norm,  zero,  norm],
        5 | 17  => [-norm,  zero,  norm],
        6 | 18  => [ norm,  zero, -norm],
        7 | 19  => [-norm,  zero, -norm],
        8 | 20  => [ zero,  norm,  norm],
        9 | 21  => [ zero, -norm,  norm],
        10 | 22 => [ zero,  norm, -norm],
        11 | 23 => [ zero, -norm, -norm],
        24 => [ norm2,  norm2,  norm2],
        25 => [-norm2,  norm2,  norm2],
        26 => [ norm2, -norm2,  norm2],
        27 => [-norm2, -norm2,  norm2],
        28 => [ norm2,  norm2, -norm2],
        29 => [-norm2,  norm2, -norm2],
        30 => [ norm2, -norm2, -norm2],
        31 => [-norm2, -norm2, -norm2],
        _ => panic!("Attempt to access gradient {} of 32", index % 32),
    }
}

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get4<T: Float>(index: usize) -> math::Vector4<T> {
    let zero = T::zero();
    // Vectors are combinations of -1, 0, and 1, precompute the normalized elements
    let norm = math::cast(0.5773502691896258);
    let norm2 = math::cast(0.5);

    match index % 64 {
        // 32 edges then 16 corners repeated twice
        0  => [ zero,  norm,  norm,  norm],
        1  => [ zero,  norm,  norm, -norm],
        2  => [ zero,  norm, -norm,  norm],
        3  => [ zero,  norm, -norm, -norm],
        4  => [ zero, -norm,  norm,  norm],
        5  => [ zero, -norm,  norm, -norm],
        6  => [ zero, -norm, -norm,  norm],
        7  => [ zero, -norm, -norm, -norm],
        8  => [ norm,  zero,  norm,  norm],
        9  => [ norm,  zero,  norm, -norm],
        10 => [ norm,  zero, -norm,  norm],
        11 => [ norm,  zero, -norm, -norm],
        12 => [-norm,  zero,  norm,  norm],
        13 => [-norm,  zero,  norm, -norm],
        14 => [-norm,  zero, -norm,  norm],
        15 => [-norm,  zero, -norm, -norm],
        16 => [ norm,  norm,  zero,  norm],
        17 => [ norm,  norm,  zero, -norm],
        18 => [ norm, -norm,  zero,  norm],
        19 => [ norm, -norm,  zero, -norm],
        20 => [-norm,  norm,  zero,  norm],
        21 => [-norm,  norm,  zero, -norm],
        22 => [-norm, -norm,  zero,  norm],
        23 => [-norm, -norm,  zero, -norm],
        24 => [ norm,  norm,  norm,  zero],
        25 => [ norm,  norm, -norm,  zero],
        26 => [ norm, -norm,  norm,  zero],
        27 => [ norm, -norm, -norm,  zero],
        28 => [-norm,  norm,  norm,  zero],
        29 => [-norm,  norm, -norm,  zero],
        30 => [-norm, -norm,  norm,  zero],
        31 => [-norm, -norm, -norm,  zero],
        32 | 48 => [ norm2,  norm2,  norm2,  norm2],
        33 | 49 => [-norm2,  norm2,  norm2,  norm2],
        34 | 50 => [ norm2, -norm2,  norm2,  norm2],
        35 | 51 => [-norm2, -norm2,  norm2,  norm2],
        36 | 52 => [ norm2,  norm2, -norm2,  norm2],
        37 | 53 => [-norm2,  norm2, -norm2,  norm2],
        38 | 54 => [ norm2,  norm2,  norm2, -norm2],
        39 | 55 => [-norm2,  norm2,  norm2, -norm2],
        40 | 56 => [ norm2, -norm2, -norm2,  norm2],
        41 | 57 => [-norm2, -norm2, -norm2,  norm2],
        42 | 58 => [ norm2, -norm2,  norm2, -norm2],
        43 | 59 => [-norm2, -norm2,  norm2, -norm2],
        44 | 60 => [ norm2,  norm2, -norm2, -norm2],
        45 | 61 => [-norm2,  norm2, -norm2, -norm2],
        46 | 62 => [ norm2, -norm2, -norm2, -norm2],
        47 | 63 => [-norm2, -norm2, -norm2, -norm2],
        _ => panic!("Attempt to access gradient {} of 64", index % 64),
    }
}
