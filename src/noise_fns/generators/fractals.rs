pub use self::{basicmulti::*, billow::*, fbm::*, hybridmulti::*, ridgedmulti::*};
use alloc::vec::Vec;

mod basicmulti;
mod billow;
mod fbm;
mod hybridmulti;
mod ridgedmulti;

use crate::Seedable;

/// Trait for `MultiFractal` functions
pub trait MultiFractal {
    fn set_octaves(self, octaves: usize) -> Self;

    fn set_frequency(self, frequency: f64) -> Self;

    fn set_lacunarity(self, lacunarity: f64) -> Self;

    fn set_persistence(self, persistence: f64) -> Self;
}

fn build_sources<Source>(seed: u32, octaves: usize) -> Vec<Source>
where
    Source: Default + Seedable,
{
    let mut sources = Vec::with_capacity(octaves);
    for x in 0..octaves {
        let source = Source::default();
        sources.push(source.set_seed(seed + x as u32));
    }
    sources
}
