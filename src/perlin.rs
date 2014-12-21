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

use std::num::Float;

use seed::Seed;
use util::{lerp, scurve3, scurve5};

const GRADIENT_2D: [::Point2d<f32>, ..8] = [
    [ 0.70710678118,  0.70710678118],
    [ 0.70710678118, -0.70710678118],
    [-0.70710678118,  0.70710678118],
    [-0.70710678118, -0.70710678118],
    [ 1.0,  0.0],
    [-1.0,  0.0],
    [ 0.0,  1.0],
    [ 0.0, -1.0],
];

const GRADIENT_3D: [::Point3d<f32>, ..12] = [
    [ 0.70710678118,  0.70710678118, 0.0],
    [ 0.70710678118, -0.70710678118, 0.0],
    [-0.70710678118,  0.70710678118, 0.0],
    [-0.70710678118, -0.70710678118, 0.0],
    [ 0.70710678118, 0.0,  0.70710678118],
    [ 0.70710678118, 0.0, -0.70710678118],
    [-0.70710678118, 0.0,  0.70710678118],
    [-0.70710678118, 0.0, -0.70710678118],
    [0.0,  0.70710678118,  0.70710678118],
    [0.0,  0.70710678118, -0.70710678118],
    [0.0, -0.70710678118,  0.70710678118],
    [0.0, -0.70710678118, -0.70710678118],
];

const GRADIENT_4D: [::Point4d<f32>, ..32] = [
    [ 0.57735026919,  0.57735026919,  0.57735026919, 0.0],
    [ 0.57735026919, -0.57735026919,  0.57735026919, 0.0],
    [-0.57735026919,  0.57735026919,  0.57735026919, 0.0],
    [-0.57735026919, -0.57735026919,  0.57735026919, 0.0],
    [ 0.57735026919,  0.57735026919, -0.57735026919, 0.0],
    [ 0.57735026919, -0.57735026919, -0.57735026919, 0.0],
    [-0.57735026919,  0.57735026919, -0.57735026919, 0.0],
    [-0.57735026919, -0.57735026919, -0.57735026919, 0.0],
    [ 0.57735026919,  0.57735026919, 0.0,  0.57735026919],
    [ 0.57735026919, -0.57735026919, 0.0,  0.57735026919],
    [-0.57735026919,  0.57735026919, 0.0,  0.57735026919],
    [-0.57735026919, -0.57735026919, 0.0,  0.57735026919],
    [ 0.57735026919,  0.57735026919, 0.0, -0.57735026919],
    [ 0.57735026919, -0.57735026919, 0.0, -0.57735026919],
    [-0.57735026919,  0.57735026919, 0.0, -0.57735026919],
    [-0.57735026919, -0.57735026919, 0.0, -0.57735026919],
    [ 0.57735026919, 0.0,  0.57735026919,  0.57735026919],
    [ 0.57735026919, 0.0, -0.57735026919,  0.57735026919],
    [-0.57735026919, 0.0,  0.57735026919,  0.57735026919],
    [-0.57735026919, 0.0, -0.57735026919,  0.57735026919],
    [ 0.57735026919, 0.0,  0.57735026919, -0.57735026919],
    [ 0.57735026919, 0.0, -0.57735026919, -0.57735026919],
    [-0.57735026919, 0.0,  0.57735026919, -0.57735026919],
    [-0.57735026919, 0.0, -0.57735026919, -0.57735026919],
    [0.0,  0.57735026919,  0.57735026919,  0.57735026919],
    [0.0,  0.57735026919, -0.57735026919,  0.57735026919],
    [0.0, -0.57735026919,  0.57735026919,  0.57735026919],
    [0.0, -0.57735026919, -0.57735026919,  0.57735026919],
    [0.0,  0.57735026919,  0.57735026919, -0.57735026919],
    [0.0,  0.57735026919, -0.57735026919, -0.57735026919],
    [0.0, -0.57735026919,  0.57735026919, -0.57735026919],
    [0.0, -0.57735026919, -0.57735026919, -0.57735026919],
];

