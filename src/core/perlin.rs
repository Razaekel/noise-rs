use crate::{
    math::{
        interpolate::linear,
        s_curve::quintic::Quintic,
        vectors::{Vector2, Vector3, Vector4},
    },
    permutationtable::NoiseHasher,
};
use core::f64;

#[inline(always)]
pub fn perlin_1d<NH>(point: f64, hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    //
    // 1/(sqrt(N)/2), N=1 -> 2/sqrt(1) -> 2
    const SCALE_FACTOR: f64 = 2.0;

    let corner = point as isize;
    let distance = point - corner as f64;

    macro_rules! call_gradient(
        ($x_offset:expr) => {
            {
                let offset = distance - $x_offset as f64;
                match hasher.hash(&[corner + $x_offset]) & 0b1 {
                    0 =>  offset, // ( 1 )
                    1 => -offset, // (-1 )
                    _ => unreachable!(),
                }
            }
        }
    );

    let g0 = call_gradient!(0);
    let g1 = call_gradient!(1);

    let curve = distance.map_quintic();

    let result = linear(g0, g1, curve) * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    result.clamp(-1.0, 1.0)
}

#[inline(always)]
pub fn perlin_2d<NH>(point: Vector2<f64>, hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    //
    // 1/(sqrt(N)/2), N=2 -> 2/sqrt(2)
    const SCALE_FACTOR: f64 = 2.0 / f64::consts::SQRT_2;

    let corner = point.floor_to_isize();
    let distance = point - corner.numcast().unwrap();

    macro_rules! call_gradient(
        ($x:expr, $y:expr) => {
            {
                let offset = Vector2::new($x, $y);
                let point = distance - offset.numcast().unwrap();

                match hasher.hash(&(corner + offset).into_array()) & 0b11 {
                    0 =>  point.x + point.y, // ( 1,  1)
                    1 => -point.x + point.y, // (-1,  1)
                    2 =>  point.x - point.y, // ( 1, -1)
                    3 => -point.x - point.y, // (-1, -1)
                    _ => unreachable!(),
                }
            }
        }
    );

    let g00 = call_gradient!(0, 0);
    let g10 = call_gradient!(1, 0);
    let g01 = call_gradient!(0, 1);
    let g11 = call_gradient!(1, 1);

    let curve = distance.map_quintic();

    let result = linear(
        linear(g00, g01, curve.y),
        linear(g10, g11, curve.y),
        curve.x,
    ) * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    result.clamp(-1.0, 1.0)
}

#[inline(always)]
pub fn perlin_3d<NH>(point: Vector3<f64>, hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    //
    // 1/(sqrt(N)/2), N=3 -> 2/sqrt(3)
    // sqrt() is not a const function, so use a high-precision value instead.
    // TODO: Replace fixed const values with const fn if sqrt() ever becomes a const function.
    // 2/sqrt(3) = 1.1547005383792515290182975610039149112952035025402537520372046529
    const SCALE_FACTOR: f64 = 1.154_700_538_379_251_5;

    let corner = point.floor_to_isize();
    let distance = point - corner.numcast().unwrap();

    macro_rules! call_gradient(
        ($x:expr, $y:expr, $z:expr) => {
            {
                let offset = Vector3::new($x, $y, $z);
                let point = distance - offset.numcast().unwrap();

                match hasher.hash(&(corner + offset).into_array()) & 0b1111 {
                    0  | 12 =>  point.x + point.y, // ( 1,  1,  0)
                    1  | 13 => -point.x + point.y, // (-1,  1,  0)
                    2       =>  point.x - point.y, // ( 1, -1,  0)
                    3       => -point.x - point.y, // (-1, -1,  0)
                    4       =>  point.x + point.z, // ( 1,  0,  1)
                    5       => -point.x + point.z, // (-1,  0,  1)
                    6       =>  point.x - point.z, // ( 1,  0, -1)
                    7       => -point.x - point.z, // (-1,  0, -1)
                    8       =>  point.y + point.z, // ( 0,  1,  1)
                    9  | 14 => -point.y + point.z, // ( 0, -1,  1)
                    10      =>  point.y - point.z, // ( 0,  1, -1)
                    11 | 15 => -point.y - point.z, // ( 0, -1, -1)
                    _ => unreachable!(),
                }
            }
        }
    );

    let g000 = call_gradient!(0, 0, 0);
    let g100 = call_gradient!(1, 0, 0);
    let g010 = call_gradient!(0, 1, 0);
    let g110 = call_gradient!(1, 1, 0);
    let g001 = call_gradient!(0, 0, 1);
    let g101 = call_gradient!(1, 0, 1);
    let g011 = call_gradient!(0, 1, 1);
    let g111 = call_gradient!(1, 1, 1);

    let curve = distance.map_quintic();

    let result = linear(
        linear(
            linear(g000, g001, curve.z),
            linear(g010, g011, curve.z),
            curve.y,
        ),
        linear(
            linear(g100, g101, curve.z),
            linear(g110, g111, curve.z),
            curve.y,
        ),
        curve.x,
    ) * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    result.clamp(-1.0, 1.0)
}

