use num_traits::Float;

/// Cubic S-Curve
///
/// Maps the provided value onto the cubic S-curve function -2x<sup>3</sup> + 3x<sup>2</sup>.
/// This creates a curve with endpoints (0,0) and (1,1), and a first derivative of zero at the
/// endpoints, allowing the curves to be combined together without discontinuities.
///
/// Values outside the range of [0, 1] will be clamped to the range before mapping.
pub trait Cubic {
    fn map_cubic(&self) -> Self;
}

impl Cubic for f32 {
    fn map_cubic(&self) -> Self {
        let x = self.clamp(0.0, 1.0);

        x * x * (3.0 - (x * 2.0))
    }
}

impl Cubic for f64 {
    fn map_cubic(&self) -> Self {
        let x = self.clamp(0.0, 1.0);

        x * x * (3.0 - (x * 2.0))
    }
}

impl<T> Cubic for [T; 2]
where
    T: Float + Cubic,
{
    fn map_cubic(&self) -> Self {
        [self[0].map_cubic(), self[1].map_cubic()]
    }
}

impl<T> Cubic for [T; 3]
where
    T: Float + Cubic,
{
    fn map_cubic(&self) -> Self {
        [
            self[0].map_cubic(),
            self[1].map_cubic(),
            self[2].map_cubic(),
        ]
    }
}

impl<T> Cubic for [T; 4]
where
    T: Float + Cubic,
{
    fn map_cubic(&self) -> Self {
        [
            self[0].map_cubic(),
            self[1].map_cubic(),
            self[2].map_cubic(),
            self[3].map_cubic(),
        ]
    }
}
