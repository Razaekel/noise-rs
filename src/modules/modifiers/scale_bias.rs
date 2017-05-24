// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use modules::NoiseModule;
use num_traits::Float;

/// Noise module that applies a scaling factor and a bias to the output value
/// from the source module.
///
/// The module retrieves the output value from the source module, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
pub struct ScaleBias<Source, T> {
    /// Outputs a value.
    pub source: Source,

    /// Scaling factor to apply to the output value from the source module.
    /// The default value is 1.0.
    pub scale: T,

    /// Bias to apply to the scaled output value from the source module.
    /// The default value is 0.0.
    pub bias: T,
}

impl<Source, T> ScaleBias<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> ScaleBias<Source, T> {
        ScaleBias {
            source: source,
            scale: T::one(),
            bias: T::zero(),
        }
    }

    pub fn set_scale(self, scale: T) -> ScaleBias<Source, T> {
        ScaleBias { scale: scale, ..self }
    }

    pub fn set_bias(self, bias: T) -> ScaleBias<Source, T> {
        ScaleBias { bias: bias, ..self }
    }
}

impl<Source, T, U> NoiseModule<T, U> for ScaleBias<Source, U>
    where Source: NoiseModule<T, U>,
          T: Copy,
          U: Float,
{
    fn get(&self, point: T) -> U {
        (self.source.get(point)).mul_add(self.scale, self.bias)
    }
}
