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

    #[inline]
    fn add<Other>(self, other: Other) -> Add<T, Self, Other, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM> + Sized,
    {
        Add::new(self, other)
    }

    #[inline]
    fn max<Other>(self, other: Other) -> Max<T, Self, Other, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM> + Sized,
    {
        Max::new(self, other)
    }

    #[inline]
    fn min<Other>(self, other: Other) -> Min<T, Self, Other, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM> + Sized,
    {
        Min::new(self, other)
    }

    fn multiply<Other>(self, other: Other) -> Multiply<T, Self, Other, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM>,
    {
        Multiply::new(self, other)
    }

    fn power<Other>(self, other: Other) -> Power<T, Self, Other, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM>,
    {
        Power::new(self, other)
    }

    fn abs(self) -> Abs<T, Self, DIM>
    where
        Self: Sized,
    {
        Abs::new(self)
    }

    fn clamp(self, max: f64, min: f64) -> Clamp<T, Self, DIM>
    where
        Self: Sized,
    {
        Clamp::new(self).set_bounds(min, max)
    }

    fn exponent(self, exponent: f64) -> Exponent<T, Self, DIM>
    where
        Self: Sized,
    {
        Exponent::new(self).set_exponent(exponent)
    }

    fn negate(self) -> Negate<T, Self, DIM>
    where
        Self: Sized,
    {
        Negate::new(self)
    }

    fn scale_bias(self, scale: f64, bias: f64) -> ScaleBias<T, Self, DIM>
    where
        Self: Sized,
    {
        ScaleBias::new(self).set_scale(scale).set_bias(bias)
    }

    fn blend<Other, Control>(
        self,
        other: Other,
        control: Control,
    ) -> Blend<T, Self, Other, Control, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM> + Sized,
        Control: NoiseFn<T, DIM> + Sized,
    {
        Blend::new(self, other, control)
    }

    fn select<Other, Control>(
        self,
        other: Other,
        control: Control,
        lower_bound: f64,
        upper_bound: f64,
        falloff: f64,
    ) -> Select<T, Self, Other, Control, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM> + Sized,
        Control: NoiseFn<T, DIM> + Sized,
    {
        Select::new(self, other, control)
            .set_bounds(lower_bound, upper_bound)
            .set_falloff(falloff)
    }
}

impl<'a, T, M, const DIM: usize> NoiseFn<T, DIM> for &'a M
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
    fn set_seed(self, seed: u32) -> Self;

    /// Getter to retrieve the seed from the function
    fn seed(&self) -> u32;
}
