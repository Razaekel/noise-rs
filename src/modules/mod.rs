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

pub use self::cache::*;
pub use self::combiners::*;
pub use self::generators::*;
pub use self::modifiers::*;
pub use self::selectors::*;
pub use self::transformers::*;

mod combiners;
mod generators;
mod modifiers;
mod selectors;
mod cache;
mod transformers;

/// Base trait for noise modules.
///
/// A noise module is a object that calculates and outputs a value given a
/// n-Dimensional input value, where n is (2,3,4).
///
/// Each type of noise module uses a specific method to calculate an output
/// value. Some of these methods include:
///
/// * Calculating a value using a coherent-noise function or some other
///     mathematical function.
/// * Mathematically changing the output value from another noise module
///     in various ways.
/// * Combining the output values from two noise modules in various ways.
pub trait NoiseModule<T> {
    type Output;

    fn get(&self, point: T) -> Self::Output;
}

impl<'a, T, M: NoiseModule<T>> NoiseModule<T> for &'a M {
    type Output = M::Output;

    #[inline]
    fn get(&self, point: T) -> M::Output {
        M::get(*self, point)
    }
}

/// Trait for modules that require a seed before generating their values
pub trait Seedable {
    fn set_seed(self, seed: usize) -> Self;
}
