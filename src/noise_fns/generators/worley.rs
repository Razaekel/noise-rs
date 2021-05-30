use crate::{
    math,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::{NoiseHasher, PermutationTable},
};

/// Noise function that outputs Worley noise.
pub struct Worley {
    /// Specifies the distance function to use when calculating the boundaries of
    /// the cell.
    pub distance_function: Box<dyn Fn(&[f64], &[f64]) -> f64>,

    /// Signifies whether the distance from the borders of the cell should be returned, or the
    /// value for the cell.
    pub return_type: ReturnType,

    /// Frequency of the seed points.
    pub frequency: f64,

    seed: u32,
    perm_table: PermutationTable,
}

impl Worley {
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;

    pub fn new(seed: u32) -> Self {
        Self {
            perm_table: PermutationTable::new(seed),
            seed,
            distance_function: Box::new(distance_functions::euclidean),
            return_type: ReturnType::Value,
            frequency: Self::DEFAULT_FREQUENCY,
        }
    }

    /// Sets the distance function used by the Worley cells.
    pub fn set_distance_function<F>(self, function: F) -> Self
    where
        F: Fn(&[f64], &[f64]) -> f64 + 'static,
    {
        Self {
            distance_function: Box::new(function),
            ..self
        }
    }

    /// Enables or disables applying the distance from the nearest seed point
    /// to the output value.
    pub fn set_return_type(self, return_type: ReturnType) -> Self {
        Self {
            return_type,
            ..self
        }
    }

    /// Sets the frequency of the seed points.
    pub fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }
}

impl Default for Worley {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl Seedable for Worley {
    /// Sets the seed value used by the Worley cells.
    fn set_seed(self, seed: u32) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Self {
            perm_table: PermutationTable::new(seed),
            seed,
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

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
            .fold(std::f64::MIN, |a, b| a.max(b))
    }

