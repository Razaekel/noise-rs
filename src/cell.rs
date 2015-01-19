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
pub fn get_vec2<T: Float>(index: usize) -> math::Point2<T> {
    let length = math::cast::<_,T>((index & 0xF8) >> 3) * math::cast(0.5 / 31.0);
    let diag = length * math::cast(0.70710678118f32);
    let one = length;
    let zero: T = math::cast(0.0f32);
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
pub fn get_vec3<T: Float>(index: usize) -> math::Point3<T> {
    let length = math::cast::<_,T>((index & 0xE0) >> 5) * math::cast(0.5 / 7.0);
    let diag = length * math::cast(0.70710678118f32);
    let one = length;
    let zero = math::cast(0.0f32);
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
pub fn get_vec4<T: Float>(index: usize) -> math::Point4<T> {
    let length = math::cast::<_,T>((index & 0xE0) >> 5) * math::cast(0.5 / 7.0);
    let diag = length * math::cast(0.57735026919f32);
    let zero = math::cast(0.0f32);
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
pub fn cell2_seed_point<T, F>(seed: &Seed, point: &math::Point2<T>, range_func: F) -> (math::Point2<T>, T)
    where T: Float, F: Fn(math::Point2<T>, math::Point2<T>) -> T
{
    fn get_point<T: Float>(seed: &Seed, whole: math::Point2<isize>) -> math::Point2<T> {
        math::add2(get_vec2(seed.get2(whole)), math::cast2::<_,T>(whole))
    }

    let whole0  = math::map2(*point, math::cast);
    let whole1  = math::add2(whole0, math::one2());

    let p00 = get_point(seed, [whole0[0], whole0[1]]);
    let p10 = get_point(seed, [whole1[0], whole0[1]]);
    let p01 = get_point(seed, [whole0[0], whole1[1]]);
    let p11 = get_point(seed, [whole1[0], whole1[1]]);

    let r00 = range_func(*point, p00);
    let r10 = range_func(*point, p10);
    let r01 = range_func(*point, p01);
    let r11 = range_func(*point, p11);

    if r00 < r10 && r00 < r01 && r00 < r11 {
        (p00, r00)
    } else if r10 < r01 && r10 < r11 {
        (p10, r10)
    } else if r01 < r11 {
        (p01, r01)
    } else {
        (p11, r11)
    }
}

#[inline(always)]
pub fn cell3_seed_point<T, F>(seed: &Seed, point: &math::Point3<T>, range_func: F) -> (math::Point3<T>, T)
    where T: Float, F: Fn(math::Point3<T>, math::Point3<T>) -> T
{
    fn get_point<T: Float>(seed: &Seed, whole: math::Point3<isize>) -> math::Point3<T> {
        math::add3(get_vec3(seed.get3(whole)), math::cast3::<_,T>(whole))
    }

    let whole0  = math::map3(*point, math::cast);
    let whole1  = math::add3(whole0, math::one3());

    let p000 = get_point(seed, [whole0[0], whole0[1], whole0[2]]);
    let p100 = get_point(seed, [whole1[0], whole0[1], whole0[2]]);
    let p010 = get_point(seed, [whole0[0], whole1[1], whole0[2]]);
    let p110 = get_point(seed, [whole1[0], whole1[1], whole0[2]]);
    let p001 = get_point(seed, [whole0[0], whole0[1], whole1[2]]);
    let p101 = get_point(seed, [whole1[0], whole0[1], whole1[2]]);
    let p011 = get_point(seed, [whole0[0], whole1[1], whole1[2]]);
    let p111 = get_point(seed, [whole1[0], whole1[1], whole1[2]]);

    let r000 = range_func(*point, p000);
    let r100 = range_func(*point, p100);
    let r010 = range_func(*point, p010);
    let r110 = range_func(*point, p110);
    let r001 = range_func(*point, p001);
    let r101 = range_func(*point, p101);
    let r011 = range_func(*point, p011);
    let r111 = range_func(*point, p111);

    if r000 < r100 && r000 < r010 && r000 < r110 && r000 < r001 && r000 < r101 && r000 < r011 && r000 < r111 {
        (p000, r000)
    } else if r100 < r010 && r100 < r110 && r100 < r001 && r100 < r101 && r100 < r011 && r100 < r111 {
        (p100, r100)
    } else if r010 < r110 && r010 < r001 && r010 < r101 && r010 < r011 && r010 < r111 {
        (p010, r010)
    } else if r110 < r001 && r110 < r101 && r110 < r011 && r110 < r111 {
        (p110, r110)
    } else if r001 < r101 && r001 < r011 && r001 < r111 {
        (p001, r001)
    } else if r101 < r011 && r101 < r111 {
        (p101, r101)
    } else if r011 < r111 {
        (p011, r011)
    } else {
        (p111, r111)
    }
}

#[inline(always)]
pub fn cell4_seed_point<T, F>(seed: &Seed, point: &math::Point4<T>, range_func: F) -> (math::Point4<T>, T)
    where T: Float, F: Fn(math::Point4<T>, math::Point4<T>) -> T
{
    fn get_point<T: Float>(seed: &Seed, whole: math::Point4<isize>) -> math::Point4<T> {
        math::add4(get_vec4(seed.get4(whole)), math::cast4::<_,T>(whole))
    }

    let whole0  = math::map4(*point, math::cast);
    let whole1  = math::add4(whole0, math::one4());

    let p0000 = get_point(seed, [whole0[0], whole0[1], whole0[2], whole0[3]]);
    let p1000 = get_point(seed, [whole1[0], whole0[1], whole0[2], whole0[3]]);
    let p0100 = get_point(seed, [whole0[0], whole1[1], whole0[2], whole0[3]]);
    let p1100 = get_point(seed, [whole1[0], whole1[1], whole0[2], whole0[3]]);
    let p0010 = get_point(seed, [whole0[0], whole0[1], whole1[2], whole0[3]]);
    let p1010 = get_point(seed, [whole1[0], whole0[1], whole1[2], whole0[3]]);
    let p0110 = get_point(seed, [whole0[0], whole1[1], whole1[2], whole0[3]]);
    let p1110 = get_point(seed, [whole1[0], whole1[1], whole1[2], whole0[3]]);
    let p0001 = get_point(seed, [whole0[0], whole0[1], whole0[2], whole1[3]]);
    let p1001 = get_point(seed, [whole1[0], whole0[1], whole0[2], whole1[3]]);
    let p0101 = get_point(seed, [whole0[0], whole1[1], whole0[2], whole1[3]]);
    let p1101 = get_point(seed, [whole1[0], whole1[1], whole0[2], whole1[3]]);
    let p0011 = get_point(seed, [whole0[0], whole0[1], whole1[2], whole1[3]]);
    let p1011 = get_point(seed, [whole1[0], whole0[1], whole1[2], whole1[3]]);
    let p0111 = get_point(seed, [whole0[0], whole1[1], whole1[2], whole1[3]]);
    let p1111 = get_point(seed, [whole1[0], whole1[1], whole1[2], whole1[3]]);

    let r0000 = range_func(*point, p0000);
    let r1000 = range_func(*point, p1000);
    let r0100 = range_func(*point, p0100);
    let r1100 = range_func(*point, p1100);
    let r0010 = range_func(*point, p0010);
    let r1010 = range_func(*point, p1010);
    let r0110 = range_func(*point, p0110);
    let r1110 = range_func(*point, p1110);
    let r0001 = range_func(*point, p0001);
    let r1001 = range_func(*point, p1001);
    let r0101 = range_func(*point, p0101);
    let r1101 = range_func(*point, p1101);
    let r0011 = range_func(*point, p0011);
    let r1011 = range_func(*point, p1011);
    let r0111 = range_func(*point, p0111);
    let r1111 = range_func(*point, p1111);

    if r0000 < r1000 && r0000 < r0100 && r0000 < r1100 && r0000 < r0010 && r0000 < r1010 && r0000 < r0110 && r0000 < r1110 &&
        r0000 < r0001 && r0000 < r1001 && r0000 < r0101 && r0000 < r1101 && r0000 < r0011 && r0000 < r1011 && r0000 < r0111 && r0000 < r1111 {
        (p0000, r0000)
    } else if r1000 < r0100 && r1000 < r1100 && r1000 < r0010 && r1000 < r1010 && r1000 < r0110 && r1000 < r1110 && r1000 < r0001 &&
        r1000 < r1001 && r1000 < r0101 && r1000 < r1101 && r1000 < r0011 && r1000 < r1011 && r1000 < r0111 && r1000 < r1111 {
        (p1000, r1000)
    } else if r0100 < r1100 && r0100 < r0010 && r0100 < r1010 && r0100 < r0110 && r0100 < r1110 && r0100 < r0001 && r0100 < r1001 &&
        r0100 < r0101 && r0100 < r1101 && r0100 < r0011 && r0100 < r1011 && r0100 < r0111 && r0100 < r1111 {
        (p0100, r0100)
    } else if r1100 < r0010 && r1100 < r1010 && r1100 < r0110 && r1100 < r1110 && r1100 < r0001 && r1100 < r1001 && r1100 < r0101 &&
        r1100 < r1101 && r1100 < r0011 && r1100 < r1011 && r1100 < r0111 && r1100 < r1111 {
        (p1100, r1100)
    } else if r0010 < r1010 && r0010 < r0110 && r0010 < r1110 && r0010 < r0001 && r0010 < r1001 && r0010 < r0101 && r0010 < r1101 &&
        r0010 < r0011 && r0010 < r1011 && r0010 < r0111 && r0010 < r1111 {
        (p0010, r0010)
    } else if r1010 < r0110 && r1010 < r1110 && r1010 < r0001 && r1010 < r1001 && r1010 < r0101 && r1010 < r1101 && r1010 < r0011 &&
        r1010 < r1011 && r1010 < r0111 && r1010 < r1111 {
        (p1010, r1010)
    } else if r0110 < r1110 && r0110 < r0001 && r0110 < r1001 && r0110 < r0101 && r0110 < r1101 && r0110 < r0011 && r0110 < r1011 &&
        r0110 < r0111 && r0110 < r1111 {
        (p0110, r0110)
    } else if r1110 < r0001 && r1110 < r1001 && r1110 < r0101 && r1110 < r1101 && r1110 < r0011 && r1110 < r1011 && r1110 < r0111 &&
        r1110 < r1111 {
        (p1110, r1110)
    } else if r0001 < r1001 && r0001 < r0101 && r0001 < r1101 && r0001 < r0011 && r0001 < r1011 && r0001 < r0111 && r0001 < r1111 {
        (p0001, r0001)
    } else if r1001 < r0101 && r1001 < r1101 && r1001 < r0011 && r1001 < r1011 && r1001 < r0111 && r1001 < r1111 {
        (p1001, r1001)
    } else if r0101 < r1101 && r0101 < r0011 && r0101 < r1011 && r0101 < r0111 && r0101 < r1111 {
        (p0101, r0101)
    } else if r1101 < r0011 && r1101 < r1011 && r1101 < r0111 && r1101 < r1111 {
        (p1101, r1101)
    } else if r0011 < r1011 && r0011 < r0111 && r0011 < r1111 {
        (p0011, r0011)
    } else if r1011 < r0111 && r1011 < r1111 {
        (p1011, r1011)
    } else if r0111 < r1111 {
        (p0111, r0111)
    } else {
        (p1111, r1111)
    }
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

#[inline(always)]
pub fn cell2_seed_cell<T, F>(seed: &Seed, point: &math::Point2<T>, range_func: F) -> math::Point2<i64>
    where T: Float, F: Fn(math::Point2<T>, math::Point2<T>) -> T
{
    let cell = get_cell2(*point);
    let mut range: T = Float::max_value();
    let mut seed_cell: math::Point2<i64> = [0, 0];

    for x_offset in -1..2 {
        for y_offset in -1..2 {
            let cell = math::add2(cell, math::cast2([x_offset, y_offset]));
            let cur_seed_point = get_cell_point2(seed, cell);
            let cur_range = range_func(*point, cur_seed_point);
            if cur_range < range {
                range = cur_range;
                seed_cell = math::cast2(cell);
            }
        }
    }

    seed_cell
}

#[inline(always)]
pub fn cell3_seed_cell<T, F>(seed: &Seed, point: &math::Point3<T>, range_func: F) -> math::Point3<i64>
    where T: Float, F: Fn(math::Point3<T>, math::Point3<T>) -> T
{
    let cell = get_cell3(*point);
    let mut range: T = Float::max_value();
    let mut seed_cell: math::Point3<i64> = [0, 0, 0];

    for x_offset in -1..2 {
        for y_offset in -1..2 {
            for z_offset in -1..2 {
                let cell = math::add3(cell, math::cast3([x_offset, y_offset, z_offset]));
                let cur_seed_point = get_cell_point3(seed, cell);
                let cur_range = range_func(*point, cur_seed_point);
                if cur_range < range {
                    range = cur_range;
                    seed_cell = math::cast3(cell);
                }
            }
        }
    }

    seed_cell
}

#[inline(always)]
pub fn cell4_seed_cell<T, F>(seed: &Seed, point: &math::Point4<T>, range_func: F) -> math::Point4<i64>
    where T: Float, F: Fn(math::Point4<T>, math::Point4<T>) -> T
{
    let cell = get_cell4(*point);
    let mut range: T = Float::max_value();
    let mut seed_cell: math::Point4<i64> = [0, 0, 0, 0];

    for x_offset in -1..2 {
        for y_offset in -1..2 {
            for z_offset in -1..2 {
                for w_offset in -1..2 {
                    let cell = math::add4(cell, math::cast4([x_offset, y_offset, z_offset, w_offset]));
                    let cur_seed_point = get_cell_point4(seed, cell);
                    let cur_range = range_func(*point, cur_seed_point);
                    if cur_range < range {
                        range = cur_range;
                        seed_cell = math::cast4(cell);
                    }
                }
            }
        }
    }

    seed_cell
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

pub fn cell2_value<T: Float>(seed: &Seed, point: &math::Point2<T>) -> T {
    let cell = cell2_seed_cell(seed, point, range_sqr_euclidian2);
    math::cast::<_,T>(seed.get2(cell)) * math::cast(1.0 / 255.0)
}

pub fn cell3_value<T: Float>(seed: &Seed, point: &math::Point3<T>) -> T {
    let cell = cell3_seed_cell(seed, point, range_sqr_euclidian3);
    math::cast::<_,T>(seed.get3(cell)) * math::cast(1.0 / 255.0)
}

pub fn cell4_value<T: Float>(seed: &Seed, point: &math::Point4<T>) -> T {
    let cell = cell4_seed_cell(seed, point, range_sqr_euclidian4);
    math::cast::<_,T>(seed.get4(cell)) * math::cast(1.0 / 255.0)
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

pub fn cell2_manhattan_value<T: Float>(seed: &Seed, point: &math::Point2<T>) -> T {
    let cell = cell2_seed_cell(seed, point, range_manhattan2);
    math::cast::<_,T>(seed.get2(cell)) * math::cast(1.0 / 255.0)
}

pub fn cell3_manhattan_value<T: Float>(seed: &Seed, point: &math::Point3<T>) -> T {
    let cell = cell3_seed_cell(seed, point, range_manhattan3);
    math::cast::<_,T>(seed.get3(cell)) * math::cast(1.0 / 255.0)
}

pub fn cell4_manhattan_value<T: Float>(seed: &Seed, point: &math::Point4<T>) -> T {
    let cell = cell4_seed_cell(seed, point, range_manhattan4);
    math::cast::<_,T>(seed.get4(cell)) * math::cast(1.0 / 255.0)
}
