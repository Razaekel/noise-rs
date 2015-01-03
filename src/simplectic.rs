// Copyright 2013 The noise-rs developers. For a full listing of the authors,
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

//! This is a form of gradient noise, similar to Perlin or simplex noise,
//! but using a simplectic honeycomb. The 2D case is functionally identical
//! to simplex noise. The 3D case is a tetrahedral-octohedral honeycomb and
//! the 4D case is a 5-cell honeycomb. See the Wikipedia article on
//! simplectic honeycombs for more info:
//! http://en.wikipedia.org/wiki/Simplectic_honeycomb
//!
//! The virtue of this noise, over the alternatives, is it's algorithmic
//! cleanliness. In any dimension, it can be seen as a series of layers of
//! the noise function from the lower dimension. So for any given point,
//! one can calculate the lower dimensional noise twice, one for the layer
//! below and one for the layer above, to get the relevant set of gradient
//! points. Thus, 3D noise can be composed out of two iterations of 2D
//! noise, and 4D noise can be composed out of two iterations of 3D noise,
//! or four iterations of 2D noise.

use std::num::{cast, Float};

use {gradient, math, Seed};

struct SimplecticPoint2<T: Float> {
    x_cell: i64,
    y_cell: i64,
    x_offset: T,
    y_offset: T,
}

impl<T: Float> SimplecticPoint2<T> {
    fn to_simplectic_point3(&self, z_cell: i64, z_offset: T) -> SimplecticPoint3<T> {
        SimplecticPoint3 {
            x_cell: self.x_cell,
            y_cell: self.y_cell,
            z_cell: z_cell,
            x_offset: self.x_offset,
            y_offset: self.y_offset,
            z_offset: z_offset,
        }
    }
}

struct SimplecticPoint3<T: Float> {
    x_cell: i64,
    y_cell: i64,
    z_cell: i64,
    x_offset: T,
    y_offset: T,
    z_offset: T,
}

impl<T: Float> SimplecticPoint3<T> {
    fn to_simplectic_point4(&self, w_cell: i64, w_offset: T) -> SimplecticPoint4<T> {
        SimplecticPoint4 {
            x_cell: self.x_cell,
            y_cell: self.y_cell,
            z_cell: self.z_cell,
            w_cell: w_cell,
            x_offset: self.x_offset,
            y_offset: self.y_offset,
            z_offset: self.z_offset,
            w_offset: w_offset,
        }
    }
}

struct SimplecticPoint4<T: Float> {
    x_cell: i64,
    y_cell: i64,
    z_cell: i64,
    w_cell: i64,
    x_offset: T,
    y_offset: T,
    z_offset: T,
    w_offset: T,
}

fn skew_constant<T: Float>() -> T { math::cast(0.36602540378_f64) } // 0.5 * (sqrt(3.0) - 1.0)
fn unskew_constant<T: Float>() -> T { math::cast(0.2113248654_f64) } // (3.0 - sqrt(3.0)) / 6.0

fn simplex_size<T: Float>() -> T { math::cast(0.70710678119_f64) }
fn inv_simplex_size<T: Float>() -> T { math::cast(1.41421356235_f64) } // 1 / simplex_size()
fn layer_offset_x<T: Float>() -> T { math::cast(0.45534180126_f64) } // (2.0 - 3.0 * unskew_constant()) / 3.0
fn layer_offset_y<T: Float>() -> T { math::cast(0.12200846793_f64) } // (1.0 - 3.0 * unskew_constant()) / 3.0
fn layer_offset_z<T: Float>() -> T { math::cast(0.35355339059_f64) } // (1.0 - 3.0 * unskew_constant()) / 3.0

fn norm2_constant<T: Float>() -> T { math::cast(8.0_f64) }
fn norm3_constant<T: Float>() -> T { math::cast(9.0_f64) }
fn norm4_constant<T: Float>() -> T { math::cast(10.0_f64) }