fn get_perlin2d_gradient(seed: &Seed, x_whole: int, y_whole: int, x_frac: f32, y_frac: f32) -> f32 {
    let vector = GRADIENT_2D[seed.get2d(x_whole, y_whole) % GRADIENT_2D.len()];
    return x_frac * vector[0] + y_frac * vector[1];
}

fn get_perlin3d_gradient(seed: &Seed, x_whole: int, y_whole: int, z_whole: int, x_frac: f32, y_frac: f32, z_frac: f32) -> f32 {
    let vector = GRADIENT_3D[seed.get3d(x_whole, y_whole, z_whole) % GRADIENT_3D.len()];
    return x_frac * vector[0] + y_frac * vector[1] + z_frac * vector[2];
}

fn get_perlin4d_gradient(seed: &Seed, x_whole: int, y_whole: int, z_whole: int, w_whole: int, x_frac: f32, y_frac: f32, z_frac: f32, w_frac: f32) -> f32 {
    let vector = GRADIENT_4D[seed.get4d(x_whole, y_whole, z_whole, w_whole) % GRADIENT_4D.len()];
    return x_frac * vector[0] + y_frac * vector[1] + z_frac * vector[2] + w_frac * vector[3];
}

fn perlin2d<T: Float>(seed: &Seed, point: &::Point2d<T>, quality: ::Quality) -> f32 {
    use Quality::{ Fast, Best };

    let xfloor = point[0].floor().to_f32().unwrap();
    let yfloor = point[1].floor().to_f32().unwrap();

    let x0_whole = xfloor.to_int().unwrap();
    let y0_whole = yfloor.to_int().unwrap();

    let x1_whole = x0_whole + 1;
    let y1_whole = y0_whole + 1;

    let x0_frac = point[0].to_f32().unwrap() - xfloor;
    let y0_frac = point[1].to_f32().unwrap() - yfloor;

    let x1_frac = x0_frac - 1.0;
    let y1_frac = y0_frac - 1.0;

    let (x_curve, y_curve) = match quality {
        Fast => (
            scurve3(x0_frac),
            scurve3(y0_frac),
        ),
        Best => (
            scurve5(x0_frac),
            scurve5(y0_frac),
        )
    };

    let n0 = get_perlin2d_gradient(seed, x0_whole, y0_whole, x0_frac, y0_frac);
    let n1 = get_perlin2d_gradient(seed, x1_whole, y0_whole, x1_frac, y0_frac);
    let interpolated_x0 = lerp(x_curve, n0, n1);

    let n0 = get_perlin2d_gradient(seed, x0_whole, y1_whole, x0_frac, y1_frac);
    let n1 = get_perlin2d_gradient(seed, x1_whole, y1_whole, x1_frac, y1_frac);
    let interpolated_x1 = lerp(x_curve, n0, n1);

    return lerp(y_curve, interpolated_x0, interpolated_x1);
}

pub fn perlin2d_fast<T: Float>(seed: &Seed, point: &::Point2d<T>) -> f32 {
    return perlin2d(seed, point, ::Quality::Fast);
}

pub fn perlin2d_best<T: Float>(seed: &Seed, point: &::Point2d<T>) -> f32 {
    return perlin2d(seed, point, ::Quality::Best);
}