    pub fn quadratic(p1: &[f64], p2: &[f64]) -> f64 {
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

impl NoiseFn<2> for Worley {
    fn get(&self, point: [f64; 2]) -> f64 {
        worley_2d(
            &self.perm_table,
            &self.distance_function,
            self.return_type,
            math::mul2(point, self.frequency),
        )
    }
}

#[inline]
fn worley_2d<F>(
    hasher: &dyn NoiseHasher,
    distance_function: F,
    return_type: ReturnType,
    point: [f64; 2],
) -> f64
where
    F: Fn(&[f64], &[f64]) -> f64,
{
    #[inline]
    fn get_point(hasher: &dyn NoiseHasher, whole: [isize; 2]) -> [f64; 2] {
        math::add2(get_vec2(hasher.hash(&whole)), math::to_f64_2(whole))
    }

    let cell = math::map2(point, f64::floor);
    let whole = math::to_isize2(cell);
    let frac = math::sub2(point, cell);

    let x_half = frac[0] > 0.5;
    let y_half = frac[1] > 0.5;

    let near = [whole[0] + (x_half as isize), whole[1] + (y_half as isize)];
    let far = [whole[0] + (!x_half as isize), whole[1] + (!y_half as isize)];

    let mut seed_cell = near;
    let seed_point = get_point(hasher, near);
    let mut distance = distance_function(&point, &seed_point);

    let x_distance = (0.5 - frac[0]) * (0.5 - frac[0]); // x-distance squared to center line
    let y_distance = (0.5 - frac[1]) * (0.5 - frac[1]); // y-distance squared to center line

    macro_rules! test_point(
            [$x:expr, $y:expr] => {
                {
                    let cur_point = get_point(hasher, [$x, $y]);
                    let cur_distance = distance_function(&point, &cur_point);
                    if cur_distance < distance {
                        distance = cur_distance;
                        seed_cell = [$x, $y];
                    }
                }
            }
        );

    if x_distance < distance {
        test_point![far[0], near[1]];
    }

    if y_distance < distance {
        test_point![near[0], far[1]];
    }

    if x_distance < distance && y_distance < distance {
        test_point![far[0], far[1]];
    }

    let value = match return_type {
        ReturnType::Distance => distance,
        ReturnType::Value => hasher.hash(&seed_cell) as f64 / 255.0,
    };

    value * 2.0 - 1.0
}

#[rustfmt::skip]
fn get_vec2(index: usize) -> [f64; 2] {
    let length = ((index & 0xF8) >> 3) as f64 * 0.5 / 31.0;
    let diag = length * std::f64::consts::FRAC_1_SQRT_2;

    match index & 0x07 {
        0 => [   diag,    diag],
        1 => [   diag,   -diag],
        2 => [  -diag,    diag],
        3 => [  -diag,   -diag],
        4 => [ length,     0.0],
        5 => [-length,     0.0],
        6 => [    0.0,  length],
        7 => [    0.0, -length],
        _ => unreachable!(),
    }
}

impl NoiseFn<3> for Worley {
    fn get(&self, point: [f64; 3]) -> f64 {
        worley_3d(
            &self.perm_table,
            &self.distance_function,
            self.return_type,
            math::mul3(point, self.frequency),
        )
    }
}

#[inline]
fn worley_3d<F>(
    hasher: &dyn NoiseHasher,
    distance_function: F,
    return_type: ReturnType,
    point: [f64; 3],
) -> f64
where
    F: Fn(&[f64], &[f64]) -> f64,
{
    fn get_point(hasher: &dyn NoiseHasher, whole: [isize; 3]) -> [f64; 3] {
        math::add3(get_vec3(hasher.hash(&whole)), math::to_f64_3(whole))
    }

    let cell = math::map3(point, f64::floor);
    let whole = math::to_isize3(cell);
    let frac = math::sub3(point, cell);

    let x_half = frac[0] > 0.5;
    let y_half = frac[1] > 0.5;
    let z_half = frac[2] > 0.5;

    let near = [
        whole[0] + (x_half as isize),
        whole[1] + (y_half as isize),
        whole[2] + (z_half as isize),
    ];
    let far = [
        whole[0] + (!x_half as isize),
        whole[1] + (!y_half as isize),
        whole[2] + (!z_half as isize),
    ];

    let mut seed_cell = near;
    let seed_point = get_point(hasher, near);
    let mut distance = distance_function(&point, &seed_point);

    let x_distance = (0.5 - frac[0]) * (0.5 - frac[0]); // x-distance squared to center line
    let y_distance = (0.5 - frac[1]) * (0.5 - frac[1]); // y-distance squared to center line
    let z_distance = (0.5 - frac[2]) * (0.5 - frac[2]); // z-distance squared to center line

    macro_rules! test_point(
            [$x:expr, $y:expr, $z:expr] => {
                {
                    let cur_point = get_point(hasher, [$x, $y, $z]);
                    let cur_distance = distance_function(&point, &cur_point);
                    if cur_distance < distance {
                        distance = cur_distance;
                        seed_cell = [$x, $y, $z];
                    }
                }
            }
        );

    if x_distance < distance {
        test_point![far[0], near[1], near[2]];
    }
    if y_distance < distance {
        test_point![near[0], far[1], near[2]];
    }
    if z_distance < distance {
        test_point![near[0], near[1], far[2]];
    }

    if x_distance < distance && y_distance < distance {
        test_point![far[0], far[1], near[2]];
    }
    if x_distance < distance && z_distance < distance {
        test_point![far[0], near[1], far[2]];
    }
    if y_distance < distance && z_distance < distance {
        test_point![near[0], far[1], far[2]];
    }

    if x_distance < distance && y_distance < distance && z_distance < distance {
        test_point![far[0], far[1], far[2]];
    }

    let value = match return_type {
        ReturnType::Distance => distance,
        ReturnType::Value => hasher.hash(&seed_cell) as f64 / 255.0,
    };

    value * 2.0 - 1.0
}

#[rustfmt::skip]
fn get_vec3(index: usize) -> [f64; 3] {
    let length = ((index & 0xE0) >> 5) as f64 * 0.5 / 7.0;
    let diag = length * std::f64::consts::FRAC_1_SQRT_2;

    match index % 18 {
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
        _ => panic!("Attempt to access 3D gradient {} of 18", index % 18),
    }
}

#[allow(clippy::cognitive_complexity)]
impl NoiseFn<4> for Worley {
    fn get(&self, point: [f64; 4]) -> f64 {
        worley_4d(
            &self.perm_table,
            &self.distance_function,
            self.return_type,
            math::mul4(point, self.frequency),
        )
    }
}

#[inline]
#[allow(clippy::cognitive_complexity)]
fn worley_4d<F>(
    hasher: &dyn NoiseHasher,
    distance_function: F,
    return_type: ReturnType,
    point: [f64; 4],
) -> f64
where
    F: Fn(&[f64], &[f64]) -> f64,
{
    fn get_point(hasher: &dyn NoiseHasher, whole: [isize; 4]) -> [f64; 4] {
        math::add4(get_vec4(hasher.hash(&whole)), math::to_f64_4(whole))
    }

    let cell = math::map4(point, f64::floor);
    let whole = math::to_isize4(cell);
    let frac = math::sub4(point, cell);

    let half: Vec<bool> = frac.iter().map(|a| *a > 0.5).collect();

    let near = [
        whole[0] + (half[0] as isize),
        whole[1] + (half[1] as isize),
        whole[2] + (half[2] as isize),
        whole[3] + (half[3] as isize),
    ];
    let far = [
        whole[0] + (!half[0] as isize),
        whole[1] + (!half[0] as isize),
        whole[2] + (!half[0] as isize),
        whole[3] + (!half[0] as isize),
    ];

    let mut seed_cell = near;
    let seed_point = get_point(hasher, near);
    let mut distance = distance_function(&point, &seed_point);

    // get distance squared to center line for each axis
    let center_distance = frac
        .iter()
        .map(|a| (0.5 - a).powf(2.0))
        .collect::<Vec<f64>>();

    macro_rules! test_point(
            [$x:expr, $y:expr, $z:expr, $w:expr] => {
                {
                    let cur_point = get_point(hasher, [$x, $y, $z, $w]);
                    let cur_distance = distance_function(&point, &cur_point);
                    if cur_distance < distance {
                        distance = cur_distance;
                        seed_cell = [$x, $y, $z, $w];
                    }
                }
            }
        );

    if center_distance[0] < distance {
        test_point![far[0], near[1], near[2], near[3]];
    }
    if center_distance[1] < distance {
        test_point![near[0], far[1], near[2], near[3]];
    }
    if center_distance[2] < distance {
        test_point![near[0], near[1], far[2], near[3]];
    }
    if center_distance[3] < distance {
        test_point![near[0], near[1], near[2], far[3]];
    }

    if center_distance[0] < distance && center_distance[1] < distance {
        test_point![far[0], far[1], near[2], near[3]];
    }
    if center_distance[0] < distance && center_distance[2] < distance {
        test_point![far[0], near[1], far[2], near[3]];
    }
    if center_distance[0] < distance && center_distance[3] < distance {
        test_point![far[0], near[1], near[2], far[3]];
    }
    if center_distance[1] < distance && center_distance[2] < distance {
        test_point![near[0], far[1], far[2], near[3]];
    }
    if center_distance[1] < distance && center_distance[3] < distance {
        test_point![near[0], far[1], near[2], far[3]];
    }
    if center_distance[2] < distance && center_distance[3] < distance {
        test_point![near[0], near[1], far[2], far[3]];
    }

    if center_distance[0] < distance
        && center_distance[1] < distance
        && center_distance[2] < distance
    {
        test_point![far[0], far[1], far[2], near[3]];
    }
    if center_distance[0] < distance
        && center_distance[1] < distance
        && center_distance[3] < distance
    {
        test_point![far[0], far[1], near[2], far[3]];
    }
    if center_distance[0] < distance
        && center_distance[2] < distance
        && center_distance[3] < distance
    {
        test_point![far[0], near[1], far[2], far[3]];
    }
    if center_distance[1] < distance
        && center_distance[2] < distance
        && center_distance[3] < distance
    {
        test_point![near[0], far[1], far[2], far[3]];
    }

    if center_distance[0] < distance
        && center_distance[1] < distance
        && center_distance[2] < distance
        && center_distance[3] < distance
    {
        test_point![far[0], far[1], far[2], far[3]];
    }

    let value = match return_type {
        ReturnType::Distance => distance,
        ReturnType::Value => hasher.hash(&seed_cell) as f64 / 255.0,
    };

    value * 2.0 - 1.0
}

#[rustfmt::skip]
fn get_vec4(index: usize) -> [f64; 4] {
    let length = ((index & 0xE0) >> 5) as f64 * 0.5 / 7.0;
    let diag = length * 0.577_350_269_189_625_8;

    match index % 32 {
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
        _ => panic!("Attempt to access 4D gradient {} of 32", index % 32),
    }
}
