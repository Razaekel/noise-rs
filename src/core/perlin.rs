use crate::{
    math::{
        s_curve::quintic::Quintic,
        vectors::{Vector, Vector2, Vector3, Vector4},
    },
    permutationtable::NoiseHasher,
};
use num_traits::{Float, NumCast};

// Scale Factor
//
//           1             2
// F = ------------- = ---------
//      sqrt(N) / 2     sqrt(N)
fn scale_factor<F>(n: usize) -> F
where
    F: Float,
{
    let n: F = NumCast::from(n).unwrap();

    F::from(2.0).unwrap() / n.sqrt()
}

macro_rules! perlin_2d {
    ($name:ident, $f:ty) => {
        pub fn $name<NH>(point: [$f; 2], hasher: &NH) -> $f
        where
            NH: NoiseHasher + ?Sized,
        {
            // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
            // Need to invert this value and multiply the unscaled result by the value to get a scaled
            // range of (-1, 1).
            //
            // 1/(sqrt(N)/2), N=2 -> sqrt(2)
            let scale_factor: $f = scale_factor(2);

            let point = Vector2::from(point);

    #[inline(always)]
            #[rustfmt::skip]
            fn gradient_dot_v(perm: usize, point: Vector2<$f>) -> $f {
                let [x, y] = point.into_array();

                match perm & 0b11 {
                    0 =>  x + y, // ( 1,  1)
                    1 => -x + y, // (-1,  1)
                    2 =>  x - y, // ( 1, -1)
                    3 => -x - y, // (-1, -1)
                    _ => unreachable!(),
                }
            }

            let floored = point.floor();
            let corner: Vector2<isize> = floored.numcast().unwrap();
            let distance = point - floored;

            macro_rules! call_gradient(
                                        ($x:expr, $y:expr) => {
                                            {
                                                let offset = Vector2::new($x, $y);
                                                gradient_dot_v(
                                                    hasher.hash(&(corner + offset).into_array()),
                                                    distance - offset.numcast().unwrap()
                                                )
                                            }
                                        }
                                    );

            let g00 = call_gradient!(0, 0);
            let g10 = call_gradient!(1, 0);
            let g01 = call_gradient!(0, 1);
            let g11 = call_gradient!(1, 1);

            let [u, v] = distance.map_quintic().into_array();

            let unscaled_result = {
                let k0 = g00;
                let k1 = g10 - g00;
                let k2 = g01 - g00;
                let k3 = g00 + g11 - g10 - g01;

                k0 + k1 * u + k2 * v + k3 * u * v
            };

            let scaled_result = unscaled_result * scale_factor;

            // At this point, we should be really damn close to the (-1, 1) range, but some float errors
            // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
            // outliers and return it.
            scaled_result.clamp(-1.0, 1.0)
        }
    };
}

perlin_2d!(perlin_2d_f32, f32);
perlin_2d!(perlin_2d_f64, f64);

// #[inline(always)]
// fn bilinear_interpolation(u: f64, v: f64, g00: f64, g01: f64, g10: f64, g11: f64) -> f64 {
//     let k0 = g00;
//     let k1 = g10 - g00;
//     let k2 = g01 - g00;
//     let k3 = g00 + g11 - g10 - g01;
//
//     k0 + k1 * u + k2 * v + k3 * u * v
// }

