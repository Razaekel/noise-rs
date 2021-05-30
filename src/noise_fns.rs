use num_traits::ToPrimitive;

pub use self::{
    cache::*, combiners::*, generators::*, modifiers::*, selectors::*, transformers::*,
};

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
pub trait NoiseFn<const N: usize> {
    fn get(&self, point: [f64; N]) -> f64;

    fn get_f32(&self, point: [f32; N]) -> f32 {
        let coords = point.iter().map(|p| p.to_f64());
        let mut a: [f64; N] = [0.0; N];
        for (i, c) in coords.enumerate() {
            a[i] = c.unwrap();
        }
        self.get(a) as f32
    }
}

impl<'a, M: NoiseFn<N>, const N: usize> NoiseFn<N> for &'a M {
    #[inline]
    fn get(&self, point: [f64; N]) -> f64 {
        M::get(*self, point)
    }
}

/// Trait for functions that require a seed before generating their values
pub trait Seedable {

    /// Set the seed for the function implementing the `Seedable` trait
    fn set_seed(self, seed: u32) -> Self;

    /// Getter to retrieve the seed from the function
    fn seed(&self) -> u32;
}
