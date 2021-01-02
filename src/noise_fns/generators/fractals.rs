pub use self::{basicmulti::*, billow::*, fbm::*, hybridmulti::*, ridgedmulti::*};

mod basicmulti;
mod billow;
mod fbm;
mod hybridmulti;
mod ridgedmulti;

use crate::noise_fns::Perlin;

/// Trait for `MultiFractal` functions
pub trait MultiFractal {
    fn set_octaves(self, octaves: usize) -> Self;

    fn set_frequency(self, frequency: f64) -> Self;

    fn set_lacunarity(self, lacunarity: f64) -> Self;

    fn set_persistence(self, persistence: f64) -> Self;
}

fn build_sources(seed: u32, octaves: usize) -> Vec<Perlin> {
    let mut sources = Vec::with_capacity(octaves);
    for x in 0..octaves {
        sources.push(Perlin::new(seed + x as u32));
    }
    sources
}
