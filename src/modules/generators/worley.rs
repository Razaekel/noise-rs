// Copyright 2016 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num_traits::Float;
use {PermutationTable, math};
use math::{Point2, Point3, Point4};
use modules::{NoiseModule, Seedable};

/// Default noise seed for the Worley noise module.
pub const DEFAULT_WORLEY_SEED: usize = 0;
/// Default RangeFunction for the Worley noise module.
pub const DEFAULT_WORLEY_RANGEFUNCTION: RangeFunction = RangeFunction::Euclidean;
/// Default frequency for the Worley noise module.
pub const DEFAULT_WORLEY_FREQUENCY: f32 = 1.0;
/// Default displacement for the Worley noise module.
pub const DEFAULT_WORLEY_DISPLACEMENT: f32 = 1.0;

/// Noise module that outputs 2/3/4-dimensional Worley noise.
#[derive(Clone, Copy, Debug)]
pub struct Worley<T> {
    perm_table: PermutationTable,

    /// Seed.
    pub seed: usize,

    /// Specifies the range function to use when calculating the boundaries of
    /// the cell.
    pub range_function: RangeFunction,

    /// Determines if the distance from the nearest seed point is applied to
    /// the output value.
    pub enable_range: bool,

    /// Frequency of the seed points.
    pub frequency: T,

    /// Scale of the random displacement to apply to each cell.
    ///
    /// The noise module assigns each Worley cell a random constant value from
    /// a value noise function. The `displacement` _value_ controls the range
    /// random values to assign to each cell. The range of random values is +/-
    /// the displacement value.
    pub displacement: T,
}

impl<T> Worley<T>
    where T: Float,
{
    pub fn new() -> Worley<T> {
        Worley {
            perm_table: PermutationTable::new(DEFAULT_WORLEY_SEED as u32),
            seed: DEFAULT_WORLEY_SEED,
            range_function: DEFAULT_WORLEY_RANGEFUNCTION,
            enable_range: false,
            frequency: math::cast(DEFAULT_WORLEY_FREQUENCY),
            displacement: math::cast(DEFAULT_WORLEY_DISPLACEMENT),
        }
    }

    /// Sets the range function used by the Worley cells.
    pub fn set_range_function(self, range_function: RangeFunction) -> Worley<T> {
        Worley { range_function: range_function, ..self }
    }

    /// Enables or disables applying the distance from the nearest seed point
    /// to the output value.
    pub fn enable_range(self, enable_range: bool) -> Worley<T> {
        Worley { enable_range: enable_range, ..self }
    }

    /// Sets the frequency of the seed points.
    pub fn set_frequency(self, frequency: T) -> Worley<T> {
        Worley { frequency: frequency, ..self }
    }

    pub fn set_displacement(self, displacement: T) -> Worley<T> {
        Worley { displacement: displacement, ..self }
    }
}

impl<T> Seedable for Worley<T> {
    /// Sets the seed value used by the Worley cells.
    fn set_seed(self, seed: usize) -> Worley<T> {
        Worley {
            perm_table: PermutationTable::new(seed as u32),
            seed: seed,
            ..self
        }
    }
}

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

fn calculate_range<T: Float>(range_function: RangeFunction, p1: &[T], p2: &[T]) -> T {
    match range_function {
        RangeFunction::Euclidean => range_euclidean(p1, p2),
        RangeFunction::EuclideanSquared => range_euclidean_squared(p1, p2),
        RangeFunction::Manhattan => range_manhattan(p1, p2),
        RangeFunction::Chebyshev => range_chebyshev(p1, p2),
        RangeFunction::Quadratic => range_quadratic(p1, p2),
    }
}

fn range_euclidean<T: Float>(p1: &[T], p2: &[T]) -> T {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| *a - *b)
        .map(|a| a * a)
        .fold(T::zero(), |acc, x| acc + x)
        .sqrt()
}

fn range_euclidean_squared<T: Float>(p1: &[T], p2: &[T]) -> T {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| *a - *b)
        .map(|a| a * a)
        .fold(T::zero(), |acc, x| acc + x)
}

fn range_manhattan<T: Float>(p1: &[T], p2: &[T]) -> T {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| *a - *b)
        .map(|a| a.abs())
        .fold(T::zero(), |acc, x| acc + x)
}

fn range_chebyshev<T: Float>(p1: &[T], p2: &[T]) -> T {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| *a - *b)
        .map(|a| a.abs())
        .fold(T::min_value(), |a, b| a.max(b))
}

fn range_quadratic<T: Float>(p1: &[T], p2: &[T]) -> T {
    let temp: Vec<T> = p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| *a - *b)
        .collect();

    let length = temp.len();
    let mut result = T::zero();

    for i in 0..length {
        for j in 0..length {
            result = result + (temp[i] * temp[j]);
        }
    }

    result
}

