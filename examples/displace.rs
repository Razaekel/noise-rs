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

use noise::{Checkerboard, Constant, Cylinders, Displace, Perlin};

mod debug;

fn main() {
    let cboard = Checkerboard::new();
    let constant = Constant::new(0.0);
    let cylinders = Cylinders::new();
    let perlin = Perlin::new();
    let displace = Displace::new(cylinders, cboard, perlin, constant, constant);

    debug::render_noise_module("displace.png", &displace, 1024, 1024, 50);
}
