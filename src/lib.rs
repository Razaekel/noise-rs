//! A procedural noise generation library for Rust.
//!
//! # Example
//!
//! ```rust
//! use noise::{NoiseFn, Perlin, Seedable};
//!
//! let perlin = Perlin::new(1);
//! let val = perlin.get([42.4, 37.7, 2.8]);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_copy_implementations)]

extern crate alloc;

pub use crate::noise_fns::*;

mod gradient;
mod math;
mod noise_fns;
mod permutationtable;
pub mod utils;