#[inline(always)]
fn simplectic2_points<T: Float>(point: &::Point2<T>) -> [SimplecticPoint2<T>; 3] {
    let zero: T = math::cast(0u);
    let one: T = math::cast(1u);
    let two: T = math::cast(2u);

    // Skew the input coordinates into the grid to figure out which grid cell we're in
    let skew_offset = (point[0] + point[1]) * skew_constant();
    let x_cell = (point[0] + skew_offset).floor();
    let y_cell = (point[1] + skew_offset).floor();

    // Unskew the floored coordinates to find the real coordinates of the cell's origin
    let unskew_offset = (x_cell + y_cell) * unskew_constant();
    let x_origin = x_cell - unskew_offset;
    let y_origin = y_cell - unskew_offset;

    // Compute the delta from the first point, which is the cell origin
    let dx0 = point[0] - x_origin;
    let dy0 = point[1] - y_origin;

    // Compute the delta from the second point, which depends on which simplex we're in
    let (x1_offset, y1_offset) = if dx0 > dy0 { (one, zero) } else { (zero, one) };
    let dx1 = dx0 - x1_offset + unskew_constant();
    let dy1 = dy0 - y1_offset + unskew_constant();

    // Compute the delta from the third point
    let dx2 = dx0 - one + two * unskew_constant();
    let dy2 = dy0 - one + two * unskew_constant();

    [
        SimplecticPoint2 {
            x_cell: math::cast(x_cell),
            y_cell: math::cast(y_cell),
            x_offset: dx0,
            y_offset: dy0,
        },
        SimplecticPoint2 {
            x_cell: math::cast(x_cell + x1_offset),
            y_cell: math::cast(y_cell + y1_offset),
            x_offset: dx1,
            y_offset: dy1,
        },
        SimplecticPoint2 {
            x_cell: 1i64 + math::cast(x_cell),
            y_cell: 1i64 + math::cast(y_cell),
            x_offset: dx2,
            y_offset: dy2,
        },
    ]
}

#[inline(always)]
fn simplectic3_points<T: Float>(point: &::Point3<T>) -> [SimplecticPoint3<T>; 6] {
    let layer = (point[2] * inv_simplex_size()).floor();
    let layer_int: i64 = math::cast(layer);

    let (layer1_point, layer2_point) = if layer_int % 2 == 0 {
        ([point[0], point[1]], [point[0] + layer_offset_x(), point[1] + layer_offset_y()])
    } else {
        ([point[0] + layer_offset_x(), point[1] + layer_offset_y()], [point[0], point[1]])
    };

    let [p1, p2, p3] = simplectic2_points(&layer1_point);
    let [p4, p5, p6] = simplectic2_points(&layer2_point);

    let z_offset = point[2] - layer * simplex_size();
    [
        p1.to_simplectic_point3(layer_int, z_offset),
        p2.to_simplectic_point3(layer_int, z_offset),
        p3.to_simplectic_point3(layer_int, z_offset),
        p4.to_simplectic_point3(layer_int + 1, z_offset - simplex_size()),
        p5.to_simplectic_point3(layer_int + 1, z_offset - simplex_size()),
        p6.to_simplectic_point3(layer_int + 1, z_offset - simplex_size()),
    ]
}

