use crate::{
    math,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::PermutationTable,
};
use std;

/// Noise function that outputs Worley noise.
#[derive(Clone, Copy, Debug)]
pub struct Worley {
    /// Specifies the range function to use when calculating the boundaries of
    /// the cell.
    pub range_function: RangeFunction,

    /// Determines if the distance from the nearest seed point is applied to
    /// the output value.
    pub enable_range: bool,

    /// Frequency of the seed points.
    pub frequency: f64,

    /// Scale of the random displacement to apply to each cell.
    ///
    /// The noise function assigns each Worley cell a random constant value from
    /// a value noise function. The `displacement` _value_ controls the range
    /// random values to assign to each cell. The range of random values is +/-
    /// the displacement value.
    pub displacement: f64,

    seed: u32,
    perm_table: PermutationTable,
}

impl Worley {
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_RANGEFUNCTION: RangeFunction = RangeFunction::Euclidean;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;
    pub const DEFAULT_DISPLACEMENT: f64 = 1.0;

    pub fn new() -> Self {
        Self {
            perm_table: PermutationTable::new(Self::DEFAULT_SEED),
            seed: Self::DEFAULT_SEED,
            range_function: Self::DEFAULT_RANGEFUNCTION,
            enable_range: false,
            frequency: Self::DEFAULT_FREQUENCY,
            displacement: Self::DEFAULT_DISPLACEMENT,
        }
    }

    /// Sets the range function used by the Worley cells.
    pub fn set_range_function(self, range_function: RangeFunction) -> Self {
        Self {
            range_function,
            ..self
        }
    }

    /// Enables or disables applying the distance from the nearest seed point
    /// to the output value.
    pub fn enable_range(self, enable_range: bool) -> Self {
        Self {
            enable_range,
            ..self
        }
    }

    /// Sets the frequency of the seed points.
    pub fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }

    pub fn set_displacement(self, displacement: f64) -> Self {
        Self {
            displacement,
            ..self
        }
    }
}

impl Default for Worley {
    fn default() -> Self {
        Self::new()
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

/// Set of distance functions that can be used in the Worley noise function.
#[derive(Clone, Copy, Debug)]
pub enum RangeFunction {
    /// The standard linear distance. Expensive to compute because it requires
    /// square root calculations.
    Euclidean,

    /// Same as Euclidean, but without the square root calculations. Distance
    /// results will be smaller, however, but hash patterns will be the same.
    EuclideanSquared,

    /// Measured by only moving in straight lines along the axes. Diagonal
    /// movement is not allowed, which leads to increased distances.
    Manhattan,

    /// Measured by taking the largest distance along any axis as the total
    /// distance. Since this eliminates all but one dimension, it results in
    /// significantly shorter distances and produces regions where the
    /// distances are uniform.
    Chebyshev,

    /// Experimental function where all values are multiplied together and then
    /// added up like a quadratic equation.
    Quadratic,
}

fn calculate_range(range_function: RangeFunction, p1: &[f64], p2: &[f64]) -> f64 {
    match range_function {
        RangeFunction::Euclidean => range_euclidean(p1, p2),
        RangeFunction::EuclideanSquared => range_euclidean_squared(p1, p2),
        RangeFunction::Manhattan => range_manhattan(p1, p2),
        RangeFunction::Chebyshev => range_chebyshev(p1, p2),
        RangeFunction::Quadratic => range_quadratic(p1, p2),
    }
}

fn range_euclidean(p1: &[f64], p2: &[f64]) -> f64 {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| *a - *b)
        .map(|a| a * a)
        .fold(0.0, |acc, x| acc + x)
        .sqrt()
}

fn range_euclidean_squared(p1: &[f64], p2: &[f64]) -> f64 {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| *a - *b)
        .map(|a| a * a)
        .fold(0.0, |acc, x| acc + x)
}

fn range_manhattan(p1: &[f64], p2: &[f64]) -> f64 {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| *a - *b)
        .map(|a| a.abs())
        .fold(0.0, |acc, x| acc + x)
}

fn range_chebyshev(p1: &[f64], p2: &[f64]) -> f64 {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| *a - *b)
        .map(|a| a.abs())
        .fold(std::f64::MIN, |a, b| a.max(b))
}

