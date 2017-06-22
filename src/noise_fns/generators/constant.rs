// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use noise_fns::NoiseFn;
use num_traits::Float;

/// Noise function that outputs a constant value.
///
/// This function takes a input, value, and returns that input for all points,
/// producing a constant-valued field.
///
/// This function is not very useful by itself, but can be used as a source
/// function for other noise functions.
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

impl<T, U> NoiseFn<T, U> for Constant<U>
where
    T: Copy,
    U: Float,
{
    fn get(&self, _point: T) -> U {
        self.value
    }
}
