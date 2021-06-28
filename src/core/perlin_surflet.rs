use crate::{
    gradient,
    math::vectors::{Vector, Vector2, Vector3, Vector4},
    permutationtable::NoiseHasher,
};

#[inline(always)]
pub fn perlin_surflet_2d(point: [f64; 2], hasher: &dyn NoiseHasher) -> f64 {
    const SCALE_FACTOR: f64 = 3.160_493_827_160_493_7;

    #[inline(always)]
    fn surflet(hasher: &dyn NoiseHasher, corner: Vector2<isize>, distance: Vector2<f64>) -> f64 {
        let attn: f64 = 1.0 - distance.magnitude_squared();

        if attn > 0.0 {
            attn.powi(4)
                * distance.dot(Vector2::from(gradient::grad2(
                    hasher.hash(&corner.into_array()),
                )))
        } else {
            0.0
        }
    }

    let point = Vector2::from(point);

    let floored = point.floor();
    let corner = floored.numcast().unwrap();
    let distance = point - floored;

    macro_rules! call_surflet(
        ($x:expr, $y:expr) => {
            {
                let offset = Vector2::new($x, $y);
                surflet(hasher, corner + offset, distance - offset.numcast().unwrap())
            }
        }
    );

    let f00 = call_surflet!(0, 0);
    let f10 = call_surflet!(1, 0);
    let f01 = call_surflet!(0, 1);
    let f11 = call_surflet!(1, 1);

    // Multiply by arbitrary value to scale to -1..1
    ((f00 + f10 + f01 + f11) * SCALE_FACTOR).clamp(-1.0, 1.0)
}

pub fn perlin_surflet_3d(point: [f64; 3], hasher: &dyn NoiseHasher) -> f64 {
    const SCALE_FACTOR: f64 = 3.889_855_325_553_107_4;

    #[inline(always)]
    fn surflet(hasher: &dyn NoiseHasher, corner: Vector3<isize>, distance: Vector3<f64>) -> f64 {
        let attn: f64 = 1.0 - distance.magnitude_squared();

        if attn > 0.0 {
            attn.powi(4)
                * distance.dot(Vector3::from(gradient::grad3(
                    hasher.hash(&corner.into_array()),
                )))
        } else {
            0.0
        }
    }

    let point = Vector3::from(point);

    let floored = point.floor();
    let corner = floored.numcast().unwrap();
    let distance = point - floored;

    macro_rules! call_surflet(
        ($x:expr, $y:expr, $z:expr) => {
            {
                let offset = Vector3::new($x, $y, $z);
                surflet(hasher, corner + offset, distance - offset.numcast().unwrap())
            }
        }
    );

    let f000 = call_surflet!(0, 0, 0);
    let f100 = call_surflet!(1, 0, 0);
    let f010 = call_surflet!(0, 1, 0);
    let f110 = call_surflet!(1, 1, 0);
    let f001 = call_surflet!(0, 0, 1);
    let f101 = call_surflet!(1, 0, 1);
    let f011 = call_surflet!(0, 1, 1);
    let f111 = call_surflet!(1, 1, 1);

    // Multiply by arbitrary value to scale to -1..1
    ((f000 + f100 + f010 + f110 + f001 + f101 + f011 + f111) * SCALE_FACTOR).clamp(-1.0, 1.0)
}

pub fn perlin_surflet_4d(point: [f64; 4], hasher: &dyn NoiseHasher) -> f64 {
    const SCALE_FACTOR: f64 = 4.424_369_240_215_691;

    #[inline(always)]
    fn surflet(hasher: &dyn NoiseHasher, corner: Vector4<isize>, distance: Vector4<f64>) -> f64 {
        let attn: f64 = 1.0 - distance.magnitude_squared();

        if attn > 0.0 {
            attn.powi(4)
                * distance.dot(Vector4::from(gradient::grad4(
                    hasher.hash(&corner.into_array()),
                )))
        } else {
            0.0
        }
    }

    let point = Vector4::from(point);

    let floored = point.floor();
    let corner = floored.numcast().unwrap();
    let distance = point - floored;

    macro_rules! call_surflet(
        ($x:expr, $y:expr, $z:expr, $w:expr) => {
            {
                let offset = Vector4::new($x, $y, $z, $w);
                surflet(hasher, corner + offset, distance - offset.numcast().unwrap())
            }
        }
    );

    let f0000 = call_surflet!(0, 0, 0, 0);
    let f1000 = call_surflet!(1, 0, 0, 0);
    let f0100 = call_surflet!(0, 1, 0, 0);
    let f1100 = call_surflet!(1, 1, 0, 0);
    let f0010 = call_surflet!(0, 0, 1, 0);
    let f1010 = call_surflet!(1, 0, 1, 0);
    let f0110 = call_surflet!(0, 1, 1, 0);
    let f1110 = call_surflet!(1, 1, 1, 0);
    let f0001 = call_surflet!(0, 0, 0, 1);
    let f1001 = call_surflet!(1, 0, 0, 1);
    let f0101 = call_surflet!(0, 1, 0, 1);
    let f1101 = call_surflet!(1, 1, 0, 1);
    let f0011 = call_surflet!(0, 0, 1, 1);
    let f1011 = call_surflet!(1, 0, 1, 1);
    let f0111 = call_surflet!(0, 1, 1, 1);
    let f1111 = call_surflet!(1, 1, 1, 1);

    // Multiply by arbitrary value to scale to -1..1
    ((f0000
        + f1000
        + f0100
        + f1100
        + f0010
        + f1010
        + f0110
        + f1110
        + f0001
        + f1001
        + f0101
        + f1101
        + f0011
        + f1011
        + f0111
        + f1111)
        * SCALE_FACTOR)
        .clamp(-1.0, 1.0)
}
