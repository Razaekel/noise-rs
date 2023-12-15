use crate::math::vectors::{Vector2, Vector3, Vector4};

macro_rules! impl_sphere {
    ($name:ident, $vector:ty) => {
        #[inline(always)]
        pub fn $name(point: $vector, frequency: f64) -> f64 {
            let point = point * frequency;

            let dist_from_center = point.magnitude();

            let dist_from_smaller_sphere = dist_from_center - dist_from_center.floor();
            let dist_from_larger_sphere = 1.0 - dist_from_smaller_sphere;
            let nearest_dist = dist_from_smaller_sphere.min(dist_from_larger_sphere);

            1.0 - (nearest_dist * 4.0)
        }
    };
}

impl_sphere!(spheres_2d, Vector2<f64>);
impl_sphere!(spheres_3d, Vector3<f64>);
impl_sphere!(spheres_4d, Vector4<f64>);
