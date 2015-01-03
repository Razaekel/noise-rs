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

/*
    This is a form of gradient noise, similar to Perlin or simplex noise,
    but using a simplectic honeycomb. The 2D case is functionally identical
    to simplex noise. The 3D case is a tetrahedral-octohedral honeycomb and
    the 4D case is a 5-cell honeycomb. See the Wikipedia article on
    simplectic honeycombs for more info:
    http://en.wikipedia.org/wiki/Simplectic_honeycomb

    The virtue of this noise, over the alternatives, is it's algorithmic
    cleanliness. In any dimension, it can be seen as a series of layers of
    the noise function from the lower dimension. So for any given point,
    one can calculate the lower dimensional noise twice, one for the layer
    below and one for the layer above, to get the relevant set of gradient
    points. Thus, 3D noise can be composed out of two iterations of 2D
    noise, and 4D noise can be composed out of two iterations of 3D noise,
    or four iterations of 2D noise.
*/

use std::num::{cast, Float};

use {gradient, math, Seed};

struct SimplecticPoint2<T: Float> {
    x_cell: i64,
    y_cell: i64,
    x_offset: T,
    y_offset: T,
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
    fn from_simplectic_point_2(point: &SimplecticPoint2<T>, z_cell: i64, z_offset: T) -> SimplecticPoint3<T> {
        SimplecticPoint3 {
            x_cell: point.x_cell,
            y_cell: point.y_cell,
            z_cell: z_cell,
            x_offset: point.x_offset,
            y_offset: point.y_offset,
            z_offset: z_offset,
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

impl<T: Float> SimplecticPoint3<T> {
    fn from_simplectic_point_3(point: &SimplecticPoint3<T>, w_cell: i64, w_offset: T) -> SimplecticPoint4<T> {
        SimplecticPoint4 {
            x_cell: point.x_cell,
            y_cell: point.y_cell,
            z_cell: point.z_cell,
            w_cell: w_cell,
            x_offset: point.x_offset,
            y_offset: point.y_offset,
            z_offset: point.z_offset,
            w_offset: w_offset,
        }
    }
}

const SKEW_CONSTANT: f64 = 0.36602540378; // 0.5*(sqrt(3.0)-1.0)
const UNSKEW_CONSTANT: f64 = 0.2113248654; // (3.0-sqrt(3.0))/6.0

const SIMPLEX_SIZE: f64 = 0.70710678119;
const INV_SIMPLEX_SIZE: f64 = 1.41421356235; // 1 / SIMPLEX_SIZE
const LAYER_OFFSET_X: f64 = 0.45534180126; // (2.0-3.0*UNSKEW_CONSTANT)/3.0
const LAYER_OFFSET_Y: f64 = 0.12200846793; // (1.0-3.0*UNSKEW_CONSTANT)/3.0
const LAYER_OFFSET_Z: f64 = 0.35355339059; // (1.0-3.0*UNSKEW_CONSTANT)/3.0

const NORM_CONSTANT_2D: f64 = 8.0;
const NORM_CONSTANT_3D: f64 = 9.0;
const NORM_CONSTANT_4D: f64 = 10.0;

#[inline(always)]
fn simplectic2_points<T: Float>(point: &::Point2<T>) -> [SimplecticPoint2<T>, ..3] {
    let zero: T = math::cast(0u);
    let one: T = math::cast(1u);
    let two: T = math::cast(2u);
    let skew_constant: T = math::cast(SKEW_CONSTANT);
    let unskew_constant: T = math::cast(UNSKEW_CONSTANT);

    // Skew the input coordinates into the grid to figure out which grid cell we're in
    let skew_offset = (point[0] + point[1]) * skew_constant;
    let x_cell = (point[0] + skew_offset).floor();
    let y_cell = (point[1] + skew_offset).floor();

    // Unskew the floored coordinates to find the real coordinates of the cell's origin
    let unskew_offset = (x_cell + y_cell) * unskew_constant;
    let x_origin = x_cell - unskew_offset;
    let y_origin = y_cell - unskew_offset;

    // Compute the delta from the first point, which is the cell origin
    let dx0 = point[0] - x_origin;
    let dy0 = point[1] - y_origin;

    // Compute the delta from the second point, which depends on which simplex we're in
    let (x1_offset, y1_offset) = if dx0 > dy0 { (one, zero) } else { (zero, one) };
    let dx1 = dx0 - x1_offset + unskew_constant;
    let dy1 = dy0 - y1_offset + unskew_constant;

    // Compute the delta from the third point
    let dx2 = dx0 - one + two * unskew_constant;
    let dy2 = dy0 - one + two * unskew_constant;

    [
        SimplecticPoint2 {
            x_cell: x_cell.to_i64().unwrap(),
            y_cell: y_cell.to_i64().unwrap(),
            x_offset: dx0,
            y_offset: dy0,
        },
        SimplecticPoint2 {
            x_cell: (x_cell + x1_offset).to_i64().unwrap(),
            y_cell: (y_cell + y1_offset).to_i64().unwrap(),
            x_offset: dx1,
            y_offset: dy1,
        },
        SimplecticPoint2 {
            x_cell: x_cell.to_i64().unwrap() + 1,
            y_cell: y_cell.to_i64().unwrap() + 1,
            x_offset: dx2,
            y_offset: dy2,
        },
    ]
}

#[inline(always)]
fn simplectic3_points<T: Float>(point: &::Point3<T>) -> [SimplecticPoint3<T>, ..6] {
    let layer_offset_x: T = math::cast(LAYER_OFFSET_X);
    let layer_offset_y: T = math::cast(LAYER_OFFSET_Y);
    let layer_scale: T = math::cast(SIMPLEX_SIZE);
    let inv_layer_scale: T = math::cast(INV_SIMPLEX_SIZE);

    let layer = (point[2] * inv_layer_scale).floor();
    let layer_int = layer.to_i64().unwrap();

    let (layer1_point, layer2_point) = if layer_int % 2 == 0 {
        ([point[0], point[1]], [point[0] + layer_offset_x, point[1] + layer_offset_y])
    } else {
        ([point[0] + layer_offset_x, point[1] + layer_offset_y], [point[0], point[1]])
    };

    let [p1, p2, p3] = simplectic2_points(&layer1_point);
    let [p4, p5, p6] = simplectic2_points(&layer2_point);

    let z_offset = point[2] - layer * layer_scale;
    [
        SimplecticPoint3::from_simplectic_point_2(&p1, layer_int, z_offset),
        SimplecticPoint3::from_simplectic_point_2(&p2, layer_int, z_offset),
        SimplecticPoint3::from_simplectic_point_2(&p3, layer_int, z_offset),
        SimplecticPoint3::from_simplectic_point_2(&p4, layer_int + 1, z_offset - layer_scale),
        SimplecticPoint3::from_simplectic_point_2(&p5, layer_int + 1, z_offset - layer_scale),
        SimplecticPoint3::from_simplectic_point_2(&p6, layer_int + 1, z_offset - layer_scale),
    ]
}

#[inline(always)]
fn simplectic4_points<T: Float>(point: &::Point4<T>) -> [SimplecticPoint4<T>, ..12] {
    let layer_offset_x: T = math::cast(LAYER_OFFSET_X);
    let layer_offset_y: T = math::cast(LAYER_OFFSET_Y);
    let layer_offset_z: T = math::cast(LAYER_OFFSET_Z);
    let layer_scale: T = math::cast(SIMPLEX_SIZE);
    let inv_layer_scale: T = math::cast(INV_SIMPLEX_SIZE);

    let layer = (point[3] * inv_layer_scale).floor();
    let layer_int = layer.to_i64().unwrap();

    let (layer1_point, layer2_point) = if layer_int % 2 == 0 {
        ([point[0], point[1], point[2]], [point[0] + layer_offset_x, point[1] + layer_offset_y, point[2] + layer_offset_z])
    } else {
        ([point[0] + layer_offset_x, point[1] + layer_offset_y, point[2] + layer_offset_z], [point[0], point[1], point[2]])
    };

    let [p1, p2, p3, p4, p5, p6] = simplectic3_points(&layer1_point);
    let [p7, p8, p9, p10, p11, p12] = simplectic3_points(&layer2_point);

    let w_offset = point[3] - layer * layer_scale;
    [
        SimplecticPoint3::from_simplectic_point_3(&p1, layer_int, w_offset),
        SimplecticPoint3::from_simplectic_point_3(&p2, layer_int, w_offset),
        SimplecticPoint3::from_simplectic_point_3(&p3, layer_int, w_offset),
        SimplecticPoint3::from_simplectic_point_3(&p4, layer_int, w_offset),
        SimplecticPoint3::from_simplectic_point_3(&p5, layer_int, w_offset),
        SimplecticPoint3::from_simplectic_point_3(&p6, layer_int, w_offset),
        SimplecticPoint3::from_simplectic_point_3(&p7, layer_int + 1, w_offset - layer_scale),
        SimplecticPoint3::from_simplectic_point_3(&p8, layer_int + 1, w_offset - layer_scale),
        SimplecticPoint3::from_simplectic_point_3(&p9, layer_int + 1, w_offset - layer_scale),
        SimplecticPoint3::from_simplectic_point_3(&p10, layer_int + 1, w_offset - layer_scale),
        SimplecticPoint3::from_simplectic_point_3(&p11, layer_int + 1, w_offset - layer_scale),
        SimplecticPoint3::from_simplectic_point_3(&p12, layer_int + 1, w_offset - layer_scale),
    ]
}

pub fn simplectic2<T: Float>(seed: &Seed, point: &::Point2<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: &Seed, p: &SimplecticPoint2<T>) -> T {
        let attn = math::cast::<_, T>(SIMPLEX_SIZE) - p.x_offset*p.x_offset - p.y_offset*p.y_offset;
        if attn > Float::zero() {
            let vec = gradient::get2::<T>(seed.get2(p.x_cell, p.y_cell));
            let attn2 = attn*attn;
            attn2*attn2*(p.x_offset*vec[0] + p.y_offset*vec[1])
        } else {
            Float::zero()
        }
    }

    let [p1, p2, p3] = simplectic2_points(point);

    (gradient(seed, &p1) + gradient(seed, &p2) + gradient(seed, &p3)) * math::cast(NORM_CONSTANT_2D)
}

pub fn simplectic3<T: Float>(seed: &Seed, point: &::Point3<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: &Seed, p: &SimplecticPoint3<T>) -> T {
        let attn = math::cast::<_, T>(SIMPLEX_SIZE) - p.x_offset*p.x_offset - p.y_offset*p.y_offset - p.z_offset*p.z_offset;
        if attn > Float::zero() {
            let vec = gradient::get3::<T>(seed.get3(p.x_cell, p.y_cell, p.z_cell));
            let attn2 = attn*attn;
            attn2*attn2*(p.x_offset*vec[0] + p.y_offset*vec[1] + p.z_offset*vec[2])
        } else {
            Float::zero()
        }
    }

    let [p1, p2, p3, p4, p5, p6] = simplectic3_points(point);

    (
        gradient(seed, &p1) + gradient(seed, &p2) + gradient(seed, &p3) + gradient(seed, &p4) + gradient(seed, &p5) + gradient(seed, &p6)
    ) * math::cast(NORM_CONSTANT_3D)
}

pub fn simplectic4<T: Float>(seed: &Seed, point: &::Point4<T>) -> T {
    #[inline(always)]
    fn gradient<T: Float>(seed: &Seed, p: &SimplecticPoint4<T>) -> T {
        let attn = math::cast::<_, T>(SIMPLEX_SIZE) - p.x_offset*p.x_offset - p.y_offset*p.y_offset - p.z_offset*p.z_offset - p.w_offset*p.w_offset;
        if attn > Float::zero() {
            let vec = gradient::get4::<T>(seed.get4(p.x_cell, p.y_cell, p.z_cell, p.w_cell));
            let attn2 = attn*attn;
            attn2*attn2*(p.x_offset*vec[0] + p.y_offset*vec[1] + p.z_offset*vec[2] + p.w_offset*vec[3])
        } else {
            Float::zero()
        }
    }

    let [p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12] = simplectic4_points(point);

    (
        gradient(seed, &p1) + gradient(seed, &p2) + gradient(seed, &p3) + gradient(seed, &p4) + gradient(seed, &p5) + gradient(seed, &p6) +
        gradient(seed, &p7) + gradient(seed, &p8) + gradient(seed, &p9) + gradient(seed, &p10) + gradient(seed, &p11) + gradient(seed, &p12)
    ) * math::cast(NORM_CONSTANT_4D)
}
