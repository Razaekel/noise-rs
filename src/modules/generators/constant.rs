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
use modules::NoiseModule;

/// Noise module that outputs a constant value.
///
/// This module takes a input, value, and returns that input for all points,
/// producing a contant-valued field.
///
/// This module is not very useful by itself, but can be used as a source
/// module for other noise modules.
#[derive(Clone, Copy, Debug)]
pub struct Constant<T: Float> {
    /// Constant value.
    pub value: T,
}

impl<T: Float> Constant<T> {
    pub fn new(value: T) -> Constant<T> {
        Constant { value: value }
    }
}

impl<T, U> NoiseModule<U> for Constant<T>
    where T: Float,
          U: Copy,
{
    type Output = T;

    fn get(&self, _point: U) -> Self::Output {
        self.value
    }
}
