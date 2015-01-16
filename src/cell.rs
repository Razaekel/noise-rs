// Copyright 2015 The noise-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

use {math, Seed};
use std::num::Float;

#[inline(always)]
fn get_cell2<T: Float>(point: math::Point2<T>) -> math::Point2<T> {
    [point[0].floor(), point[1].floor()]
}

#[inline(always)]
fn get_cell3<T: Float>(point: math::Point3<T>) -> math::Point3<T> {
    [point[0].floor(), point[1].floor(), point[2].floor()]
}

#[inline(always)]
fn get_cell4<T: Float>(point: math::Point4<T>) -> math::Point4<T> {
    [point[0].floor(), point[1].floor(), point[2].floor(), point[3].floor()]
}

#[inline(always)]
fn get_cell_point2<T: Float>(seed: &Seed, cell: math::Point2<T>) -> math::Point2<T> {
    let val = seed.get2(math::cast2::<_,i64>(cell));
    math::add2(cell, math::mul2(math::cast2([val & 0x0F, val & 0xF0 >> 4]), math::cast(1.0 / 15.0)))
}

#[inline(always)]
fn get_cell_point3<T: Float>(seed: &Seed, cell: math::Point3<T>) -> math::Point3<T> {
    let cell_int = math::cast3::<_,i64>(cell);
    let val1 = seed.get3(cell_int);
    let val2 = seed.get3([cell_int[0], cell_int[1], cell_int[2] + 128]);
    math::add3(cell, math::mul3(math::cast3([val1 & 0x0F, val1 & 0xF0 >> 4, val2 & 0x0F]), math::cast(1.0 / 15.0)))
}

