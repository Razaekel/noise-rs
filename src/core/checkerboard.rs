use crate::math::vectors::{Vector2, Vector3, Vector4};

#[inline(always)]
pub fn checkerboard_2d(point: Vector2<f64>, grid_size: f64) -> f64 {
    let floor = (point / grid_size).floor_to_isize();
    if (floor.x & 1) ^ (floor.y & 1) == 0 {
        -1.0
    } else {
        1.0
    }
}

#[inline(always)]
pub fn checkerboard_3d(point: Vector3<f64>, grid_size: f64) -> f64 {
    let floor = (point / grid_size).floor_to_isize();
    if (floor.x & 1) ^ (floor.y & 1) ^ (floor.z & 1) == 0 {
        -1.0
    } else {
        1.0
    }
}

#[inline(always)]
pub fn checkerboard_4d(point: Vector4<f64>, grid_size: f64) -> f64 {
    let floor = (point / grid_size).floor_to_isize();
    if (floor.x & 1) ^ (floor.y & 1) ^ (floor.z & 1) ^ (floor.w & 1) == 0 {
        -1.0
    } else {
        1.0
    }
}
