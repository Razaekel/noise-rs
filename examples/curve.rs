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

extern crate noise;

use noise::{Curve, Perlin};

mod debug;

fn main() {
    let perlin = Perlin::new();
    let curve = Curve::new(perlin)
        .add_control_point(-2.0, -2.0)
        .add_control_point(-1.0, -1.25)
        .add_control_point(0.0, -0.75)
        .add_control_point(0.5, -0.25)
        .add_control_point(0.625, 0.875)
        .add_control_point(0.75, 1.0)
        .add_control_point(2.0, 1.25);

    debug::render_noise_module3("curve.png", curve, 1024, 1024, 100);
}
