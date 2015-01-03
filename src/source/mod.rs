use std::num::Float;

pub use self::perlin::Perlin;
pub use self::ridgedmulti::RidgedMulti;

#[deprecated = "Use noise::{perlin2, perlin3, perlin4}, with noise::{Brownian2, Brownian3, Brownian4}"]
pub mod perlin;
pub mod ridgedmulti;

/// A source of noise values.
pub trait Source {
    /// For the given coordinate, return the value
    /// The value is between -1 and 1
    fn get<F:Float>(&self, x: F, y: F, z: F) -> F;
}
