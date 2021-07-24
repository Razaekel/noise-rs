use crate::{
    math::vectors::{Vector, Vector2, Vector3, Vector4, VectorMap},
    permutationtable::NoiseHasher,
};
use core::f64;

#[derive(Clone, Copy, Debug)]
pub enum ReturnType {
    Distance,
    Value,
}

pub mod distance_functions {
    pub fn euclidean(p1: &[f64], p2: &[f64]) -> f64 {
        p1.iter()
            .zip(p2)
            .map(|(a, b)| *a - *b)
            .map(|a| a * a)
            .fold(0.0, |acc, x| acc + x)
            .sqrt()
    }

    pub fn euclidean_squared(p1: &[f64], p2: &[f64]) -> f64 {
        p1.iter()
            .zip(p2)
            .map(|(a, b)| *a - *b)
            .map(|a| a * a)
            .fold(0.0, |acc, x| acc + x)
    }

    pub fn manhattan(p1: &[f64], p2: &[f64]) -> f64 {
        p1.iter()
            .zip(p2)
            .map(|(a, b)| *a - *b)
            .map(|a| a.abs())
            .fold(0.0, |acc, x| acc + x)
    }

    pub fn chebyshev(p1: &[f64], p2: &[f64]) -> f64 {
        p1.iter()
            .zip(p2)
            .map(|(a, b)| *a - *b)
            .map(|a| a.abs())
            .fold(f64::MIN, |a, b| a.max(b))
    }

    pub fn quadratic(p1: &[f64], p2: &[f64]) -> f64 {
        #[cfg(not(feature = "std"))]
        use alloc::vec::Vec;

        let temp: Vec<f64> = p1.iter().zip(p2).map(|(a, b)| *a - *b).collect();

        let mut result = 0.0;

        for i in &temp {
            for j in &temp {
                result += *i * *j;
            }
        }

        result
    }
}

pub fn worley_2d<F, NH>(
    hasher: &NH,
    distance_function: F,
    return_type: ReturnType,
    point: [f64; 2],
) -> f64
where
    F: Fn(&[f64], &[f64]) -> f64,
    NH: NoiseHasher + ?Sized,
{
    let point = Vector2::from(point);

    fn get_point(index: usize, whole: Vector2<isize>) -> Vector2<f64> {
        get_vec2(index) + whole.numcast().unwrap()
    }

    let cell = point.floor();
    let whole = cell.numcast().unwrap();
    let frac = point - cell;

    let half = frac.map(|x| x > 0.5);

    let near = whole + half.map(|x| x as isize);
    let far = whole + half.map(|x| !x as isize);

    let mut seed_cell = near;
    let seed_index = hasher.hash(&near.into_array());
    let seed_point = get_point(seed_index, near);
    let mut distance = distance_function(&point.into_array(), &seed_point.into_array());

    let range = frac.map(|x| (0.5 - x).powf(2.0));

    macro_rules! test_point(
        [$x:expr, $y:expr] => {
            {
                let test_point = Vector2::from([$x, $y]);
                let index = hasher.hash(&test_point.into_array());
                let offset = get_point(index, test_point);
                let cur_distance = distance_function(&point.into_array(), &offset.into_array());
                if cur_distance < distance {
                    distance = cur_distance;
                    seed_cell = test_point;
                }
            }
        }
    );

    if range.x < distance {
        test_point![far.x, near.y];
    }

    if range.y < distance {
        test_point![near.x, far.y];
    }

    if range.x < distance && range.y < distance {
        test_point![far.x, far.y];
    }

    let value = match return_type {
        ReturnType::Distance => distance,
        ReturnType::Value => hasher.hash(&seed_cell.into_array()) as f64 / 255.0,
    };

    value * 2.0 - 1.0
}

#[rustfmt::skip]
fn get_vec2(index: usize) -> Vector2<f64> {
    let length = ((index & 0xF8) >> 3) as f64 * 0.5 / 31.0;
    let diag = length * f64::consts::FRAC_1_SQRT_2;

    Vector2::from(match index & 0x07 {
        0 => [   diag,    diag],
        1 => [   diag,   -diag],
        2 => [  -diag,    diag],
        3 => [  -diag,   -diag],
        4 => [ length,     0.0],
        5 => [-length,     0.0],
        6 => [    0.0,  length],
        7 => [    0.0, -length],
        _ => unreachable!(),
    })
}

