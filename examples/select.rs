// Copyright 2013 The Noise-rs Developers.
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

use noise::modules::*;

mod debug;

fn main() {
    let checkerboard = Checkerboard::new(0);
    let cylinders = Cylinders::new(1.0);
    let perlin = Perlin::new(0);
    let constant = Constant::new(0.5);
    let select1 = Select::new(perlin, cylinders, checkerboard, 0.5, 0.0, 1.0);
    let select2 = Select::new(perlin, constant, checkerboard, 0.0, 0.0, 1.0);

    debug::render_png2("select1.png", select1, 1024, 1024, 100);
    debug::render_png2("select2.png", select2, 1024, 1024, 100);
}