#[inline(always)]
pub fn perlin_4d<NH>(point: Vector4<f64>, hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    const SCALE_FACTOR: f64 = 1.0; // 1/(sqrt(N)/2), N=4 -> 2/sqrt(4) -> 2/2 -> 1

    let corner = point.floor_to_isize();
    let distance = point - corner.numcast().unwrap();

    macro_rules! call_gradient(
        ($x:expr, $y:expr, $z:expr, $w:expr) => {
            {
                let offset = Vector4::new($x, $y, $z, $w);
                let point = distance - offset.numcast().unwrap();

                match hasher.hash(&(corner + offset).into_array()) & 0b11111 {
                    0  | 28 =>  point.x + point.y + point.z, // ( 1,  1,  1,  0)
                    1       => -point.x + point.y + point.z, // (-1,  1,  1,  0)
                    2       =>  point.x - point.y + point.z, // ( 1, -1,  1,  0)
                    3       =>  point.x + point.y - point.z, // ( 1,  1, -1,  0)
                    4       => -point.x + point.y - point.z, // (-1,  1, -1,  0)
                    5       =>  point.x - point.y - point.z, // ( 1, -1, -1,  0)
                    6       =>  point.x - point.y - point.z, // (-1, -1, -1,  0)
                    7  | 29 =>  point.x + point.y + point.w, // ( 1,  1,  0,  1)
                    8       => -point.x + point.y + point.w, // (-1,  1,  0,  1)
                    9       =>  point.x - point.y + point.w, // ( 1, -1,  0,  1)
                    10      =>  point.x + point.y - point.w, // ( 1,  1,  0, -1)
                    11      =>  point.x + point.y - point.w, // (-1,  1,  0, -1)
                    12      =>  point.x + point.y - point.w, // ( 1, -1,  0, -1)
                    13      => -point.x - point.y - point.w, // (-1, -1,  0, -1)
                    14 | 30 =>  point.x + point.z + point.w, // ( 1,  0,  1,  1)
                    15      => -point.x + point.z + point.w, // (-1,  0,  1,  1)
                    16      =>  point.x - point.z + point.w, // ( 1,  0, -1,  1)
                    17      =>  point.x + point.z - point.w, // ( 1,  0,  1, -1)
                    18      =>  point.x + point.z - point.w, // (-1,  0,  1, -1)
                    19      =>  point.x + point.z - point.w, // ( 1,  0, -1, -1)
                    20      => -point.x - point.z - point.w, // (-1,  0, -1, -1)
                    21 | 31 =>  point.y + point.z + point.w, // ( 0,  1,  1,  1)
                    22      => -point.y + point.z + point.w, // ( 0, -1,  1,  1)
                    23      =>  point.y - point.z + point.w, // ( 0,  1, -1,  1)
                    24      =>  point.y - point.z - point.w, // ( 0,  1,  1, -1)
                    25      => -point.y - point.z - point.w, // ( 0, -1,  1, -1)
                    26      =>  point.y - point.z - point.w, // ( 0,  1, -1, -1)
                    27      => -point.y - point.z - point.w, // ( 0, -1, -1, -1)
                    _ => unreachable!(),
                }
            }
        }
    );

    let g0000 = call_gradient!(0, 0, 0, 0);
    let g1000 = call_gradient!(1, 0, 0, 0);
    let g0100 = call_gradient!(0, 1, 0, 0);
    let g1100 = call_gradient!(1, 1, 0, 0);
    let g0010 = call_gradient!(0, 0, 1, 0);
    let g1010 = call_gradient!(1, 0, 1, 0);
    let g0110 = call_gradient!(0, 1, 1, 0);
    let g1110 = call_gradient!(1, 1, 1, 0);
    let g0001 = call_gradient!(0, 0, 0, 1);
    let g1001 = call_gradient!(1, 0, 0, 1);
    let g0101 = call_gradient!(0, 1, 0, 1);
    let g1101 = call_gradient!(1, 1, 0, 1);
    let g0011 = call_gradient!(0, 0, 1, 1);
    let g1011 = call_gradient!(1, 0, 1, 1);
    let g0111 = call_gradient!(0, 1, 1, 1);
    let g1111 = call_gradient!(1, 1, 1, 1);

    let curve = distance.map_quintic();

    let result = linear(
        linear(
            linear(
                linear(g0000, g0001, curve.w),
                linear(g0010, g0011, curve.w),
                curve.z,
            ),
            linear(
                linear(g0100, g0101, curve.w),
                linear(g0110, g0111, curve.w),
                curve.z,
            ),
            curve.y,
        ),
        linear(
            linear(
                linear(g1000, g1001, curve.w),
                linear(g1010, g1011, curve.w),
                curve.z,
            ),
            linear(
                linear(g1100, g1101, curve.w),
                linear(g1110, g1111, curve.w),
                curve.z,
            ),
            curve.y,
        ),
        curve.x,
    ) * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    result.clamp(-1.0, 1.0)
}