#[inline(always)]
fn get_cell_point4<T: Float>(seed: &Seed, cell: math::Point4<T>) -> math::Point4<T> {
    let cell_int = math::cast4::<_,i64>(cell);
    let val1 = seed.get4(cell_int);
    let val2 = seed.get4([cell_int[0], cell_int[1], cell_int[2], cell_int[3] + 128]);
    math::add4(cell, math::mul4(math::cast4([val1 & 0x0F, val1 & 0xF0 >> 4, val2 & 0x0F, val2 & 0xF0 >> 4]), math::cast(1.0 / 15.0)))
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
pub fn cell2_seed_point<T, F>(seed: &Seed, point: &math::Point2<T>, range_func: F) -> (math::Point2<T>, T)
    where T: Float, F: Fn(math::Point2<T>, math::Point2<T>) -> T
{
    let zero: T = math::cast(0);

    let cell = get_cell2(*point);
    let mut range: T = Float::max_value();
    let mut seed_point: math::Point2<T> = [zero, zero];

    for x_offset in -1..2 {
        for y_offset in -1..2 {
            let cur_seed_point = get_cell_point2(seed, math::add2(cell, math::cast2([x_offset, y_offset])));
            let cur_range = range_func(*point, cur_seed_point);
            if cur_range < range {
                range = cur_range;
                seed_point = cur_seed_point;
            }
        }
    }

    (seed_point, range)
}

#[inline(always)]
pub fn cell3_seed_point<T, F>(seed: &Seed, point: &math::Point3<T>, range_func: F) -> (math::Point3<T>, T)
    where T: Float, F: Fn(math::Point3<T>, math::Point3<T>) -> T
{
    let zero: T = math::cast(0);

    let cell = get_cell3(*point);
    let mut range: T = Float::max_value();
    let mut seed_point: math::Point3<T> = [zero, zero, zero];

    for x_offset in -1..2 {
        for y_offset in -1..2 {
            for z_offset in -1..2 {
                let cur_seed_point = get_cell_point3(seed, math::add3(cell, math::cast3([x_offset, y_offset, z_offset])));
                let cur_range = range_func(*point, cur_seed_point);
                if cur_range < range {
                    range = cur_range;
                    seed_point = cur_seed_point;
                }
            }
        }
    }

    (seed_point, range)
}

#[inline(always)]
pub fn cell4_seed_point<T, F>(seed: &Seed, point: &math::Point4<T>, range_func: F) -> (math::Point4<T>, T)
    where T: Float, F: Fn(math::Point4<T>, math::Point4<T>) -> T
{
    let zero: T = math::cast(0);

    let cell = get_cell4(*point);
    let mut range: T = Float::max_value();
    let mut seed_point: math::Point4<T> = [zero, zero, zero, zero];

    for x_offset in -1..2 {
        for y_offset in -1..2 {
            for z_offset in -1..2 {
                for w_offset in -1..2 {
                    let cur_seed_point = get_cell_point4(seed, math::add4(cell, math::cast4([x_offset, y_offset, z_offset, w_offset])));
                    let cur_range = range_func(*point, cur_seed_point);
                    if cur_range < range {
                        range = cur_range;
                        seed_point = cur_seed_point;
                    }
                }
            }
        }
    }

    (seed_point, range)
}

#[inline(always)]
pub fn cell2_seed_2_points<T, F>(seed: &Seed, point: &math::Point2<T>, range_func: F) -> (math::Point2<T>, T, math::Point2<T>, T)
    where T: Float, F: Fn(math::Point2<T>, math::Point2<T>) -> T
{
    let zero: T = math::cast(0);

    let cell = get_cell2(*point);
    let mut range1: T = Float::max_value();
    let mut range2: T = Float::max_value();
    let mut seed_point1: math::Point2<T> = [zero, zero];
    let mut seed_point2: math::Point2<T> = [zero, zero];

    for x_offset in -1..2 {
        for y_offset in -1..2 {
            let cur_seed_point = get_cell_point2(seed, math::add2(cell, math::cast2([x_offset, y_offset])));
            let cur_range = range_func(*point, cur_seed_point);
            if cur_range < range1 {
                range2 = range1;
                seed_point2 = seed_point1;
                range1 = cur_range;
                seed_point1 = cur_seed_point;
            } else if cur_range < range2 {
                range2 = cur_range;
                seed_point2 = cur_seed_point;
            }
        }
    }

    (seed_point1, range1, seed_point2, range2)
}

#[inline(always)]
pub fn cell3_seed_2_points<T, F>(seed: &Seed, point: &math::Point3<T>, range_func: F) -> (math::Point3<T>, T, math::Point3<T>, T)
    where T: Float, F: Fn(math::Point3<T>, math::Point3<T>) -> T
{
    let zero: T = math::cast(0);

    let cell = get_cell3(*point);
    let mut range1: T = Float::max_value();
    let mut range2: T = Float::max_value();
    let mut seed_point1: math::Point3<T> = [zero, zero, zero];
    let mut seed_point2: math::Point3<T> = [zero, zero, zero];

    for x_offset in -1..2 {
        for y_offset in -1..2 {
            for z_offset in -1..2 {
                let cur_seed_point = get_cell_point3(seed, math::add3(cell, math::cast3([x_offset, y_offset, z_offset])));
                let cur_range = range_func(*point, cur_seed_point);
                if cur_range < range1 {
                    range2 = range1;
                    seed_point2 = seed_point1;
                    range1 = cur_range;
                    seed_point1 = cur_seed_point;
                } else if cur_range < range2 {
                    range2 = cur_range;
                    seed_point2 = cur_seed_point;
                }
            }
        }
    }

    (seed_point1, range1, seed_point2, range2)
}

#[inline(always)]
pub fn cell4_seed_2_points<T, F>(seed: &Seed, point: &math::Point4<T>, range_func: F) -> (math::Point4<T>, T, math::Point4<T>, T)
    where T: Float, F: Fn(math::Point4<T>, math::Point4<T>) -> T
{
    let zero: T = math::cast(0);

    let cell = get_cell4(*point);
    let mut range1: T = Float::max_value();
    let mut range2: T = Float::max_value();
    let mut seed_point1: math::Point4<T> = [zero, zero, zero, zero];
    let mut seed_point2: math::Point4<T> = [zero, zero, zero, zero];

    for x_offset in -1..2 {
        for y_offset in -1..2 {
            for z_offset in -1..2 {
                for w_offset in -1..2 {
                    let cur_seed_point = get_cell_point4(seed, math::add4(cell, math::cast4([x_offset, y_offset, z_offset, w_offset])));
                    let cur_range = range_func(*point, cur_seed_point);
                    if cur_range < range1 {
                        range2 = range1;
                        seed_point2 = seed_point1;
                        range1 = cur_range;
                        seed_point1 = cur_seed_point;
                    } else if cur_range < range2 {
                        range2 = cur_range;
                        seed_point2 = cur_seed_point;
                    }
                }
            }
        }
    }

    (seed_point1, range1, seed_point2, range2)
}

pub fn cell2_range<T: Float>(seed: &Seed, point: &math::Point2<T>) -> T {
    let (_, range) = cell2_seed_point(seed, point, range_sqr_euclidian2);
    range
}

pub fn cell3_range<T: Float>(seed: &Seed, point: &math::Point3<T>) -> T {
    let (_, range) = cell3_seed_point(seed, point, range_sqr_euclidian3);
    range
}

pub fn cell4_range<T: Float>(seed: &Seed, point: &math::Point4<T>) -> T {
    let (_, range) = cell4_seed_point(seed, point, range_sqr_euclidian4);
    range
}

pub fn cell2_range_inv<T: Float>(seed: &Seed, point: &math::Point2<T>) -> T {
    let (_, range1, _, range2) = cell2_seed_2_points(seed, point, range_sqr_euclidian2);
    range2 - range1
}

pub fn cell3_range_inv<T: Float>(seed: &Seed, point: &math::Point3<T>) -> T {
    let (_, range1, _, range2) = cell3_seed_2_points(seed, point, range_sqr_euclidian3);
    range2 - range1
}

pub fn cell4_range_inv<T: Float>(seed: &Seed, point: &math::Point4<T>) -> T {
    let (_, range1, _, range2) = cell4_seed_2_points(seed, point, range_sqr_euclidian4);
    range2 - range1
}

pub fn cell2_manhattan<T: Float>(seed: &Seed, point: &math::Point2<T>) -> T {
    let (_, range) = cell2_seed_point(seed, point, range_manhattan2);
    range
}

pub fn cell3_manhattan<T: Float>(seed: &Seed, point: &math::Point3<T>) -> T {
    let (_, range) = cell3_seed_point(seed, point, range_manhattan3);
    range
}

pub fn cell4_manhattan<T: Float>(seed: &Seed, point: &math::Point4<T>) -> T {
    let (_, range) = cell4_seed_point(seed, point, range_manhattan4);
    range
}

pub fn cell2_manhattan_inv<T: Float>(seed: &Seed, point: &math::Point2<T>) -> T {
    let (_, range1, _, range2) = cell2_seed_2_points(seed, point, range_manhattan2);
    range2 - range1
}

pub fn cell3_manhattan_inv<T: Float>(seed: &Seed, point: &math::Point3<T>) -> T {
    let (_, range1, _, range2) = cell3_seed_2_points(seed, point, range_manhattan3);
    range2 - range1
}

pub fn cell4_manhattan_inv<T: Float>(seed: &Seed, point: &math::Point4<T>) -> T {
    let (_, range1, _, range2) = cell4_seed_2_points(seed, point, range_manhattan4);
    range2 - range1
}