#[inline(always)]
pub fn worley_3d<F, NH>(
    hasher: &NH,
    distance_function: F,
    return_type: ReturnType,
    point: [f64; 3],
) -> f64
where
    F: Fn(&[f64], &[f64]) -> f64,
    NH: NoiseHasher + ?Sized,
{
    let point = Vector3::from(point);

    fn get_point(index: usize, whole: Vector3<isize>) -> Vector3<f64> {
        get_vec3(index) + whole.numcast().unwrap()
    }

    let cell = point.floor();
    let whole = cell.numcast().unwrap();
    let frac = point - cell;

    let half = frac.map(|x| x > 0.5);

    let near = whole + half.map(|x| x as isize);
    let far = whole + half.map(|x| !x as isize);

    let mut seed_cell = near;
    let seed_index = hasher.hash(&near.into_array());
    let seed_point = get_point(seed_index, near);
    let mut distance = distance_function(&point.into_array(), &seed_point.into_array());

    let range = frac.map(|x| (0.5 - x).powf(2.0));

    macro_rules! test_point(
        [$x:expr, $y:expr, $z:expr] => {
            {
                let test_point = Vector3::from([$x, $y, $z]);
                let index = hasher.hash(&test_point.into_array());
                let offset = get_point(index, test_point);
                let cur_distance = distance_function(&point.into_array(), &offset.into_array());
                if cur_distance < distance {
                    distance = cur_distance;
                    seed_cell = test_point;
                }
            }
        }
    );

    if range.x < distance {
        test_point![far.x, near.y, near.z];
    }
    if range.y < distance {
        test_point![near.x, far.y, near.z];
    }
    if range.z < distance {
        test_point![near.x, near.y, far.z];
    }

    if range.x < distance && range.y < distance {
        test_point![far.x, far.y, near.z];
    }
    if range.x < distance && range.z < distance {
        test_point![far.x, near.y, far.z];
    }
    if range.y < distance && range.z < distance {
        test_point![near.x, far.y, far.z];
    }

    if range.x < distance && range.y < distance && range.z < distance {
        test_point![far.x, far.y, far.z];
    }

    let value = match return_type {
        ReturnType::Distance => distance,
        ReturnType::Value => hasher.hash(&seed_cell.into_array()) as f64 / 255.0,
    };

    value * 2.0 - 1.0
}

#[rustfmt::skip]
#[inline]
fn get_vec3(index: usize) -> Vector3<f64> {
    let length = ((index & 0xE0) >> 5) as f64 * 0.5 / 7.0;
    let diag = length * f64::consts::FRAC_1_SQRT_2;

    Vector3::from(match index % 18 {
        0  => [   diag,    diag,     0.0],
        1  => [   diag,   -diag,     0.0],
        2  => [  -diag,    diag,     0.0],
        3  => [  -diag,   -diag,     0.0],
        4  => [   diag,     0.0,    diag],
        5  => [   diag,     0.0,   -diag],
        6  => [  -diag,     0.0,    diag],
        7  => [  -diag,     0.0,   -diag],
        8  => [    0.0,    diag,    diag],
        9  => [    0.0,    diag,   -diag],
        10 => [    0.0,   -diag,    diag],
        11 => [    0.0,   -diag,   -diag],
        12 => [ length,     0.0,     0.0],
        13 => [    0.0,  length,     0.0],
        14 => [    0.0,     0.0,  length],
        15 => [-length,     0.0,     0.0],
        16 => [    0.0, -length,     0.0],
        17 => [    0.0,     0.0, -length],
        _ => unreachable!("Attempt to access 3D gradient {} of 18", index % 18),
    })
}

