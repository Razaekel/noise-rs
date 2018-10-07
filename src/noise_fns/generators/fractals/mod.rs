pub use self::basicmulti::*;
pub use self::billow::*;
pub use self::fbm::*;
pub use self::hybridmulti::*;
pub use self::ridgedmulti::*;

use rand::Rng;

use noise_fns::{seed_rng, default_rng};

mod basicmulti;
mod billow;
mod fbm;
mod hybridmulti;
mod ridgedmulti;

use noise_fns::{Perlin, Seedable};

/// Trait for `MultiFractal` functions
pub trait MultiFractal {
    fn set_octaves(self, octaves: usize) -> Self;

    fn set_frequency(self, frequency: f64) -> Self;

    fn set_lacunarity(self, lacunarity: f64) -> Self;

    fn set_persistence(self, persistence: f64) -> Self;
}

pub trait RandomFractal {
    /// Generate a new random fractal noise function.
    ///
    /// `octaves` is the total number of frequency octaves to generate the noise with. The number of octaves control the
    /// _amount of detail_ in the noise function. Adding more octaves increases the detail, with the drawback of
    /// increasing the calculation time.
    ///
    /// `params` dictates the relationship between successive octaves; see `FractalParams` for details.
    fn from_rng<R: Rng + ?Sized>(rng: &mut R, octaves: usize, params: FractalParams) -> Self;

    fn from_seed(seed: u128, octaves: usize, params: FractalParams) -> Self
    where Self: Sized
    {
        Self::from_rng(&mut seed_rng(seed), octaves, params)
    }

    fn new(octaves: usize, params: FractalParams) -> Self
    where Self: Sized
    {
        Self::from_rng(&mut default_rng(), octaves, params)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FractalParams {
    /// The number of cycles per unit length that the noise function outputs.
    pub frequency: f64,

    /// A multiplier that determines how quickly the frequency increases for
    /// each successive octave in the noise function.
    ///
    /// The frequency of each successive octave is equal to the product of the
    /// previous octave's frequency and the lacunarity value.
    ///
    /// A lacunarity of 2.0 results in the frequency doubling every octave. For
    /// almost all cases, 2.0 is a good value to use.
    pub lacunarity: f64,

    /// A multiplier that determines how quickly the amplitudes diminish for
    /// each successive octave in the noise function.
    ///
    /// The amplitude of each successive octave is equal to the product of the
    /// previous octave's amplitude and the persistence value. Increasing the
    /// persistence produces "rougher" noise.
    pub persistence: f64,
}

fn build_sources(seed: u32, octaves: usize) -> Vec<Perlin> {
    let mut sources = Vec::with_capacity(octaves);
    for x in 0..octaves {
        sources.push(Perlin::new().set_seed(seed + x as u32));
    }
    sources
}
