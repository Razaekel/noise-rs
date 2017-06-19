// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

//! A procedural noise generation library for Rust.
//!
//! # Example
//!
//! ```rust
//! use noise::{NoiseModule, Perlin};
//!
//! let perlin = Perlin::new();
//! let val = perlin.get([42.4, 37.7, 2.8]);
//! ```

#![deny(missing_copy_implementations)]

extern crate num_traits;
extern crate rand;

pub use math::{Point2, Point3, Point4};
pub use modules::*;

mod gradient;
mod math;
mod modules;
mod permutationtable;
