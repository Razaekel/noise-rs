use crate::{
    math::{interpolate, s_curve::quintic::Quintic, vectors::*},
    permutationtable::NoiseHasher,
};

pub fn value_2d<NH>(point: [f64; 2], hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    let point = Vector2::from(point);

    let floored = point.floor();
    let corner = floored.numcast().unwrap();
    let weight = (point - floored).map_quintic();

    macro_rules! get(
        ($corner:expr, $offset:expr) => {
            {
               hasher.hash(&($corner + Vector2::from($offset)).into_array()) as f64 / 255.0
            }
        }
    );

    let f00 = get!(corner, [0, 0]);
    let f10 = get!(corner, [1, 0]);
    let f01 = get!(corner, [0, 1]);
    let f11 = get!(corner, [1, 1]);

    let d0 = interpolate::linear(f00, f10, weight.x);
    let d1 = interpolate::linear(f01, f11, weight.x);
    let d = interpolate::linear(d0, d1, weight.y);

    d * 2.0 - 1.0
}

pub fn value_3d<NH>(point: [f64; 3], hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    let point = Vector3::from(point);

    let floored = point.floor();
    let corner = floored.numcast().unwrap();
    let weight = (point - floored).map_quintic();

    macro_rules! get(
        ($corner:expr, $offset:expr) => {
            {
               hasher.hash(&($corner + Vector3::from($offset)).into_array()) as f64 / 255.0
            }
        }
    );

    let f000 = get!(corner, [0, 0, 0]);
    let f100 = get!(corner, [1, 0, 0]);
    let f010 = get!(corner, [0, 1, 0]);
    let f110 = get!(corner, [1, 1, 0]);
    let f001 = get!(corner, [0, 0, 1]);
    let f101 = get!(corner, [1, 0, 1]);
    let f011 = get!(corner, [0, 1, 1]);
    let f111 = get!(corner, [1, 1, 1]);

    let d00 = interpolate::linear(f000, f100, weight.x);
    let d01 = interpolate::linear(f001, f101, weight.x);
    let d10 = interpolate::linear(f010, f110, weight.x);
    let d11 = interpolate::linear(f011, f111, weight.x);
    let d0 = interpolate::linear(d00, d10, weight.y);
    let d1 = interpolate::linear(d01, d11, weight.y);
    let d = interpolate::linear(d0, d1, weight.z);

    d * 2.0 - 1.0
}

pub fn value_4d<NH>(point: [f64; 4], hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    let point = Vector4::from(point);

    let floored = point.floor();
    let corner = floored.numcast().unwrap();
    let weight = (point - floored).map_quintic();

    macro_rules! get(
        ($corner:expr, $offset:expr) => {
            {
               hasher.hash(&($corner + Vector4::from($offset)).into_array()) as f64 / 255.0
            }
        }
    );

    let f0000 = get!(corner, [0, 0, 0, 0]);
    let f1000 = get!(corner, [1, 0, 0, 0]);
    let f0100 = get!(corner, [0, 1, 0, 0]);
    let f1100 = get!(corner, [1, 1, 0, 0]);
    let f0010 = get!(corner, [0, 0, 1, 0]);
    let f1010 = get!(corner, [1, 0, 1, 0]);
    let f0110 = get!(corner, [0, 1, 1, 0]);
    let f1110 = get!(corner, [1, 1, 1, 0]);
    let f0001 = get!(corner, [0, 0, 0, 1]);
    let f1001 = get!(corner, [1, 0, 0, 1]);
    let f0101 = get!(corner, [0, 1, 0, 1]);
    let f1101 = get!(corner, [1, 1, 0, 1]);
    let f0011 = get!(corner, [0, 0, 1, 1]);
    let f1011 = get!(corner, [1, 0, 1, 1]);
    let f0111 = get!(corner, [0, 1, 1, 1]);
    let f1111 = get!(corner, [1, 1, 1, 1]);

    let d000 = interpolate::linear(f0000, f1000, weight.x);
    let d010 = interpolate::linear(f0010, f1010, weight.x);
    let d100 = interpolate::linear(f0100, f1100, weight.x);
    let d110 = interpolate::linear(f0110, f1110, weight.x);
    let d001 = interpolate::linear(f0001, f1001, weight.x);
    let d011 = interpolate::linear(f0011, f1011, weight.x);
    let d101 = interpolate::linear(f0101, f1101, weight.x);
    let d111 = interpolate::linear(f0111, f1111, weight.x);
    let d00 = interpolate::linear(d000, d100, weight.y);
    let d10 = interpolate::linear(d010, d110, weight.y);
    let d01 = interpolate::linear(d001, d101, weight.y);
    let d11 = interpolate::linear(d011, d111, weight.y);
    let d0 = interpolate::linear(d00, d10, weight.z);
    let d1 = interpolate::linear(d01, d11, weight.z);
    let d = interpolate::linear(d0, d1, weight.w);

    d * 2.0 - 1.0
}
