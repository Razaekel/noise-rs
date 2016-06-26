// Copyright 2015 The Noise-rs Developers.
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

use {PermutationTable, math};
use num_traits::Float;

#[inline(always)]
fn get_cell_point2<T: Float>(perm_table: &PermutationTable,
                             cell: math::Point2<T>)
                             -> math::Point2<T> {
    let val = perm_table.get2(math::cast2::<_, i64>(cell));
    math::add2(cell,
               math::mul2(math::cast2([val & 0x0F, (val & 0xF0) >> 4]),
                          math::cast(1.0 / 15.0)))
}

#[inline(always)]
fn get_cell_point3<T: Float>(perm_table: &PermutationTable,
                             cell: math::Point3<T>)
                             -> math::Point3<T> {
    let cell_int = math::cast3::<_, i64>(cell);
    let val1 = perm_table.get3(cell_int);
    let val2 = perm_table.get3([cell_int[0], cell_int[1], cell_int[2] + 128]);
    math::add3(cell,
               math::mul3(math::cast3([val1 & 0x0F, (val1 & 0xF0) >> 4, val2 & 0x0F]),
                          math::cast(1.0 / 15.0)))
}

#[inline(always)]
fn get_cell_point4<T: Float>(perm_table: &PermutationTable,
                             cell: math::Point4<T>)
                             -> math::Point4<T> {
    let cell_int = math::cast4::<_, i64>(cell);
    let val1 = perm_table.get4(cell_int);
    let val2 = perm_table.get4([cell_int[0], cell_int[1], cell_int[2], cell_int[3] + 128]);
    math::add4(cell,
               math::mul4(math::cast4([val1 & 0x0F,
                                       (val1 & 0xF0) >> 4,
                                       val2 & 0x0F,
                                       (val2 & 0xF0) >> 4]),
                          math::cast(1.0 / 15.0)))
}

#[inline(always)]
pub fn range_sqr_euclidian2<T: Float>(p1: math::Point2<T>, p2: math::Point2<T>) -> T {
    let offset = math::sub2(p1, p2);
    math::dot2(offset, offset)
}

#[inline(always)]
pub fn range_sqr_euclidian3<T: Float>(p1: math::Point3<T>, p2: math::Point3<T>) -> T {
    let offset = math::sub3(p1, p2);
    math::dot3(offset, offset)
}

#[inline(always)]
pub fn range_sqr_euclidian4<T: Float>(p1: math::Point4<T>, p2: math::Point4<T>) -> T {
    let offset = math::sub4(p1, p2);
    math::dot4(offset, offset)
}

#[inline(always)]
pub fn range_manhattan2<T: Float>(p1: math::Point2<T>, p2: math::Point2<T>) -> T {
    let offset = math::sub2(p1, p2);
    offset[0].abs() + offset[1].abs()
}

#[inline(always)]
pub fn range_manhattan3<T: Float>(p1: math::Point3<T>, p2: math::Point3<T>) -> T {
    let offset = math::sub3(p1, p2);
    offset[0].abs() + offset[1].abs() + offset[2].abs()
}

