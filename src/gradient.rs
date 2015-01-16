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

#[inline(always)]
fn gradient<T: Float>(index: usize) -> math::Point4<T> {
    let one: T = Float::one();
    let zero: T = Float::zero();

    match index % 32 {
        0  => [ zero,  one,   one,   one],
        1  => [ zero,  one,   one,  -one],
        2  => [ zero,  one,  -one,   one],
        3  => [ zero,  one,  -one,  -one],
        4  => [ zero, -one,   one,   one],
        5  => [ zero, -one,   one,  -one],
        6  => [ zero, -one,  -one,   one],
        7  => [ zero, -one,  -one,  -one],
        8  => [ one,   zero,  one,   one],
        9  => [ one,   zero,  one,  -one],
        10 => [ one,   zero, -one,   one],
        11 => [ one,   zero, -one,  -one],
        12 => [-one,   zero,  one,   one],
        13 => [-one,   zero,  one,  -one],
        14 => [-one,   zero, -one,   one],
        15 => [-one,   zero, -one,  -one],
        16 => [ one,   one,   zero,  one],
        17 => [ one,   one,   zero, -one],
        18 => [ one,  -one,   zero,  one],
        19 => [ one,  -one,   zero, -one],
        20 => [-one,   one,   zero,  one],
        21 => [-one,   one,   zero, -one],
        22 => [-one,  -one,   zero,  one],
        23 => [-one,  -one,   zero, -one],
        24 => [ one,   one,   one,   zero],
        25 => [ one,   one,  -one,   zero],
        26 => [ one,  -one,   one,   zero],
        27 => [ one,  -one,  -one,   zero],
        28 => [-one,   one,   one,   zero],
        29 => [-one,   one,  -one,   zero],
        30 => [-one,  -one,   one,   zero],
        31 => [-one,  -one,  -one,   zero],
        _ => panic!("Attempt to access gradient {} of 32", index % 32),
    }
}

#[inline(always)]
pub fn get2<T: Float>(index: usize) -> math::Point2<T> {
    let value = gradient(index);
    [value[0], value[1]]
}

#[inline(always)]
pub fn get3<T: Float>(index: usize) -> math::Point3<T> {
    let value = gradient(index);
    [value[0], value[1], value[2]]
}

#[inline(always)]
pub fn get4<T: Float>(index: usize) -> math::Point4<T> {
    gradient(index)
}
