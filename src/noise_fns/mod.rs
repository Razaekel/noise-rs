use rand::{Rng, SeedableRng, XorShiftRng};

pub use self::cache::*;
pub use self::combiners::*;
pub use self::generators::*;
pub use self::modifiers::*;
pub use self::selectors::*;
pub use self::transformers::*;

mod combiners;
mod generators;
mod modifiers;
mod selectors;
mod cache;
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
pub trait NoiseFn<T> {
    fn get(&self, point: T) -> f64;
}

impl<'a, T, M: NoiseFn<T>> NoiseFn<T> for &'a M {
    #[inline]
    fn get(&self, point: T) -> f64 {
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

/// Noise functions that can be randomly generated.
pub trait Random {
    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self;

    fn from_seed(seed: u128) -> Self
        where Self: Sized
    {
        Self::from_rng(&mut seed_rng(seed))
    }
}

fn default<T: Random>() -> T { T::from_rng(&mut default_rng()) }

const DEFAULT_SEED: u128 = 0x52d80a69a14cb7ee252471a2a8ee0185;

fn seed_rng(seed: u128) -> impl Rng {
    let mut bytes = [0; 16];
    for (i, x) in bytes.iter_mut().enumerate() {
        *x = (seed >> (i * 8)) as u8;
    }
    XorShiftRng::from_seed(bytes)
}

fn default_rng() -> impl Rng { seed_rng(DEFAULT_SEED) }
