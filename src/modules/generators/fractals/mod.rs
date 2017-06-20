// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

pub use self::basicmulti::*;
pub use self::billow::*;
pub use self::fbm::*;
pub use self::hybridmulti::*;
pub use self::ridgedmulti::*;

mod basicmulti;
mod billow;
mod fbm;
mod hybridmulti;
mod ridgedmulti;

use modules::{Perlin, Seedable};

/// Trait for `MultiFractal` modules
pub trait MultiFractal<T> {
    fn set_octaves(self, octaves: usize) -> Self;

    fn set_frequency(self, frequency: T) -> Self;

    fn set_lacunarity(self, lacunarity: T) -> Self;

    fn set_persistence(self, persistence: T) -> Self;
}

fn build_sources(seed: u32, octaves: usize) -> Vec<Perlin> {
    let mut sources = Vec::with_capacity(octaves);
    for x in 0..octaves {
        sources.push(Perlin::new().set_seed(seed + x as u32));
    }
    sources
}
