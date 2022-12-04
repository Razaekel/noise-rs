use core::ops::{Add, Mul};
use num_traits::real::Real;
use num_traits::{One, Zero};

pub use self::{vector2::*, vector3::*, vector4::*};

mod vector2;
mod vector3;
mod vector4;

pub trait Vector<T, const DIM: usize> {
    // Create a vector with a single value broadcasted to all elements.
    fn broadcast(value: T) -> Self
    where
        T: Copy;
    // Create a vector with all elements set to zero.
    fn zero() -> Self
    where
        T: Zero + Copy;
    // Create a vector with all elements set to one.
    fn one() -> Self
    where
        T: One + Copy;
    // Create a vector with the elements incrementing by one, starting from zero.
    fn iota() -> Self
    where
        T: Zero + One;

    fn into_array(self) -> [T; DIM];

    /// Dot product between this vector and another vector.
    fn dot(self, other: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy;
    /// The squared magnitude of a vector is its length, squared.
    fn magnitude_squared(self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T>;
    fn magnitude(self) -> T
    where
        T: Add<Output = T> + Real;

    /// Applies the function f to each element of this vector, in-place.
    fn apply<F>(&mut self, f: F)
    where
        T: Copy,
        F: Fn(T) -> T;

    fn min(&self, other: &Self) -> Self
    where
        T: Ord + Copy;
    fn max(&self, other: &Self) -> Self
    where
        T: Ord + Copy;

    fn ceil(&self) -> Self
    where
        T: Real;
    fn floor(&self) -> Self
    where
        T: Real;

    /// Returns the sum of each of this vectors elements
    fn sum(self) -> T
    where
        T: Add<T, Output = T>;

    /// Returns a new vector which elements are the respective square roots of this
    /// vector's elements.
    fn sqrt(self) -> Self
    where
        T: Real;
}

pub trait VectorMap<T, U> {
    type Output;

    /// Returns a copy of this vector with the members converted using the given conversion
    /// closure.
    fn map<F>(&self, f: F) -> Self::Output
    where
        F: Fn(T) -> U;
}
