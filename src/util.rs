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

use std::num::{cast, Float};

#[inline]
pub fn lerp<T: Float>(t: T, a: T, b: T) -> T {
    t * (b - a) + a
}

#[inline]
pub fn scurve3<F: Float>(t: F) -> F {
    let three : F = cast(3i).unwrap();
    let two : F = cast(2i).unwrap();
    t * t * (three - (t * two))
}

#[inline]
pub fn scurve5<T: Float>(t: T) -> T {
    t * t * t * (t * (t * cast(6i).unwrap() - cast(15i).unwrap()) + cast(10i).unwrap())
}

pub fn clamp<F: Float>(val: F, min: F, max: F) -> F {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}


