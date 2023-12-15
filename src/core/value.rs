use crate::{
    math::{interpolate::linear, s_curve::quintic::Quintic, vectors::*},
    permutationtable::NoiseHasher,
};

pub fn value_2d<NH>(point: Vector2<f64>, hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    let corner = point.floor_to_isize();
    let weight = (point - corner.numcast().unwrap()).map_quintic();

    macro_rules! get(
        ($offset:expr) => {
            {
               hasher.hash(&(corner + $offset).into_array()) as f64 / 255.0
            }
        }
    );

    let f00 = get!(Vector2::new(0, 0));
    let f10 = get!(Vector2::new(1, 0));
    let f01 = get!(Vector2::new(0, 1));
    let f11 = get!(Vector2::new(1, 1));

    let result = linear(
        linear(f00, f10, weight.x),
        linear(f01, f11, weight.x),
        weight.y,
    );

    result * 2.0 - 1.0
}

pub fn value_3d<NH>(point: Vector3<f64>, hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    let corner = point.floor_to_isize();
    let weight = (point - corner.numcast().unwrap()).map_quintic();

    macro_rules! get(
        ($offset:expr) => {
            {
               hasher.hash(&(corner + $offset).into_array()) as f64 / 255.0
            }
        }
    );

    let f000 = get!(Vector3::new(0, 0, 0));
    let f100 = get!(Vector3::new(1, 0, 0));
    let f010 = get!(Vector3::new(0, 1, 0));
    let f110 = get!(Vector3::new(1, 1, 0));
    let f001 = get!(Vector3::new(0, 0, 1));
    let f101 = get!(Vector3::new(1, 0, 1));
    let f011 = get!(Vector3::new(0, 1, 1));
    let f111 = get!(Vector3::new(1, 1, 1));

    let result = linear(
        linear(
            linear(f000, f100, weight.x),
            linear(f010, f110, weight.x),
            weight.y,
        ),
        linear(
            linear(f001, f101, weight.x),
            linear(f011, f111, weight.x),
            weight.y,
        ),
        weight.z,
    );

    result * 2.0 - 1.0
}

pub fn value_4d<NH>(point: Vector4<f64>, hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    let corner = point.floor_to_isize();
    let weight = (point - corner.numcast().unwrap()).map_quintic();

    macro_rules! get(
        ($offset:expr) => {
            {
               hasher.hash(&(corner + $offset).into_array()) as f64 / 255.0
            }
        }
    );

    let f0000 = get!(Vector4::new(0, 0, 0, 0));
    let f1000 = get!(Vector4::new(1, 0, 0, 0));
    let f0100 = get!(Vector4::new(0, 1, 0, 0));
    let f1100 = get!(Vector4::new(1, 1, 0, 0));
    let f0010 = get!(Vector4::new(0, 0, 1, 0));
    let f1010 = get!(Vector4::new(1, 0, 1, 0));
    let f0110 = get!(Vector4::new(0, 1, 1, 0));
    let f1110 = get!(Vector4::new(1, 1, 1, 0));
    let f0001 = get!(Vector4::new(0, 0, 0, 1));
    let f1001 = get!(Vector4::new(1, 0, 0, 1));
    let f0101 = get!(Vector4::new(0, 1, 0, 1));
    let f1101 = get!(Vector4::new(1, 1, 0, 1));
    let f0011 = get!(Vector4::new(0, 0, 1, 1));
    let f1011 = get!(Vector4::new(1, 0, 1, 1));
    let f0111 = get!(Vector4::new(0, 1, 1, 1));
    let f1111 = get!(Vector4::new(1, 1, 1, 1));

    let result = linear(
        linear(
            linear(
                linear(f0000, f1000, weight.x),
                linear(f0100, f1100, weight.x),
                weight.y,
            ),
            linear(
                linear(f0010, f1010, weight.x),
                linear(f0110, f1110, weight.x),
                weight.y,
            ),
            weight.z,
        ),
        linear(
            linear(
                linear(f0001, f1001, weight.x),
                linear(f0101, f1101, weight.x),
                weight.y,
            ),
            linear(
                linear(f0011, f1011, weight.x),
                linear(f0111, f1111, weight.x),
                weight.y,
            ),
            weight.z,
        ),
        weight.w,
    );

    result * 2.0 - 1.0
}
