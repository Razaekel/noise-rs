//! An ultra-light private math library to make our short lives easier as we
//! implement super-complex noise stuff.

use num_traits::Float;

pub(crate) mod interpolate;
pub(crate) mod s_curve;
pub(crate) mod vectors;

#[cfg(not(target_os = "emscripten"))]
#[inline]
pub(crate) fn scale_shift<F: Float>(value: F, n: F) -> F {
    value.abs().mul_add(n, -F::one())
}

#[cfg(target_os = "emscripten")]
#[inline]
pub(crate) fn scale_shift<F: Float>(value: F, n: F) -> F {
    (value.abs() * n) + -F::one()
}
