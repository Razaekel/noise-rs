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

pub use brownian::{Brownian2, Brownian3, Brownian4};
pub use cell::{cell2_manhattan, cell3_manhattan, cell4_manhattan};
pub use cell::{cell2_manhattan_inv, cell3_manhattan_inv, cell4_manhattan_inv};
pub use cell::{cell2_manhattan_value, cell3_manhattan_value, cell4_manhattan_value};
pub use cell::{cell2_range, cell3_range, cell4_range};
pub use cell::{cell2_range_inv, cell3_range_inv, cell4_range_inv};
pub use cell::{cell2_seed_point, cell3_seed_point, cell4_seed_point};
pub use cell::{cell2_value, cell3_value, cell4_value};

pub use cell::{range_sqr_euclidian2, range_sqr_euclidian3, range_sqr_euclidian4};
pub use math::{Point2, Point3, Point4};

pub use modules::*;
pub use open_simplex::{open_simplex2, open_simplex3, open_simplex4};
pub use perlin::{perlin2, perlin3, perlin4};
pub use permutationtable::PermutationTable;
pub use value::{value2, value3, value4};

mod gradient;
mod math;
mod permutationtable;

mod brownian;
mod open_simplex;
mod perlin;
mod value;
mod cell;
mod modules;

/// A trait alias for a 2-dimensional noise function.
///
/// This is useful for succinctly parameterising over valid noise functions.
///
/// # Example
///
/// ```rust
/// use noise::{GenFn2, PermutationTable, Point2};
///
/// fn apply_noise2<F: GenFn2<f32>>(t: &PermutationTable, p: &Point2<f32>, f: F) -> f32 { f(t, p) }
/// ```
pub trait GenFn2<T>: Fn(&PermutationTable, &Point2<T>) -> T {}

/// A trait alias for a 3-dimensional noise function.
///
/// This is useful for succinctly parameterising over valid noise functions.
///
/// # Example
///
/// ```rust
/// use noise::{GenFn3, PermutationTable, Point3};
///
/// fn apply_noise3<F: GenFn3<f32>>(t: &PermutationTable, p: &Point3<f32>, f: F) -> f32 { f(t, p) }
/// ```
pub trait GenFn3<T>: Fn(&PermutationTable, &Point3<T>) -> T {}

/// A trait alias for a 4-dimensional noise function.
///
/// This is useful for succinctly parameterising over valid noise functions.
///
/// # Example
///
/// ```rust
/// use noise::{GenFn4, PermutationTable, Point4};
///
/// fn apply_noise4<F: GenFn4<f32>>(t: &PermutationTable, p: &Point4<f32>, f: F) -> f32 { f(t, p) }
/// ```
pub trait GenFn4<T>: Fn(&PermutationTable, &Point4<T>) -> T {}

impl<T, F> GenFn2<T> for F where F: Fn(&PermutationTable, &Point2<T>) -> T, {}
impl<T, F> GenFn3<T> for F where F: Fn(&PermutationTable, &Point3<T>) -> T, {}
impl<T, F> GenFn4<T> for F where F: Fn(&PermutationTable, &Point4<T>) -> T, {}