#[inline(always)]
fn simplectic4_points<T: Float>(point: &::Point4<T>) -> [SimplecticPoint4<T>; 12] {
    let layer = (point[3] * inv_simplex_size()).floor();
    let layer_int: i64 = math::cast(layer);

    let (layer1_point, layer2_point) = if layer_int % 2 == 0 {
        ([point[0], point[1], point[2]], [point[0] + layer_offset_x(), point[1] + layer_offset_y(), point[2] + layer_offset_z()])
    } else {
        ([point[0] + layer_offset_x(), point[1] + layer_offset_y(), point[2] + layer_offset_z()], [point[0], point[1], point[2]])
    };

    let [p1, p2, p3, p4, p5, p6] = simplectic3_points(&layer1_point);
    let [p7, p8, p9, p10, p11, p12] = simplectic3_points(&layer2_point);

    let w_offset = point[3] - layer * simplex_size();
    [
        p1.to_simplectic_point4(layer_int, w_offset),
        p2.to_simplectic_point4(layer_int, w_offset),
        p3.to_simplectic_point4(layer_int, w_offset),
        p4.to_simplectic_point4(layer_int, w_offset),
        p5.to_simplectic_point4(layer_int, w_offset),
        p6.to_simplectic_point4(layer_int, w_offset),
        p7.to_simplectic_point4(layer_int + 1, w_offset - simplex_size()),
        p8.to_simplectic_point4(layer_int + 1, w_offset - simplex_size()),
        p9.to_simplectic_point4(layer_int + 1, w_offset - simplex_size()),
        p10.to_simplectic_point4(layer_int + 1, w_offset - simplex_size()),
        p11.to_simplectic_point4(layer_int + 1, w_offset - simplex_size()),
        p12.to_simplectic_point4(layer_int + 1, w_offset - simplex_size()),
    ]
}

pub fn simplectic2<T: Float>(seed: &Seed, point: &::Point2<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: &Seed, p: &SimplecticPoint2<T>) -> T {
        let attn = simplex_size::<T>() - p.x_offset * p.x_offset
                                       - p.y_offset * p.y_offset;
        if attn > Float::zero() {
            let [gx, gy] = gradient::get2(seed.get2(p.x_cell, p.y_cell));
            let attn2 = attn * attn;
            attn2 * attn2 * (p.x_offset * gx + p.y_offset * gy)
        } else {
            Float::zero()
        }
    }

    let [p1, p2, p3] = simplectic2_points(point);

    (gradient(seed, &p1) + gradient(seed, &p2) + gradient(seed, &p3)) * norm2_constant()
}

pub fn simplectic3<T: Float>(seed: &Seed, point: &::Point3<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: &Seed, p: &SimplecticPoint3<T>) -> T {
        let attn = simplex_size::<T>() - p.x_offset * p.x_offset
                                       - p.y_offset * p.y_offset
                                       - p.z_offset * p.z_offset;
        if attn > Float::zero() {
            let [gx, gy, gz] = gradient::get3(seed.get3(p.x_cell, p.y_cell, p.z_cell));
            let attn2 = attn * attn;
            attn2 * attn2 * (p.x_offset * gx + p.y_offset * gy + p.z_offset * gz)
        } else {
            Float::zero()
        }
    }

    let [p1, p2, p3, p4, p5, p6] = simplectic3_points(point);

    (
        gradient(seed, &p1) + gradient(seed, &p2) + gradient(seed, &p3) + gradient(seed, &p4) + gradient(seed, &p5) + gradient(seed, &p6)
    ) * norm3_constant()
}

pub fn simplectic4<T: Float>(seed: &Seed, point: &::Point4<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: &Seed, p: &SimplecticPoint4<T>) -> T {
        let attn = simplex_size::<T>() - p.x_offset * p.x_offset
                                       - p.y_offset * p.y_offset
                                       - p.z_offset * p.z_offset
                                       - p.w_offset * p.w_offset;
        if attn > Float::zero() {
            let [gx, gy, gz, gw] = gradient::get4(seed.get4(p.x_cell, p.y_cell, p.z_cell, p.w_cell));
            let attn2 = attn * attn;
            attn2 * attn2 * (p.x_offset * gx + p.y_offset * gy + p.z_offset * gz + p.w_offset * gw)
        } else {
            Float::zero()
        }
    }

    let [p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12] = simplectic4_points(point);

    (
        gradient(seed, &p1) + gradient(seed, &p2) + gradient(seed, &p3) + gradient(seed, &p4) + gradient(seed, &p5) + gradient(seed, &p6) +
        gradient(seed, &p7) + gradient(seed, &p8) + gradient(seed, &p9) + gradient(seed, &p10) + gradient(seed, &p11) + gradient(seed, &p12)
    ) * norm4_constant()
}
