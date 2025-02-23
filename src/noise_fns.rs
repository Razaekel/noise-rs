pub use self::{
    cache::*, combiners::*, generators::*, modifiers::*, selectors::*, transformers::*,
};
use alloc::boxed::Box;

mod cache;
mod combiners;
mod generators;
mod modifiers;
mod selectors;
mod transformers;

/// Base trait for noise functions.
///
/// A noise function is a object that calculates and outputs a value given a
/// n-Dimensional input value, where n is (2,3,4).
///
/// Each type of noise function uses a specific method to calculate an output
/// value. Some of these methods include:
///
/// * Calculating a value using a coherent-noise function or some other
///     mathematical function.
/// * Mathematically changing the output value from another noise function
///     in various ways.
/// * Combining the output values from two noise functions in various ways.
pub trait NoiseFn<T, const DIM: usize> {
    fn get(&self, point: [T; DIM]) -> f64;
}

impl<T, M, const DIM: usize> NoiseFn<T, DIM> for &M
where
    M: NoiseFn<T, DIM> + ?Sized,
{
    #[inline]
    fn get(&self, point: [T; DIM]) -> f64 {
        M::get(*self, point)
    }
}

impl<T, M, const DIM: usize> NoiseFn<T, DIM> for Box<M>
where
    M: NoiseFn<T, DIM> + ?Sized,
{
    #[inline]
    fn get(&self, point: [T; DIM]) -> f64 {
        M::get(self, point)
    }
}

/// Trait for functions that require a seed before generating their values
pub trait Seedable {
    /// Set the seed for the function implementing the `Seedable` trait
    fn set_seed(self, seed: Seed) -> Self;

    /// Getter to retrieve the seed from the function
    fn seed(&self) -> Seed;
}

pub type Seed = [u8; 16];

pub const DEFAULT_SEED: Seed = [
    0x4f, 0x09, 0xd6, 0x9f, 0x62, 0x9b, 0x09, 0x0c, 0x0c, 0x49, 0x09, 0xfe, 0x6f, 0x1d, 0x4a, 0x38,
];