fn perlin3d<T: Float>(seed: &Seed, point: &::Point3d<T>, quality: ::Quality) -> f32 {
    use Quality::{ Fast, Best };

    let xfloor = point[0].floor().to_f32().unwrap();
    let yfloor = point[1].floor().to_f32().unwrap();
    let zfloor = point[2].floor().to_f32().unwrap();

    let x0_whole = xfloor.to_int().unwrap();
    let y0_whole = yfloor.to_int().unwrap();
    let z0_whole = zfloor.to_int().unwrap();

    let x1_whole = x0_whole + 1;
    let y1_whole = y0_whole + 1;
    let z1_whole = z0_whole + 1;

    let x0_frac = point[0].to_f32().unwrap() - xfloor;
    let y0_frac = point[1].to_f32().unwrap() - yfloor;
    let z0_frac = point[2].to_f32().unwrap() - zfloor;

    let x1_frac = x0_frac - 1.0;
    let y1_frac = y0_frac - 1.0;
    let z1_frac = z0_frac - 1.0;

    let (x_curve, y_curve, z_curve) = match quality {
        Fast => (
            scurve3(x0_frac),
            scurve3(y0_frac),
            scurve3(z0_frac)
        ),
        Best => (
            scurve5(x0_frac),
            scurve5(y0_frac),
            scurve5(z0_frac)
        )
    };

    let n0 = get_perlin3d_gradient(seed, x0_whole, y0_whole, z0_whole, x0_frac, y0_frac, z0_frac);
    let n1 = get_perlin3d_gradient(seed, x1_whole, y0_whole, z0_whole, x1_frac, y0_frac, z0_frac);
    let interpolated_x0 = lerp(x_curve, n0, n1);

    let n0 = get_perlin3d_gradient(seed, x0_whole, y1_whole, z0_whole, x0_frac, y1_frac, z0_frac);
    let n1 = get_perlin3d_gradient(seed, x1_whole, y1_whole, z0_whole, x1_frac, y1_frac, z0_frac);
    let interpolated_x1 = lerp(x_curve, n0, n1);
    let interpolated_y0 = lerp(y_curve, interpolated_x0, interpolated_x1);

    let n0 = get_perlin3d_gradient(seed, x0_whole, y0_whole, z1_whole, x0_frac, y0_frac, z1_frac);
    let n1 = get_perlin3d_gradient(seed, x1_whole, y0_whole, z1_whole, x1_frac, y0_frac, z1_frac);
    let interpolated_x0 = lerp(x_curve, n0, n1);

    let n0 = get_perlin3d_gradient(seed, x0_whole, y1_whole, z1_whole, x0_frac, y1_frac, z1_frac);
    let n1 = get_perlin3d_gradient(seed, x1_whole, y1_whole, z1_whole, x1_frac, y1_frac, z1_frac);
    let interpolated_x1 = lerp(x_curve, n0, n1);
    let interpolated_y1 = lerp(y_curve, interpolated_x0, interpolated_x1);

    return lerp(z_curve, interpolated_y0, interpolated_y1);
}

pub fn perlin3d_fast<T: Float>(seed: &Seed, point: &::Point3d<T>) -> f32 {
    return perlin3d(seed, point, ::Quality::Fast);
}

pub fn perlin3d_best<T: Float>(seed: &Seed, point: &::Point3d<T>) -> f32 {
    return perlin3d(seed, point, ::Quality::Best);
}

