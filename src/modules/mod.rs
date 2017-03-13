// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

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
