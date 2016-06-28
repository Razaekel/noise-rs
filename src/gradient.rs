// Copyright 2013 The Noise-rs Developers.
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

use num_traits::Float;

use math;

#[inline(always)]
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
pub fn get3<T: Float>(index: usize) -> math::Vector3<T> {
    let zero = T::zero();
    // Vectors are combinations of -1, 0, and 1, precompute the normalized elements
    let norm = math::cast(0.7071067811865475);
    let norm2 = math::cast(0.5773502691896258);

    match index % 32 {
        // 12 edges repeated twice then 8 corners
        0  => [ norm,  norm,  zero],
        1  => [-norm,  norm,  zero],
        2  => [ norm, -norm,  zero],
        3  => [-norm, -norm,  zero],
        4  => [ norm,  zero,  norm],
        5  => [-norm,  zero,  norm],
        6  => [ norm,  zero, -norm],
        7  => [-norm,  zero, -norm],
        8  => [ zero,  norm,  norm],
        9  => [ zero, -norm,  norm],
        10 => [ zero,  norm, -norm],
        11 => [ zero, -norm, -norm],
        12 => [ norm,  norm,  zero],
        13 => [-norm,  norm,  zero],
        14 => [ norm, -norm,  zero],
        15 => [-norm, -norm,  zero],
        16 => [ norm,  zero,  norm],
        17 => [-norm,  zero,  norm],
        18 => [ norm,  zero, -norm],
        19 => [-norm,  zero, -norm],
        20 => [ zero,  norm,  norm],
        21 => [ zero, -norm,  norm],
        22 => [ zero,  norm, -norm],
        23 => [ zero, -norm, -norm],
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
        32 => [ norm2,  norm2,  norm2,  norm2],
        33 => [-norm2,  norm2,  norm2,  norm2],
        34 => [ norm2, -norm2,  norm2,  norm2],
        35 => [-norm2, -norm2,  norm2,  norm2],
        36 => [ norm2,  norm2, -norm2,  norm2],
        37 => [-norm2,  norm2, -norm2,  norm2],
        38 => [ norm2,  norm2,  norm2, -norm2],
        39 => [-norm2,  norm2,  norm2, -norm2],
        40 => [ norm2, -norm2, -norm2,  norm2],
        41 => [-norm2, -norm2, -norm2,  norm2],
        42 => [ norm2, -norm2,  norm2, -norm2],
        43 => [-norm2, -norm2,  norm2, -norm2],
        44 => [ norm2,  norm2, -norm2, -norm2],
        45 => [-norm2,  norm2, -norm2, -norm2],
        46 => [ norm2, -norm2, -norm2, -norm2],
        47 => [-norm2, -norm2, -norm2, -norm2],
        48 => [ norm2,  norm2,  norm2,  norm2],
        49 => [-norm2,  norm2,  norm2,  norm2],
        50 => [ norm2, -norm2,  norm2,  norm2],
        51 => [-norm2, -norm2,  norm2,  norm2],
        52 => [ norm2,  norm2, -norm2,  norm2],
        53 => [-norm2,  norm2, -norm2,  norm2],
        54 => [ norm2,  norm2,  norm2, -norm2],
        55 => [-norm2,  norm2,  norm2, -norm2],
        56 => [ norm2, -norm2, -norm2,  norm2],
        57 => [-norm2, -norm2, -norm2,  norm2],
        58 => [ norm2, -norm2,  norm2, -norm2],
        59 => [-norm2, -norm2,  norm2, -norm2],
        60 => [ norm2,  norm2, -norm2, -norm2],
        61 => [-norm2,  norm2, -norm2, -norm2],
        62 => [ norm2, -norm2, -norm2, -norm2],
        63 => [-norm2, -norm2, -norm2, -norm2],
        _ => panic!("Attempt to access gradient {} of 64", index % 64),
    }
}
