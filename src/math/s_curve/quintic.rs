use crate::math::vectors::*;
use num_traits::Float;

/// Quintic Interpolation Trait
///
/// Interpolates the provided value according to the quintic S-curve function
/// 6x<sup>5</sup> - 15x<sup>4</sup> + 10x<sup>3</sup>. This creates a curve with endpoints (0,0)
/// and (1,1), and first and second derivatives of zero at the endpoints, allowing the curves to be
/// combined together without discontinuities.
///
/// # Panics
/// Self must be between 0 and 1 inclusive, otherwise this will panic.
pub trait Quintic {
    fn map_quintic(&self) -> Self;
}

impl Quintic for f32 {
    fn map_quintic(&self) -> Self {
        // Validate that `self` is between 0 and 1, inclusive
        assert!(*self >= 0.0, "Self was less than 0!");
        assert!(*self <= 1.0, "Self was greater than 1!");

        self * self * self * (self * (self * 6.0 - 15.0) + 10.0)
    }
}

impl Quintic for f64 {
    fn map_quintic(&self) -> Self {
        // Validate that `self` is between 0 and 1, inclusive
        assert!(*self >= 0.0, "Self was less than 0!");
        assert!(*self <= 1.0, "Self was greater than 1!");

        self * self * self * (self * (self * 6.0 - 15.0) + 10.0)
    }
}

impl<T> Quintic for [T; 2]
where
    T: Float + Quintic,
{
    fn map_quintic(&self) -> Self {
        [self[0].map_quintic(), self[1].map_quintic()]
    }
}

impl<T> Quintic for [T; 3]
where
    T: Float + Quintic,
{
    fn map_quintic(&self) -> Self {
        [
            self[0].map_quintic(),
            self[1].map_quintic(),
            self[2].map_quintic(),
        ]
    }
}

impl<T> Quintic for [T; 4]
where
    T: Float + Quintic,
{
    fn map_quintic(&self) -> Self {
        [
            self[0].map_quintic(),
            self[1].map_quintic(),
            self[2].map_quintic(),
            self[3].map_quintic(),
        ]
    }
}

impl<T> Quintic for Vector2<T>
where
    T: Float + Quintic,
{
    fn map_quintic(&self) -> Self {
        self.map(|x| x.map_quintic())
    }
}

impl<T> Quintic for Vector3<T>
where
    T: Float + Quintic,
{
    fn map_quintic(&self) -> Self {
        self.map(|x| x.map_quintic())
    }
}

impl<T> Quintic for Vector4<T>
where
    T: Float + Quintic,
{
    fn map_quintic(&self) -> Self {
        self.map(|x| x.map_quintic())
    }
}

// impl<T, V, const DIM: usize> Quintic for V
// where
//     T: Float + Quintic,
//     V: Vector<T, DIM>,
// {
//     fn map_quintic(&self) -> Self {
//         self.map(|x| x.map_quintic())
//     }
// }

// #[inline(always)]
// pub fn quintic<F>(x: F) -> F
//     where
//         F: Float,
// {
//     x * x * x * (x * (x * F::from(6.0).unwrap() - F::from(15.0).unwrap()) + F::from(10.0).unwrap())
// }