impl<T: Float> NoiseModule<Point2<T>> for Worley<T> {
    type Output = T;

    fn get(&self, point: Point2<T>) -> T {
        #[inline(always)]
        fn get_point<T: Float>(perm_table: &PermutationTable, whole: Point2<i64>) -> Point2<T> {
            math::add2(get_vec2(perm_table.get2(whole)), math::cast2::<_, T>(whole))
        }

        let half: T = math::cast(0.5);

        let point = &math::mul2(point, self.frequency);

        let cell = math::map2(*point, T::floor);
        let whole = math::map2(cell, math::cast::<_, i64>);
        let frac = math::sub2(*point, cell);

        let x_half = frac[0] > half;
        let y_half = frac[1] > half;

        let near = [whole[0] + (x_half as i64), whole[1] + (y_half as i64)];
        let far = [whole[0] + (!x_half as i64), whole[1] + (!y_half as i64)];

        let mut seed_cell = near;
        let seed_point = get_point(&self.perm_table, near);
        let mut range = calculate_range(self.range_function, point, &seed_point);

        let x_range = (half - frac[0]) * (half - frac[0]); // x-distance squared to center line
        let y_range = (half - frac[1]) * (half - frac[1]); // y-distance squared to center line

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

        let mut value = T::zero();

        if self.enable_range {
            value = range;
        }

        (value +
         (self.displacement * math::cast::<_, T>(self.perm_table.get2(seed_cell)) *
          math::cast(1.0 / 255.0))) * math::cast(2.0) - T::one()
    }
}

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn get_vec2<T: Float>(index: usize) -> Point2<T> {
    let length = math::cast::<_, T>((index & 0xF8) >> 3) * math::cast(0.5 / 31.0);
    let diag = length * math::cast(0.70710678118);
    let one = length;
    let zero = T::zero();
    match index & 0x07 {
        0 => [ diag,  diag],
        1 => [ diag, -diag],
        2 => [-diag,  diag],
        3 => [-diag, -diag],
        4 => [ one,   zero],
        5 => [-one,   zero],
        6 => [ zero,  one],
        7 => [ zero, -one],
        _ => unreachable!(),
    }
}

impl<T: Float> NoiseModule<Point3<T>> for Worley<T> {
    type Output = T;

    fn get(&self, point: Point3<T>) -> T {
        #[inline(always)]
        fn get_point<T: Float>(perm_table: &PermutationTable,
                               whole: math::Point3<i64>)
                               -> Point3<T> {
            math::add3(get_vec3(perm_table.get3(whole)), math::cast3::<_, T>(whole))
        }

        let half: T = math::cast(0.5);

        let point = &math::mul3(point, self.frequency);

        let cell = math::map3(*point, T::floor);
        let whole = math::map3(cell, math::cast::<_, i64>);
        let frac = math::sub3(*point, cell);

        let x_half = frac[0] > half;
        let y_half = frac[1] > half;
        let z_half = frac[2] > half;

        let near =
            [whole[0] + (x_half as i64), whole[1] + (y_half as i64), whole[2] + (z_half as i64)];
        let far =
            [whole[0] + (!x_half as i64), whole[1] + (!y_half as i64), whole[2] + (!z_half as i64)];

        let mut seed_cell = near;
        let seed_point = get_point(&self.perm_table, near);
        let mut range = calculate_range(self.range_function, point, &seed_point);

        let x_range = (half - frac[0]) * (half - frac[0]); // x-distance squared to center line
        let y_range = (half - frac[1]) * (half - frac[1]); // y-distance squared to center line
        let z_range = (half - frac[2]) * (half - frac[2]); // z-distance squared to center line

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

        let mut value = T::zero();

        if self.enable_range {
            value = range;
        }

        value +
        (self.displacement * math::cast::<_, T>(self.perm_table.get3(seed_cell)) *
         math::cast(1.0 / 255.0) * math::cast(2.0) - T::one())
    }
}

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get_vec3<T: Float>(index: usize) -> Point3<T> {
    let length = math::cast::<_, T>((index & 0xE0) >> 5) * math::cast(0.5 / 7.0);
    let diag = length * math::cast(0.70710678118f32);
    let one = length;
    let zero = T::zero();
    match index % 18 {
        0  => [ diag,  diag,  zero],
        1  => [ diag, -diag,  zero],
        2  => [-diag,  diag,  zero],
        3  => [-diag, -diag,  zero],
        4  => [ diag,  zero,  diag],
        5  => [ diag,  zero, -diag],
        6  => [-diag,  zero,  diag],
        7  => [-diag,  zero, -diag],
        8  => [ zero,  diag,  diag],
        9  => [ zero,  diag, -diag],
        10 => [ zero, -diag,  diag],
        11 => [ zero, -diag, -diag],
        12 => [ one,   zero,  zero],
        13 => [ zero,  one,   zero],
        14 => [ zero,  zero,  one],
        15 => [-one,   zero,  zero],
        16 => [ zero, -one,   zero],
        17 => [ zero,  zero, -one],
        _ => panic!("Attempt to access 3D gradient {} of 18", index % 18),
    }
}