#[inline(always)]
#[allow(clippy::cognitive_complexity)]
pub fn worley_4d<F, NH>(
    hasher: &NH,
    distance_function: F,
    return_type: ReturnType,
    point: [f64; 4],
) -> f64
where
    F: Fn(&[f64], &[f64]) -> f64,
    NH: NoiseHasher + ?Sized,
{
    let point = Vector4::from(point);

    fn get_point(index: usize, whole: Vector4<isize>) -> Vector4<f64> {
        get_vec4(index) + whole.numcast().unwrap()
    }

    let cell = point.floor();
    let whole = cell.numcast().unwrap();
    let frac = point - cell;

    let half = frac.map(|x| x > 0.5);

    let near = whole + half.map(|x| x as isize);
    let far = whole + half.map(|x| !x as isize);

    let mut seed_cell = near;
    let seed_index = hasher.hash(&near.into_array());
    let seed_point = get_point(seed_index, near);
    let mut distance = distance_function(&point.into_array(), &seed_point.into_array());

    let range = frac.map(|x| (0.5 - x).powf(2.0));

    macro_rules! test_point(
        [$x:expr, $y:expr, $z:expr, $w:expr] => {
            {
                let test_point = Vector4::from([$x, $y, $z, $w]);
                let index = hasher.hash(&test_point.into_array());
                let offset = get_point(index, test_point);
                let cur_distance = distance_function(&point.into_array(), &offset.into_array());
                if cur_distance < distance {
                    distance = cur_distance;
                    seed_cell = test_point;
                }
            }
        }
    );

    if range.x < distance {
        test_point![far.x, near.y, near.z, near.w];
    }
    if range.y < distance {
        test_point![near.x, far.y, near.z, near.w];
    }
    if range.z < distance {
        test_point![near.x, near.y, far.z, near.w];
    }
    if range.w < distance {
        test_point![near.x, near.y, near.z, far.w];
    }

    if range.x < distance && range.y < distance {
        test_point![far.x, far.y, near.z, near.w];
    }
    if range.x < distance && range.z < distance {
        test_point![far.x, near.y, far.z, near.w];
    }
    if range.x < distance && range.w < distance {
        test_point![far.x, near.y, near.z, far.w];
    }
    if range.y < distance && range.z < distance {
        test_point![near.x, far.y, far.z, near.w];
    }
    if range.y < distance && range.w < distance {
        test_point![near.x, far.y, near.z, far.w];
    }
    if range.z < distance && range.w < distance {
        test_point![near.x, near.y, far.z, far.w];
    }

    if range.x < distance && range.y < distance && range.z < distance {
        test_point![far.x, far.y, far.z, near.w];
    }
    if range.x < distance && range.y < distance && range.w < distance {
        test_point![far.x, far.y, near.z, far.w];
    }
    if range.x < distance && range.z < distance && range.w < distance {
        test_point![far.x, near.y, far.z, far.w];
    }
    if range.y < distance && range.z < distance && range.w < distance {
        test_point![near.x, far.y, far.z, far.w];
    }

    if range.x < distance && range.y < distance && range.z < distance && range.w < distance {
        test_point![far.x, far.y, far.z, far.w];
    }

    let value = match return_type {
        ReturnType::Distance => distance,
        ReturnType::Value => hasher.hash(&seed_cell.into_array()) as f64 / 255.0,
    };

    value * 2.0 - 1.0
}

#[rustfmt::skip]
#[inline(always)]
fn get_vec4(index: usize) -> Vector4<f64> {
    let length = ((index & 0xE0) >> 5) as f64 * 0.5 / 7.0;
    let diag = length * 0.577_350_269_189_625_8;

    Vector4::from(match index % 32 {
        0  => [ diag,  diag,  diag,  0.0],
        1  => [ diag, -diag,  diag,  0.0],
        2  => [-diag,  diag,  diag,  0.0],
        3  => [-diag, -diag,  diag,  0.0],
        4  => [ diag,  diag, -diag,  0.0],
        5  => [ diag, -diag, -diag,  0.0],
        6  => [-diag,  diag, -diag,  0.0],
        7  => [-diag, -diag, -diag,  0.0],
        8  => [ diag,  diag,  0.0,  diag],
        9  => [ diag, -diag,  0.0,  diag],
        10 => [-diag,  diag,  0.0,  diag],
        11 => [-diag, -diag,  0.0,  diag],
        12 => [ diag,  diag,  0.0, -diag],
        13 => [ diag, -diag,  0.0, -diag],
        14 => [-diag,  diag,  0.0, -diag],
        15 => [-diag, -diag,  0.0, -diag],
        16 => [ diag,  0.0,  diag,  diag],
        17 => [ diag,  0.0, -diag,  diag],
        18 => [-diag,  0.0,  diag,  diag],
        19 => [-diag,  0.0, -diag,  diag],
        20 => [ diag,  0.0,  diag, -diag],
        21 => [ diag,  0.0, -diag, -diag],
        22 => [-diag,  0.0,  diag, -diag],
        23 => [-diag,  0.0, -diag, -diag],
        24 => [ 0.0,  diag,  diag,  diag],
        25 => [ 0.0,  diag, -diag,  diag],
        26 => [ 0.0, -diag,  diag,  diag],
        27 => [ 0.0, -diag, -diag,  diag],
        28 => [ 0.0,  diag,  diag, -diag],
        29 => [ 0.0,  diag, -diag, -diag],
        30 => [ 0.0, -diag,  diag, -diag],
        31 => [ 0.0, -diag, -diag, -diag],
        _ => unreachable!("Attempt to access 4D gradient {} of 32", index % 32),
    })
}