fn perlin4d<T: Float>(seed: &Seed, point: &::Point4d<T>, quality: ::Quality) -> f32 {
    use Quality::{ Fast, Best };

    let xfloor = point[0].floor().to_f32().unwrap();
    let yfloor = point[1].floor().to_f32().unwrap();
    let zfloor = point[2].floor().to_f32().unwrap();
    let wfloor = point[3].floor().to_f32().unwrap();

    let x0_whole = xfloor.to_int().unwrap();
    let y0_whole = yfloor.to_int().unwrap();
    let z0_whole = zfloor.to_int().unwrap();
    let w0_whole = wfloor.to_int().unwrap();

    let x1_whole = x0_whole + 1;
    let y1_whole = y0_whole + 1;
    let z1_whole = z0_whole + 1;
    let w1_whole = w0_whole + 1;

    let x0_frac = point[0].to_f32().unwrap() - xfloor;
    let y0_frac = point[1].to_f32().unwrap() - yfloor;
    let z0_frac = point[2].to_f32().unwrap() - zfloor;
    let w0_frac = point[2].to_f32().unwrap() - wfloor;

    let x1_frac = x0_frac - 1.0;
    let y1_frac = y0_frac - 1.0;
    let z1_frac = z0_frac - 1.0;
    let w1_frac = w0_frac - 1.0;

    let (x_curve, y_curve, z_curve, w_curve) = match quality {
        Fast => (
            scurve3(x0_frac),
            scurve3(y0_frac),
            scurve3(z0_frac),
            scurve3(w0_frac)
        ),
        Best => (
            scurve5(x0_frac),
            scurve5(y0_frac),
            scurve5(z0_frac),
            scurve5(w0_frac)
        )
    };

    let n0 = get_perlin4d_gradient(seed, x0_whole, y0_whole, z0_whole, w0_whole, x0_frac, y0_frac, z0_frac, w0_frac);
    let n1 = get_perlin4d_gradient(seed, x1_whole, y0_whole, z0_whole, w0_whole, x1_frac, y0_frac, z0_frac, w0_frac);
    let interpolated_x0 = lerp(x_curve, n0, n1);

    let n0 = get_perlin4d_gradient(seed, x0_whole, y1_whole, z0_whole, w0_whole, x0_frac, y1_frac, z0_frac, w0_frac);
    let n1 = get_perlin4d_gradient(seed, x1_whole, y1_whole, z0_whole, w0_whole, x1_frac, y1_frac, z0_frac, w0_frac);
    let interpolated_x1 = lerp(x_curve, n0, n1);
    let interpolated_y0 = lerp(y_curve, interpolated_x0, interpolated_x1);

    let n0 = get_perlin4d_gradient(seed, x0_whole, y0_whole, z1_whole, w0_whole, x0_frac, y0_frac, z1_frac, w0_frac);
    let n1 = get_perlin4d_gradient(seed, x1_whole, y0_whole, z1_whole, w0_whole, x1_frac, y0_frac, z1_frac, w0_frac);
    let interpolated_x0 = lerp(x_curve, n0, n1);

    let n0 = get_perlin4d_gradient(seed, x0_whole, y1_whole, z1_whole, w0_whole, x0_frac, y1_frac, z1_frac, w0_frac);
    let n1 = get_perlin4d_gradient(seed, x1_whole, y1_whole, z1_whole, w0_whole, x1_frac, y1_frac, z1_frac, w0_frac);
    let interpolated_x1 = lerp(x_curve, n0, n1);
    let interpolated_y1 = lerp(y_curve, interpolated_x0, interpolated_x1);
    let interpolated_z0 = lerp(z_curve, interpolated_y0, interpolated_y1);

    let n0 = get_perlin4d_gradient(seed, x0_whole, y0_whole, z0_whole, w1_whole, x0_frac, y0_frac, z0_frac, w1_frac);
    let n1 = get_perlin4d_gradient(seed, x1_whole, y0_whole, z0_whole, w1_whole, x1_frac, y0_frac, z0_frac, w1_frac);
    let interpolated_x0 = lerp(x_curve, n0, n1);

    let n0 = get_perlin4d_gradient(seed, x0_whole, y1_whole, z0_whole, w1_whole, x0_frac, y1_frac, z0_frac, w1_frac);
    let n1 = get_perlin4d_gradient(seed, x1_whole, y1_whole, z0_whole, w1_whole, x1_frac, y1_frac, z0_frac, w1_frac);
    let interpolated_x1 = lerp(x_curve, n0, n1);
    let interpolated_y0 = lerp(y_curve, interpolated_x0, interpolated_x1);

    let n0 = get_perlin4d_gradient(seed, x0_whole, y0_whole, z1_whole, w1_whole, x0_frac, y0_frac, z1_frac, w1_frac);
    let n1 = get_perlin4d_gradient(seed, x1_whole, y0_whole, z1_whole, w1_whole, x1_frac, y0_frac, z1_frac, w1_frac);
    let interpolated_x0 = lerp(x_curve, n0, n1);

    let n0 = get_perlin4d_gradient(seed, x0_whole, y1_whole, z1_whole, w1_whole, x0_frac, y1_frac, z1_frac, w1_frac);
    let n1 = get_perlin4d_gradient(seed, x1_whole, y1_whole, z1_whole, w1_whole, x1_frac, y1_frac, z1_frac, w1_frac);
    let interpolated_x1 = lerp(x_curve, n0, n1);
    let interpolated_y1 = lerp(y_curve, interpolated_x0, interpolated_x1);
    let interpolated_z1 = lerp(z_curve, interpolated_y0, interpolated_y1);

    return lerp(w_curve, interpolated_z0, interpolated_z1);
}

pub fn perlin4d_fast<T: Float>(seed: &Seed, point: &::Point4d<T>) -> f32 {
    return perlin4d(seed, point, ::Quality::Fast);
}

pub fn perlin4d_best<T: Float>(seed: &Seed, point: &::Point4d<T>) -> f32 {
    return perlin4d(seed, point, ::Quality::Best);
}