impl<T: Float> NoiseModule<Point4<T>> for Worley<T> {
    type Output = T;

    fn get(&self, point: Point4<T>) -> T {
        #[inline(always)]
        fn get_point<T: Float>(perm_table: &PermutationTable, whole: Point4<i64>) -> Point4<T> {
            math::add4(get_vec4(perm_table.get4(whole)), math::cast4::<_, T>(whole))
        }

        let half: T = math::cast(0.5);

        let point = &math::mul4(point, self.frequency);

        let cell = math::map4(*point, T::floor);
        let whole = math::map4(cell, math::cast::<_, i64>);
        let frac = math::sub4(*point, cell);

        let x_half = frac[0] > half;
        let y_half = frac[1] > half;
        let z_half = frac[2] > half;
        let w_half = frac[3] > half;

        let near = [whole[0] + (x_half as i64),
                    whole[1] + (y_half as i64),
                    whole[2] + (z_half as i64),
                    whole[3] + (w_half as i64)];
        let far = [whole[0] + (!x_half as i64),
                   whole[1] + (!y_half as i64),
                   whole[2] + (!z_half as i64),
                   whole[3] + (!w_half as i64)];

        let mut seed_cell = near;
        let seed_point = get_point(&self.perm_table, near);
        let mut range = calculate_range(self.range_function, point, &seed_point);

        let x_range = (half - frac[0]) * (half - frac[0]); // x-distance squared to center line
        let y_range = (half - frac[1]) * (half - frac[1]); // y-distance squared to center line
        let z_range = (half - frac[2]) * (half - frac[2]); // z-distance squared to center line
        let w_range = (half - frac[3]) * (half - frac[3]); // w-distance squared to center line

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

        let mut value = T::zero();

        if self.enable_range {
            value = range;
        }

        value +
        (self.displacement * math::cast::<_, T>(self.perm_table.get4(seed_cell)) *
         math::cast(1.0 / 255.0) * math::cast(2.0) - T::one())
    }
}

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get_vec4<T: Float>(index: usize) -> Point4<T> {
    let length = math::cast::<_, T>((index & 0xE0) >> 5) * math::cast(0.5 / 7.0);
    let diag = length * math::cast(0.57735026919);
    let zero = T::zero();
    match index % 32 {
        0  => [ diag,  diag,  diag,  zero],
        1  => [ diag, -diag,  diag,  zero],
        2  => [-diag,  diag,  diag,  zero],
        3  => [-diag, -diag,  diag,  zero],
        4  => [ diag,  diag, -diag,  zero],
        5  => [ diag, -diag, -diag,  zero],
        6  => [-diag,  diag, -diag,  zero],
        7  => [-diag, -diag, -diag,  zero],
        8  => [ diag,  diag,  zero,  diag],
        9  => [ diag, -diag,  zero,  diag],
        10 => [-diag,  diag,  zero,  diag],
        11 => [-diag, -diag,  zero,  diag],
        12 => [ diag,  diag,  zero, -diag],
        13 => [ diag, -diag,  zero, -diag],
        14 => [-diag,  diag,  zero, -diag],
        15 => [-diag, -diag,  zero, -diag],
        16 => [ diag,  zero,  diag,  diag],
        17 => [ diag,  zero, -diag,  diag],
        18 => [-diag,  zero,  diag,  diag],
        19 => [-diag,  zero, -diag,  diag],
        20 => [ diag,  zero,  diag, -diag],
        21 => [ diag,  zero, -diag, -diag],
        22 => [-diag,  zero,  diag, -diag],
        23 => [-diag,  zero, -diag, -diag],
        24 => [ zero,  diag,  diag,  diag],
        25 => [ zero,  diag, -diag,  diag],
        26 => [ zero, -diag,  diag,  diag],
        27 => [ zero, -diag, -diag,  diag],
        28 => [ zero,  diag,  diag, -diag],
        29 => [ zero,  diag, -diag, -diag],
        30 => [ zero, -diag,  diag, -diag],
        31 => [ zero, -diag, -diag, -diag],
        _ => panic!("Attempt to access 4D gradient {} of 32", index % 32),
    }
}
