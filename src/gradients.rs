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

use math;

pub fn gradient2<T: Float>(index: uint) -> ::Point2<T> {
    let diag: T = math::cast(0.70710678118f32);
    let one: T = math::cast(1.0f32);
    let zero: T = math::cast(0.0f32);
    match index % 8 {
        0 => [ diag,  diag],
        1 => [ diag, -diag],
        2 => [-diag,  diag],
        3 => [-diag, -diag],
        4 => [ one,   zero],
        5 => [-one,   zero],
        6 => [ zero,  one],
        7 => [ zero, -one],
        _ => panic!("Attempt to access 2D gradient {} of 8", index % 8),
    }
}

pub fn gradient3<T: Float>(index: uint) -> ::Point3<T> {
    let diag: T = math::cast(0.70710678118f32);
    let zero: T = math::cast(0.0f32);
    match index % 12 {
        0  => [ diag,  diag,  zero],
        1  => [ diag, -diag,  zero],
        2  => [-diag,  diag,  zero],
        3  => [-diag, -diag,  zero],
        4  => [ diag,  zero,  diag],
        5  => [ diag,  zero, -diag],
        6  => [-diag,  zero,  diag],
        7  => [-diag,  zero, -diag],
        8  => [ zero,  diag,  diag],
        9  => [ zero,  diag, -diag],
        10 => [ zero, -diag,  diag],
        11 => [ zero, -diag, -diag],
        _ => panic!("Attempt to access 3D gradient {} of 12", index % 12),
    }
}

#[inline(always)]
pub fn gradient4<T: Float>(index: uint) -> ::Point4<T> {
    let diag: T = math::cast(0.57735026919f32);
    let zero: T = math::cast(0.0f32);
    match index % 32 {
        0  => [ diag,  diag,  diag,  zero],
        1  => [ diag, -diag,  diag,  zero],
        2  => [-diag,  diag,  diag,  zero],
        3  => [-diag, -diag,  diag,  zero],
        4  => [ diag,  diag, -diag,  zero],
        5  => [ diag, -diag, -diag,  zero],
        6  => [-diag,  diag, -diag,  zero],
        7  => [-diag, -diag, -diag,  zero],
        8  => [ diag,  diag,  zero,  diag],
        9  => [ diag, -diag,  zero,  diag],
        10 => [-diag,  diag,  zero,  diag],
        11 => [-diag, -diag,  zero,  diag],
        12 => [ diag,  diag,  zero, -diag],
        13 => [ diag, -diag,  zero, -diag],
        14 => [-diag,  diag,  zero, -diag],
        15 => [-diag, -diag,  zero, -diag],
        16 => [ diag,  zero,  diag,  diag],
        17 => [ diag,  zero, -diag,  diag],
        18 => [-diag,  zero,  diag,  diag],
        19 => [-diag,  zero, -diag,  diag],
        20 => [ diag,  zero,  diag, -diag],
        21 => [ diag,  zero, -diag, -diag],
        22 => [-diag,  zero,  diag, -diag],
        23 => [-diag,  zero, -diag, -diag],
        24 => [ zero,  diag,  diag,  diag],
        25 => [ zero,  diag, -diag,  diag],
        26 => [ zero, -diag,  diag,  diag],
        27 => [ zero, -diag, -diag,  diag],
        28 => [ zero,  diag,  diag, -diag],
        29 => [ zero,  diag, -diag, -diag],
        30 => [ zero, -diag,  diag, -diag],
        31 => [ zero, -diag, -diag, -diag],
        _ => panic!("Attempt to access 4D gradient {} of 32", index % 32),
    }
}
