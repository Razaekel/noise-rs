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

use std::num::{cast, Float};

use seed::Seed;

pub fn brownian2d<T, F>(seed: &Seed, point: &::Point2d<T>, noise_func: F, wavelength: f32, octaves: u32) -> f32
    where T: Float, F: Fn(&Seed, &::Point2d<T>) -> f32
{
    let mut frequency = 1.0 / wavelength;
    let mut amplitude = 1.0;
    let mut result = 0.0;
    for _ in range(0, octaves) {
        let scaled_point = [point[0] * cast(frequency).unwrap(), point[1] * cast(frequency).unwrap()];
        result += noise_func(seed, &scaled_point) * amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    return result;
}

pub fn brownian3d<T, F>(seed: &Seed, point: &::Point3d<T>, noise_func: F, wavelength: f32, octaves: u32) -> f32
    where T: Float, F: Fn(&Seed, &::Point3d<T>) -> f32
{
    let mut frequency = 1.0 / wavelength;
    let mut amplitude = 1.0;
    let mut result = 0.0;
    for _ in range(0, octaves) {
        let scaled_point = [point[0] * cast(frequency).unwrap(), point[1] * cast(frequency).unwrap(), point[2] * cast(frequency).unwrap()];
        result += noise_func(seed, &scaled_point) * amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    return result;
}

pub fn brownian4d<T, F>(seed: &Seed, point: &::Point4d<T>, noise_func: F, wavelength: f32, octaves: u32) -> f32
    where T: Float, F: Fn(&Seed, &::Point4d<T>) -> f32
{
    let mut frequency = 1.0 / wavelength;
    let mut amplitude = 1.0;
    let mut result = 0.0;
    for _ in range(0, octaves) {
        let scaled_point = [point[0] * cast(frequency).unwrap(), point[1] * cast(frequency).unwrap(), point[2] * cast(frequency).unwrap(), point[3] * cast(frequency).unwrap()];
        result += noise_func(seed, &scaled_point) * amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    return result;
}
