use crate::noise_fns::NoiseFn;

/// Noise function that outputs a cone.
/// 
/// This noise function takes a 2d point and outputs a cone that is aligned along the z axis.
/// The origin has a value of 1 and points with a distance from the origin beyond the radius
/// of the cone are -1.
#[derive(Clone, Copy, Debug)]
pub struct Cone {
    /// radius of the cone.
    pub radius: f64,
}

impl Cone {
    pub const DEFAULT_RADIUS: f64 = 1.0;

    pub fn new() -> Self {
        Self {
            radius: Self::DEFAULT_RADIUS,
        }
    }

    pub fn set_radius(self, radius: f64) -> Self {
        Self { radius }
    }
}

impl Default for Cone {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseFn<f64, 2> for Cone {
    fn get(&self, point: [f64; 2]) -> f64 {
        let x = point[0];
        let y = point[1];

        // Calculate the distance of the point from the origin.
        let dist_from_center = (x.powi(2) + y.powi(2)).sqrt();

        match dist_from_center > self.radius{
            true => -1f64,
            false => 1.0 - 2.0*(dist_from_center / self.radius)
        }
    }
}