#[inline(always)]
pub fn range_manhattan4<T: Float>(p1: math::Point4<T>, p2: math::Point4<T>) -> T {
    let offset = math::sub4(p1, p2);
    offset[0].abs() + offset[1].abs() + offset[2].abs() + offset[3].abs()
}

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get_vec2<T: Float>(index: usize) -> math::Point2<T> {
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

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get_vec3<T: Float>(index: usize) -> math::Point3<T> {
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
        _ => panic!("Attempt to access 3D gradient {} of 12", index % 12),
    }
}

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get_vec4<T: Float>(index: usize) -> math::Point4<T> {
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

#[inline(always)]
fn cell2_seed<T, F>(perm_table: &PermutationTable,
                    point: &math::Point2<T>,
                    range_func: F)
                    -> (math::Point2<i64>, math::Point2<T>, T)
    where T: Float,
          F: Fn(math::Point2<T>, math::Point2<T>) -> T,
{
    #[inline(always)]
    fn get_point<T: Float>(perm_table: &PermutationTable,
                           whole: math::Point2<i64>)
                           -> math::Point2<T> {
        math::add2(get_vec2(perm_table.get2(whole)), math::cast2::<_, T>(whole))
    }

    let half: T = math::cast(0.5);

    let floored = math::map2(*point, T::floor);
    let whole = math::map2(floored, math::cast::<_, i64>);
    let frac = math::sub2(*point, floored);

    let x_half = frac[0] > half;
    let y_half = frac[1] > half;

    let near = [whole[0] + (x_half as i64), whole[1] + (y_half as i64)];
    let far = [whole[0] + (!x_half as i64), whole[1] + (!y_half as i64)];

    let mut seed_cell = near;
    let mut seed_point = get_point(perm_table, near);
    let mut range = range_func(*point, seed_point);

    let x_range = (half - frac[0]) * (half - frac[0]); // x-distance squared to center line
    let y_range = (half - frac[1]) * (half - frac[1]); // y-distance squared to center line

    macro_rules! test_point(
        [$x:expr, $y:expr] => {
            {
                let cur_point = get_point(perm_table, [$x, $y]);
                let cur_range = range_func(*point, cur_point);
                if cur_range < range {
                    range = cur_range;
                    seed_cell = [$x, $y];
                    seed_point = cur_point;
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

    (seed_cell, seed_point, range)
}

#[inline(always)]
fn cell3_seed<T, F>(perm_table: &PermutationTable,
                    point: &math::Point3<T>,
                    range_func: F)
                    -> (math::Point3<i64>, math::Point3<T>, T)
    where T: Float,
          F: Fn(math::Point3<T>, math::Point3<T>) -> T,
{
    #[inline(always)]
    fn get_point<T: Float>(perm_table: &PermutationTable,
                           whole: math::Point3<i64>)
                           -> math::Point3<T> {
        math::add3(get_vec3(perm_table.get3(whole)), math::cast3::<_, T>(whole))
    }

    let half: T = math::cast(0.5);

    let floored = math::map3(*point, T::floor);
    let whole = math::map3(floored, math::cast::<_, i64>);
    let frac = math::sub3(*point, floored);

    let x_half = frac[0] > half;
    let y_half = frac[1] > half;
    let z_half = frac[2] > half;

    let near = [whole[0] + (x_half as i64), whole[1] + (y_half as i64), whole[2] + (z_half as i64)];
    let far =
        [whole[0] + (!x_half as i64), whole[1] + (!y_half as i64), whole[2] + (!z_half as i64)];

    let mut seed_cell = near;
    let mut seed_point = get_point(perm_table, near);
    let mut range = range_func(*point, seed_point);

    let x_range = (half - frac[0]) * (half - frac[0]); // x-distance squared to center line
    let y_range = (half - frac[1]) * (half - frac[1]); // y-distance squared to center line
    let z_range = (half - frac[2]) * (half - frac[2]); // z-distance squared to center line

    macro_rules! test_point(
        [$x:expr, $y:expr, $z:expr] => {
            {
                let cur_point = get_point(perm_table, [$x, $y, $z]);
                let cur_range = range_func(*point, cur_point);
                if cur_range < range {
                    range = cur_range;
                    seed_cell = [$x, $y, $z];
                    seed_point = cur_point;
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

    (seed_cell, seed_point, range)
}

#[inline(always)]
fn cell4_seed<T, F>(perm_table: &PermutationTable,
                    point: &math::Point4<T>,
                    range_func: F)
                    -> (math::Point4<i64>, math::Point4<T>, T)
    where T: Float,
          F: Fn(math::Point4<T>, math::Point4<T>) -> T,
{
    #[inline(always)]
    fn get_point<T: Float>(perm_table: &PermutationTable,
                           whole: math::Point4<i64>)
                           -> math::Point4<T> {
        math::add4(get_vec4(perm_table.get4(whole)), math::cast4::<_, T>(whole))
    }

    let half: T = math::cast(0.5);

    let floored = math::map4(*point, T::floor);
    let whole = math::map4(floored, math::cast::<_, i64>);
    let frac = math::sub4(*point, floored);

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
    let mut seed_point = get_point(perm_table, near);
    let mut range = range_func(*point, seed_point);

    let x_range = (half - frac[0]) * (half - frac[0]); // x-distance squared to center line
    let y_range = (half - frac[1]) * (half - frac[1]); // y-distance squared to center line
    let z_range = (half - frac[2]) * (half - frac[2]); // z-distance squared to center line
    let w_range = (half - frac[3]) * (half - frac[3]); // w-distance squared to center line

    macro_rules! test_point(
        [$x:expr, $y:expr, $z:expr, $w:expr] => {
            {
                let cur_point = get_point(perm_table, [$x, $y, $z, $w]);
                let cur_range = range_func(*point, cur_point);
                if cur_range < range {
                    range = cur_range;
                    seed_cell = [$x, $y, $z, $w];
                    seed_point = cur_point;
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

    (seed_cell, seed_point, range)
}

#[inline(always)]
pub fn cell2_seed_point<T, F>(perm_table: &PermutationTable,
                              point: &math::Point2<T>,
                              range_func: F)
                              -> (math::Point2<T>, T)
    where T: Float,
          F: Fn(math::Point2<T>, math::Point2<T>) -> T,
{
    let (_, seed_point, range) = cell2_seed(perm_table, point, range_func);
    (seed_point, range)
}

#[inline(always)]
pub fn cell2_seed_cell<T, F>(perm_table: &PermutationTable,
                             point: &math::Point2<T>,
                             range_func: F)
                             -> math::Point2<i64>
    where T: Float,
          F: Fn(math::Point2<T>, math::Point2<T>) -> T,
{
    let (cell, _, _) = cell2_seed(perm_table, point, range_func);
    cell
}

#[inline(always)]
pub fn cell3_seed_point<T, F>(perm_table: &PermutationTable,
                              point: &math::Point3<T>,
                              range_func: F)
                              -> (math::Point3<T>, T)
    where T: Float,
          F: Fn(math::Point3<T>, math::Point3<T>) -> T,
{
    let (_, seed_point, range) = cell3_seed(perm_table, point, range_func);
    (seed_point, range)
}

#[inline(always)]
pub fn cell3_seed_cell<T, F>(perm_table: &PermutationTable,
                             point: &math::Point3<T>,
                             range_func: F)
                             -> math::Point3<i64>
    where T: Float,
          F: Fn(math::Point3<T>, math::Point3<T>) -> T,
{
    let (cell, _, _) = cell3_seed(perm_table, point, range_func);
    cell
}

#[inline(always)]
pub fn cell4_seed_point<T, F>(perm_table: &PermutationTable,
                              point: &math::Point4<T>,
                              range_func: F)
                              -> (math::Point4<T>, T)
    where T: Float,
          F: Fn(math::Point4<T>, math::Point4<T>) -> T,
{
    let (_, seed_point, range) = cell4_seed(perm_table, point, range_func);
    (seed_point, range)
}

// These would be faster if we unrolled them like in the 2D version
// Doing that, however, increases the compile time of library users from
// 1 second to 120 seconds which is a poor tradeoff
#[cfg_attr(rustfmt, rustfmt_skip)]
static CELL3_SEARCH_ORDER: [math::Point3<isize>; 26] = [
    [-1,  0,  0], [ 1,  0,  0], [ 0, -1,  0], [ 0,  1,  0], [ 0,  0, -1],
    [ 0,  0,  1], [-1, -1,  0], [-1,  1,  0], [ 1, -1,  0], [ 1,  1,  0],
    [-1,  0, -1], [-1,  0,  1], [ 1,  0, -1], [ 1,  0,  1], [ 0, -1, -1],
    [ 0, -1,  1], [ 0,  1, -1], [ 0,  1,  1], [-1, -1, -1], [-1, -1,  1],
    [-1,  1, -1], [-1,  1,  1], [ 1, -1, -1], [ 1, -1,  1], [ 1,  1, -1],
    [ 1,  1,  1],
];

#[cfg_attr(rustfmt, rustfmt_skip)]
static CELL4_SEARCH_ORDER: [math::Point4<isize>; 80] = [
    [-1,  0,  0,  0], [ 1,  0,  0,  0], [ 0, -1,  0,  0], [ 0,  1,  0,  0],
    [ 0,  0, -1,  0], [ 0,  0,  1,  0], [ 0,  0,  0, -1], [ 0,  0,  0,  1],
    [-1, -1,  0,  0], [-1,  1,  0,  0], [ 1, -1,  0,  0], [ 1,  1,  0,  0],
    [-1,  0, -1,  0], [-1,  0,  1,  0], [ 1,  0, -1,  0], [ 1,  0,  1,  0],
    [-1,  0,  0, -1], [-1,  0,  0,  1], [ 1,  0,  0, -1], [ 1,  0,  0,  1],
    [ 0, -1, -1,  0], [ 0, -1,  1,  0], [ 0,  1, -1,  0], [ 0,  1,  1,  0],
    [ 0, -1,  0, -1], [ 0, -1,  0,  1], [ 0,  1,  0, -1], [ 0,  1,  0,  1],
    [ 0,  0, -1, -1], [ 0,  0, -1,  1], [ 0,  0,  1, -1], [ 0,  0,  1,  1],
    [-1, -1, -1,  0], [-1, -1,  1,  0], [-1,  1, -1,  0], [-1,  1,  1,  0],
    [ 1, -1, -1,  0], [ 1, -1,  1,  0], [ 1,  1, -1,  0], [ 1,  1,  1,  0],
    [-1, -1,  0, -1], [-1, -1,  0,  1], [-1,  1,  0, -1], [-1,  1,  0,  1],
    [ 1, -1,  0, -1], [ 1, -1,  0,  1], [ 1,  1,  0, -1], [ 1,  1,  0,  1],
    [-1,  0, -1, -1], [-1,  0, -1,  1], [-1,  0,  1, -1], [-1,  0,  1,  1],
    [ 1,  0, -1, -1], [ 1,  0, -1,  1], [ 1,  0,  1, -1], [ 1,  0,  1,  1],
    [ 0, -1, -1, -1], [ 0, -1, -1,  1], [ 0, -1,  1, -1], [ 0, -1,  1,  1],
    [ 0,  1, -1, -1], [ 0,  1, -1,  1], [ 0,  1,  1, -1], [ 0,  1,  1,  1],
    [-1, -1, -1, -1], [-1, -1, -1,  1], [-1, -1,  1, -1], [-1, -1,  1,  1],
    [-1,  1, -1, -1], [-1,  1, -1,  1], [-1,  1,  1, -1], [-1,  1,  1,  1],
    [ 1, -1, -1, -1], [ 1, -1, -1,  1], [ 1, -1,  1, -1], [ 1, -1,  1,  1],
    [ 1,  1, -1, -1], [ 1,  1, -1,  1], [ 1,  1,  1, -1], [ 1,  1,  1,  1],
];

#[inline(always)]
pub fn cell4_seed_cell<T, F>(perm_table: &PermutationTable,
                             point: &math::Point4<T>,
                             range_func: F)
                             -> math::Point4<i64>
    where T: Float,
          F: Fn(math::Point4<T>, math::Point4<T>) -> T,
{
    let (cell, _, _) = cell4_seed(perm_table, point, range_func);
    cell
}

#[inline(always)]
pub fn cell2_seed_2_points<T, F>(perm_table: &PermutationTable,
                                 point: &math::Point2<T>,
                                 range_func: F)
                                 -> (math::Point2<T>, T, math::Point2<T>, T)
    where T: Float,
          F: Fn(math::Point2<T>, math::Point2<T>) -> T,
{
    let one = T::one();
    let zero = T::zero();

    let cell = math::map2(*point, T::floor);
    let frac = math::sub2(*point, cell);

    let mut seed_point0 = get_cell_point2(perm_table, cell);
    let mut seed_point1 = [one, one];
    let mut range0 = range_func(*point, seed_point0);
    let mut range1 = T::max_value();

    // Distance squared for the previous, current, and next cells in each dimension
    let dx2 = [frac[0] * frac[0], zero, (one - frac[0]) * (one - frac[0])];
    let dy2 = [frac[1] * frac[1], zero, (one - frac[1]) * (one - frac[1])];

    macro_rules! test_point(
        [$x:expr, $y:expr] => {
            {
                let x_range = dx2[($x + 1) as usize];
                let y_range = dy2[($y + 1) as usize];

                if x_range + y_range < range1 {
                    let cur_point = get_cell_point2(perm_table, math::add2(cell,
                        math::cast2([$x, $y])));
                    let cur_range = range_func(*point, cur_point);
                    if cur_range < range0 {
                        range1 = range0;
                        seed_point1 = seed_point0;
                        range0 = cur_range;
                        seed_point0 = cur_point;
                    } else if cur_range < range1 {
                        range1 = cur_range;
                        seed_point1 = cur_point;
                    }
                }
            }
        }
    );

    // Check four facing cells
    test_point![-1,  0];
    test_point![ 1,  0];
    test_point![ 0, -1];
    test_point![ 0,  1];

    // Check four corner cells
    test_point![-1, -1];
    test_point![-1,  1];
    test_point![ 1, -1];
    test_point![ 1,  1];

    (seed_point0, range0, seed_point1, range1)
}

#[inline(always)]
pub fn cell3_seed_2_points<T, F>(perm_table: &PermutationTable,
                                 point: &math::Point3<T>,
                                 range_func: F)
                                 -> (math::Point3<T>, T, math::Point3<T>, T)
    where T: Float,
          F: Fn(math::Point3<T>, math::Point3<T>) -> T,
{
    let one = T::one();
    let zero = T::zero();

    let cell = math::map3(*point, T::floor);
    let frac = math::sub3(*point, cell);

    let mut seed_point0 = get_cell_point3(perm_table, cell);
    let mut seed_point1 = [one, one, one];
    let mut range0 = range_func(*point, seed_point0);
    let mut range1 = T::max_value();

    // Distance squared for the previous, current, and next cells in each dimension
    let dx2 = [frac[0] * frac[0], zero, (one - frac[0]) * (one - frac[0])];
    let dy2 = [frac[1] * frac[1], zero, (one - frac[1]) * (one - frac[1])];
    let dz2 = [frac[2] * frac[2], zero, (one - frac[2]) * (one - frac[2])];

    for offset in CELL3_SEARCH_ORDER.iter() {
        let x_range = dx2[(offset[0] + 1) as usize];
        let y_range = dy2[(offset[1] + 1) as usize];
        let z_range = dz2[(offset[2] + 1) as usize];

        if x_range + y_range + z_range < range1 {
            let cur_point = get_cell_point3(perm_table, math::add3(cell, math::cast3(*offset)));
            let cur_range = range_func(*point, cur_point);
            if cur_range < range0 {
                range1 = range0;
                seed_point1 = seed_point0;
                range0 = cur_range;
                seed_point0 = cur_point;
            } else if cur_range < range1 {
                range1 = cur_range;
                seed_point1 = cur_point;
            }
        }
    }

    (seed_point0, range0, seed_point1, range1)
}

#[inline(always)]
pub fn cell4_seed_2_points<T, F>(perm_table: &PermutationTable,
                                 point: &math::Point4<T>,
                                 range_func: F)
                                 -> (math::Point4<T>, T, math::Point4<T>, T)
    where T: Float,
          F: Fn(math::Point4<T>, math::Point4<T>) -> T,
{
    let one = T::one();
    let zero = T::zero();

    let cell = math::map4(*point, T::floor);
    let frac = math::sub4(*point, cell);

    let mut seed_point0 = get_cell_point4(perm_table, cell);
    let mut seed_point1 = [one, one, one, one];
    let mut range0 = range_func(*point, seed_point0);
    let mut range1 = T::max_value();

    // Distance squared for the previous, current, and next cells in each dimension
    let dx2 = [frac[0] * frac[0], zero, (one - frac[0]) * (one - frac[0])];
    let dy2 = [frac[1] * frac[1], zero, (one - frac[1]) * (one - frac[1])];
    let dz2 = [frac[2] * frac[2], zero, (one - frac[2]) * (one - frac[2])];
    let dw2 = [frac[3] * frac[3], zero, (one - frac[3]) * (one - frac[3])];

    for offset in CELL4_SEARCH_ORDER.iter() {
        let x_range = dx2[(offset[0] + 1) as usize];
        let y_range = dy2[(offset[1] + 1) as usize];
        let z_range = dz2[(offset[2] + 1) as usize];
        let w_range = dw2[(offset[3] + 1) as usize];

        if x_range + y_range + z_range + w_range < range1 {
            let cur_point = get_cell_point4(perm_table, math::add4(cell, math::cast4(*offset)));
            let cur_range = range_func(*point, cur_point);
            if cur_range < range0 {
                range1 = range0;
                seed_point1 = seed_point0;
                range0 = cur_range;
                seed_point0 = cur_point;
            } else if cur_range < range1 {
                range1 = cur_range;
                seed_point1 = cur_point;
            }
        }
    }

    (seed_point0, range0, seed_point1, range1)
}

pub fn cell2_range<T: Float>(perm_table: &PermutationTable, point: &math::Point2<T>) -> T {
    let (_, range) = cell2_seed_point(perm_table, point, range_sqr_euclidian2);
    range
}

pub fn cell3_range<T: Float>(perm_table: &PermutationTable, point: &math::Point3<T>) -> T {
    let (_, range) = cell3_seed_point(perm_table, point, range_sqr_euclidian3);
    range
}

pub fn cell4_range<T: Float>(perm_table: &PermutationTable, point: &math::Point4<T>) -> T {
    let (_, range) = cell4_seed_point(perm_table, point, range_sqr_euclidian4);
    range
}

pub fn cell2_range_inv<T: Float>(perm_table: &PermutationTable, point: &math::Point2<T>) -> T {
    let (_, range1, _, range2) = cell2_seed_2_points(perm_table, point, range_sqr_euclidian2);
    range2 - range1
}

pub fn cell3_range_inv<T: Float>(perm_table: &PermutationTable, point: &math::Point3<T>) -> T {
    let (_, range1, _, range2) = cell3_seed_2_points(perm_table, point, range_sqr_euclidian3);
    range2 - range1
}

pub fn cell4_range_inv<T: Float>(perm_table: &PermutationTable, point: &math::Point4<T>) -> T {
    let (_, range1, _, range2) = cell4_seed_2_points(perm_table, point, range_sqr_euclidian4);
    range2 - range1
}

pub fn cell2_value<T: Float>(perm_table: &PermutationTable, point: &math::Point2<T>) -> T {
    let cell = cell2_seed_cell(perm_table, point, range_sqr_euclidian2);
    math::cast::<_, T>(perm_table.get2(cell)) * math::cast(1.0 / 255.0)
}

pub fn cell3_value<T: Float>(perm_table: &PermutationTable, point: &math::Point3<T>) -> T {
    let cell = cell3_seed_cell(perm_table, point, range_sqr_euclidian3);
    math::cast::<_, T>(perm_table.get3(cell)) * math::cast(1.0 / 255.0)
}

pub fn cell4_value<T: Float>(perm_table: &PermutationTable, point: &math::Point4<T>) -> T {
    let cell = cell4_seed_cell(perm_table, point, range_sqr_euclidian4);
    math::cast::<_, T>(perm_table.get4(cell)) * math::cast(1.0 / 255.0)
}

pub fn cell2_manhattan<T: Float>(perm_table: &PermutationTable, point: &math::Point2<T>) -> T {
    let (_, range) = cell2_seed_point(perm_table, point, range_manhattan2);
    range
}

pub fn cell3_manhattan<T: Float>(perm_table: &PermutationTable, point: &math::Point3<T>) -> T {
    let (_, range) = cell3_seed_point(perm_table, point, range_manhattan3);
    range
}

pub fn cell4_manhattan<T: Float>(perm_table: &PermutationTable, point: &math::Point4<T>) -> T {
    let (_, range) = cell4_seed_point(perm_table, point, range_manhattan4);
    range
}

pub fn cell2_manhattan_inv<T: Float>(perm_table: &PermutationTable, point: &math::Point2<T>) -> T {
    let (_, range1, _, range2) = cell2_seed_2_points(perm_table, point, range_manhattan2);
    range2 - range1
}

pub fn cell3_manhattan_inv<T: Float>(perm_table: &PermutationTable, point: &math::Point3<T>) -> T {
    let (_, range1, _, range2) = cell3_seed_2_points(perm_table, point, range_manhattan3);
    range2 - range1
}

pub fn cell4_manhattan_inv<T: Float>(perm_table: &PermutationTable, point: &math::Point4<T>) -> T {
    let (_, range1, _, range2) = cell4_seed_2_points(perm_table, point, range_manhattan4);
    range2 - range1
}

pub fn cell2_manhattan_value<T: Float>(perm_table: &PermutationTable,
                                       point: &math::Point2<T>)
                                       -> T {
    let cell = cell2_seed_cell(perm_table, point, range_manhattan2);
    math::cast::<_, T>(perm_table.get2(cell)) * math::cast(1.0 / 255.0)
}

pub fn cell3_manhattan_value<T: Float>(perm_table: &PermutationTable,
                                       point: &math::Point3<T>)
                                       -> T {
    let cell = cell3_seed_cell(perm_table, point, range_manhattan3);
    math::cast::<_, T>(perm_table.get3(cell)) * math::cast(1.0 / 255.0)
}

pub fn cell4_manhattan_value<T: Float>(perm_table: &PermutationTable,
                                       point: &math::Point4<T>)
                                       -> T {
    let cell = cell4_seed_cell(perm_table, point, range_manhattan4);
    math::cast::<_, T>(perm_table.get4(cell)) * math::cast(1.0 / 255.0)
}
