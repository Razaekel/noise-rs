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

//! An example of using simplex noise

extern crate noise;

use noise::{Curve, OpenSimplex, Seedable};

mod debug;

fn main() {
    debug::render_noise_module2("open_simplex_scaled2.png", &Curve::new(&OpenSimplex::new()).add_control_point(-1.0, -1.0).add_control_point(-0.2, -1.0).add_control_point(0.2, 1.0).add_control_point(1.0, 1.0), 1024, 1024, 128);
    debug::render_noise_module3("open_simplex_scaled3.png", &Curve::new(&OpenSimplex::new()).add_control_point(-1.0, -1.0).add_control_point(-0.2, -1.0).add_control_point(0.2, 1.0).add_control_point(1.0, 1.0), 1024, 1024, 128);
    debug::render_noise_module4("open_simplex_scaled4.png", &Curve::new(&OpenSimplex::new()).add_control_point(-1.0, -1.0).add_control_point(-0.2, -1.0).add_control_point(0.2, 1.0).add_control_point(1.0, 1.0), 1024, 1024, 128);
    debug::render_noise_module2("open_simplex2.png", &OpenSimplex::new(), 1024, 1024, 50);
    debug::render_noise_module3("open_simplex3.png", &OpenSimplex::new(), 1024, 1024, 50);
    debug::render_noise_module4("open_simplex4.png", &OpenSimplex::new(), 1024, 1024, 50);
    debug::render_noise_module2("open_simplex_seeded2.png", &OpenSimplex::new().set_seed(1), 1024, 1024, 50);
    debug::render_noise_module3("open_simplex_seeded3.png", &OpenSimplex::new().set_seed(1), 1024, 1024, 50);
    debug::render_noise_module4("open_simplex_seeded4.png", &OpenSimplex::new().set_seed(1), 1024, 1024, 50);
}