fn range_quadratic(p1: &[f64], p2: &[f64]) -> f64 {
    let temp: Vec<f64> = p1.iter().zip(p2.iter()).map(|(a, b)| *a - *b).collect();

    let mut result = 0.0;

    for i in &temp {
        for j in &temp {
            result += *i * *j;
        }
    }

    result
}

impl NoiseFn<[f64; 2]> for Worley {
    fn get(&self, point: [f64; 2]) -> f64 {
        fn get_point(perm_table: &PermutationTable, whole: [isize; 2]) -> [f64; 2] {
            math::add2(get_vec2(perm_table.get2(whole)), math::to_f64_2(whole))
        }

        let point = &math::mul2(point, self.frequency);

        let cell = math::map2(*point, f64::floor);
        let whole = math::to_isize2(cell);
        let frac = math::sub2(*point, cell);

        let x_half = frac[0] > 0.5;
        let y_half = frac[1] > 0.5;

        let near = [whole[0] + (x_half as isize), whole[1] + (y_half as isize)];
        let far = [whole[0] + (!x_half as isize), whole[1] + (!y_half as isize)];

        let mut seed_cell = near;
        let seed_point = get_point(&self.perm_table, near);
        let mut range = calculate_range(self.range_function, point, &seed_point);

        let x_range = (0.5 - frac[0]) * (0.5 - frac[0]); // x-distance squared to center line
        let y_range = (0.5 - frac[1]) * (0.5 - frac[1]); // y-distance squared to center line

        macro_rules! test_point(
            [$x:expr, $y:expr] => {
                {
                    let cur_point = get_point(&self.perm_table, [$x, $y]);
                    let cur_range = calculate_range(self.range_function, point, &cur_point);
                    if cur_range < range {
                        range = cur_range;
                        seed_cell = [$x, $y];
                    }
                }
            }
        );

        if x_range < range {
            test_point![far[0], near[1]];
        }

        if y_range < range {
            test_point![near[0], far[1]];
        }

        if x_range < range && y_range < range {
            test_point![far[0], far[1]];
        }

        let value = if self.enable_range {
            range
        } else {
            self.displacement * self.perm_table.get2(seed_cell) as f64 / 255.0
        };

        value * 2.0 - 1.0
    }
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

impl NoiseFn<[f64; 3]> for Worley {
    fn get(&self, point: [f64; 3]) -> f64 {
        fn get_point(perm_table: &PermutationTable, whole: [isize; 3]) -> [f64; 3] {
            math::add3(get_vec3(perm_table.get3(whole)), math::to_f64_3(whole))
        }

        let point = &math::mul3(point, self.frequency);

        let cell = math::map3(*point, f64::floor);
        let whole = math::to_isize3(cell);
        let frac = math::sub3(*point, cell);

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
        let seed_point = get_point(&self.perm_table, near);
        let mut range = calculate_range(self.range_function, point, &seed_point);

        let x_range = (0.5 - frac[0]) * (0.5 - frac[0]); // x-distance squared to center line
        let y_range = (0.5 - frac[1]) * (0.5 - frac[1]); // y-distance squared to center line
        let z_range = (0.5 - frac[2]) * (0.5 - frac[2]); // z-distance squared to center line

        macro_rules! test_point(
            [$x:expr, $y:expr, $z:expr] => {
                {
                    let cur_point = get_point(&self.perm_table, [$x, $y, $z]);
                    let cur_range = calculate_range(self.range_function, point, &cur_point);
                    if cur_range < range {
                        range = cur_range;
                        seed_cell = [$x, $y, $z];
                    }
                }
            }
        );

        if x_range < range {
            test_point![far[0], near[1], near[2]];
        }
        if y_range < range {
            test_point![near[0], far[1], near[2]];
        }
        if z_range < range {
            test_point![near[0], near[1], far[2]];
        }

        if x_range < range && y_range < range {
            test_point![far[0], far[1], near[2]];
        }
        if x_range < range && z_range < range {
            test_point![far[0], near[1], far[2]];
        }
        if y_range < range && z_range < range {
            test_point![near[0], far[1], far[2]];
        }

        if x_range < range && y_range < range && z_range < range {
            test_point![far[0], far[1], far[2]];
        }

        let value = if self.enable_range {
            range
        } else {
            self.displacement * self.perm_table.get3(seed_cell) as f64 / 255.0
        };

        value * 2.0 - 1.0
    }
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

impl NoiseFn<[f64; 4]> for Worley {
    fn get(&self, point: [f64; 4]) -> f64 {
        fn get_point(perm_table: &PermutationTable, whole: [isize; 4]) -> [f64; 4] {
            math::add4(get_vec4(perm_table.get4(whole)), math::to_f64_4(whole))
        }

        let point = &math::mul4(point, self.frequency);

        let cell = math::map4(*point, f64::floor);
        let whole = math::to_isize4(cell);
        let frac = math::sub4(*point, cell);

        let x_half = frac[0] > 0.5;
        let y_half = frac[1] > 0.5;
        let z_half = frac[2] > 0.5;
        let w_half = frac[3] > 0.5;

        let near = [
            whole[0] + (x_half as isize),
            whole[1] + (y_half as isize),
            whole[2] + (z_half as isize),
            whole[3] + (w_half as isize),
        ];
        let far = [
            whole[0] + (!x_half as isize),
            whole[1] + (!y_half as isize),
            whole[2] + (!z_half as isize),
            whole[3] + (!w_half as isize),
        ];

        let mut seed_cell = near;
        let seed_point = get_point(&self.perm_table, near);
        let mut range = calculate_range(self.range_function, point, &seed_point);

        let x_range = (0.5 - frac[0]) * (0.5 - frac[0]); // x-distance squared to center line
        let y_range = (0.5 - frac[1]) * (0.5 - frac[1]); // y-distance squared to center line
        let z_range = (0.5 - frac[2]) * (0.5 - frac[2]); // z-distance squared to center line
        let w_range = (0.5 - frac[3]) * (0.5 - frac[3]); // w-distance squared to center line

        macro_rules! test_point(
            [$x:expr, $y:expr, $z:expr, $w:expr] => {
                {
                    let cur_point = get_point(&self.perm_table, [$x, $y, $z, $w]);
                    let cur_range = calculate_range(self.range_function, point, &cur_point);
                    if cur_range < range {
                        range = cur_range;
                        seed_cell = [$x, $y, $z, $w];
                    }
                }
            }
        );

        if x_range < range {
            test_point![far[0], near[1], near[2], near[3]];
        }
        if y_range < range {
            test_point![near[0], far[1], near[2], near[3]];
        }
        if z_range < range {
            test_point![near[0], near[1], far[2], near[3]];
        }
        if w_range < range {
            test_point![near[0], near[1], near[2], far[3]];
        }

        if x_range < range && y_range < range {
            test_point![far[0], far[1], near[2], near[3]];
        }
        if x_range < range && z_range < range {
            test_point![far[0], near[1], far[2], near[3]];
        }
        if x_range < range && w_range < range {
            test_point![far[0], near[1], near[2], far[3]];
        }
        if y_range < range && z_range < range {
            test_point![near[0], far[1], far[2], near[3]];
        }
        if y_range < range && w_range < range {
            test_point![near[0], far[1], near[2], far[3]];
        }
        if z_range < range && w_range < range {
            test_point![near[0], near[1], far[2], far[3]];
        }

        if x_range < range && y_range < range && z_range < range {
            test_point![far[0], far[1], far[2], near[3]];
        }
        if x_range < range && y_range < range && w_range < range {
            test_point![far[0], far[1], near[2], far[3]];
        }
        if x_range < range && z_range < range && w_range < range {
            test_point![far[0], near[1], far[2], far[3]];
        }
        if y_range < range && z_range < range && w_range < range {
            test_point![near[0], far[1], far[2], far[3]];
        }

        if x_range < range && y_range < range && z_range < range && w_range < range {
            test_point![far[0], far[1], far[2], far[3]];
        }

        let value = if self.enable_range {
            range
        } else {
            self.displacement * self.perm_table.get4(seed_cell) as f64 / 255.0
        };

        value * 2.0 - 1.0
    }
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