macro_rules! perlin_3d {
    ($name:ident, $f:ty) => {
        pub fn $name<NH>(point: [$f; 3], hasher: &NH) -> $f
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
            let scale_factor: $f = scale_factor(3);

            let point = Vector3::from(point);

    #[rustfmt::skip]
            fn gradient_dot_v(perm: usize, point: Vector3<$f>) -> $f {
                let [x, y, z] = point.into_array();

                match perm & 0b1111 {
                    0  | 12 =>  x + y    , // ( 1,  1,  0)
                    1  | 13 => -x + y    , // (-1,  1,  0)
                    2       =>  x - y    , // ( 1, -1,  0)
                    3       => -x - y    , // (-1, -1,  0)
                    4       =>  x     + z, // ( 1,  0,  1)
                    5       => -x     + z, // (-1,  0,  1)
                    6       =>  x     - z, // ( 1,  0, -1)
                    7       => -x     - z, // (-1,  0, -1)
                    8       =>      y + z, // ( 0,  1,  1)
                    9  | 14 =>     -y + z, // ( 0, -1,  1)
                    10      =>      y - z, // ( 0,  1, -1)
                    11 | 15 =>     -y - z, // ( 0, -1, -1)
                    _ => unreachable!(),
                }
            }

            let floored = point.floor();
            let corner: Vector3<isize> = floored.numcast().unwrap();
            let distance = point - floored;

            macro_rules! call_gradient(
                                ($x:expr, $y:expr, $z:expr) => {
                                    {
                                        let offset = Vector3::new($x, $y, $z);
                                        gradient_dot_v(
                                            hasher.hash(&(corner + offset).into_array()),
                                            distance - offset.numcast().unwrap()
                                        )
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

            let [a, b, c] = distance.map_quintic().into_array();

            let k0 = g000;
            let k1 = g100 - g000;
            let k2 = g010 - g000;
            let k3 = g001 - g000;
            let k4 = g000 + g110 - g100 - g010;
            let k5 = g000 + g101 - g100 - g001;
            let k6 = g000 + g011 - g010 - g001;
            let k7 = g100 + g010 + g001 + g111 - g000 - g110 - g101 - g011;

            let unscaled_result = k0
                + k1 * a
                + k2 * b
                + k3 * c
                + k4 * a * b
                + k5 * a * c
                + k6 * b * c
                + k7 * a * b * c;

            let scaled_result = unscaled_result * scale_factor;

            // At this point, we should be really damn close to the (-1, 1) range, but some float errors
            // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
            // outliers and return it.
            scaled_result.clamp(-1.0, 1.0)
        }
    };
}

perlin_3d!(perlin_3d_f32, f32);
perlin_3d!(perlin_3d_f64, f64);

macro_rules! perlin_4d {
    ($name:ident, $f:ty) => {
        pub fn $name<NH>(point: [$f; 4], hasher: &NH) -> $f
        where
            NH: NoiseHasher + ?Sized,
        {
            // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
            // Need to invert this value and multiply the unscaled result by the value to get a scaled
            // range of (-1, 1).
            let scale_factor: $f = 1.0; // 1/(sqrt(N)/2), N=4 -> 2/sqrt(4) -> 2/2 -> 1

            let point = Vector4::from(point);

    #[rustfmt::skip]
    fn gradient_dot_v(perm: usize, point: Vector4<$f>) -> $f {
        let [x, y, z, w] = point.into_array();

        match perm & 0b11111 {
            0  | 28 =>  x + y + z    , // ( 1,  1,  1,  0)
            1       => -x + y + z    , // (-1,  1,  1,  0)
            2       =>  x - y + z    , // ( 1, -1,  1,  0)
            3       =>  x + y - z    , // ( 1,  1, -1,  0)
            4       => -x + y - z    , // (-1,  1, -1,  0)
            5       =>  x - y - z    , // ( 1, -1, -1,  0)
            6       =>  x - y - z    , // (-1, -1, -1,  0)
            7  | 29 =>  x + y     + w, // ( 1,  1,  0,  1)
            8       => -x + y     + w, // (-1,  1,  0,  1)
            9       =>  x - y     + w, // ( 1, -1,  0,  1)
            10      =>  x + y     - w, // ( 1,  1,  0, -1)
            11      =>  x + y     - w, // (-1,  1,  0, -1)
            12      =>  x + y     - w, // ( 1, -1,  0, -1)
            13      => -x - y     - w, // (-1, -1,  0, -1)
            14 | 30 =>  x     + z + w, // ( 1,  0,  1,  1)
            15      => -x     + z + w, // (-1,  0,  1,  1)
            16      =>  x     - z + w, // ( 1,  0, -1,  1)
            17      =>  x     + z - w, // ( 1,  0,  1, -1)
            18      =>  x     + z - w, // (-1,  0,  1, -1)
            19      =>  x     + z - w, // ( 1,  0, -1, -1)
            20      => -x     - z - w, // (-1,  0, -1, -1)
            21 | 31 =>      y + z + w, // ( 0,  1,  1,  1)
            22      =>     -y + z + w, // ( 0, -1,  1,  1)
            23      =>      y - z + w, // ( 0,  1, -1,  1)
            24      =>      y - z - w, // ( 0,  1,  1, -1)
            25      =>     -y - z - w, // ( 0, -1,  1, -1)
            26      =>  x + y + z - w, // ( 0,  1, -1, -1)
            27      => -x + y + z - w, // ( 0, -1, -1, -1)
            _ => unreachable!(),
        }
    }

            let floored = point.floor();
            let corner: Vector4<isize> = floored.numcast().unwrap();
            let distance = point - floored;

            macro_rules! call_gradient(
                        ($x:expr, $y:expr, $z:expr, $w:expr) => {
                            {
                                let offset = Vector4::new($x, $y, $z, $w);
                                gradient_dot_v(
                                    hasher.hash(&(corner + offset).into_array()),
                                    distance - offset.numcast().unwrap()
                                )
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

            let [a, b, c, d] = distance.map_quintic().into_array();

            let k0 = g0000;
            let k1 = g1000 - g0000;
            let k2 = g0100 - g0000;
            let k3 = g0010 - g0000;
            let k4 = g0001 - g0000;
            let k5 = g0000 + g1100 - g1000 - g0100;
            let k6 = g0000 + g1010 - g1000 - g0010;
            let k7 = g0000 + g1001 - g1000 - g0001;
            let k8 = g0000 + g0110 - g0100 - g0010;
            let k9 = g0000 + g0101 - g0100 - g0001;
            let k10 = g0000 + g0011 - g0010 - g0001;
            let k11 = g1110 + g1000 + g0100 + g0010 - g0000 - g0111 - g1011 - g1101;
            let k12 = g1101 + g1000 + g0100 + g0001 - g0000 - g0111 - g1011 - g1110;
            let k13 = g1011 + g1000 + g0010 + g0001 - g0000 - g0111 - g1101 - g1110;
            let k14 = g0111 + g0100 + g0010 + g0001 - g0000 - g1011 - g1101 - g1110;
            let k15 = g1111 + g1000 + g0100 + g0010 + g0001 - g0000 - g0111 - g1011 - g1101 - g1110;

            let unscaled_result = k0
                + k1 * a
                + k2 * b
                + k3 * c
                + k4 * d
                + k5 * a * b
                + k6 * a * c
                + k7 * a * d
                + k8 * b * c
                + k9 * b * d
                + k10 * c * d
                + k11 * a * b * c
                + k12 * a * b * d
                + k13 * a * c * d
                + k14 * b * c * d
                + k15 * a * b * c * d;

            let scaled_result = unscaled_result * scale_factor;

            // At this point, we should be really damn close to the (-1, 1) range, but some float errors
            // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
            // outliers and return it.
            scaled_result.clamp(-1.0, 1.0)
        }
    };
}

perlin_4d!(perlin_4d_f32, f32);
perlin_4d!(perlin_4d_f64, f64);
