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

//! An example of using perlin noise

extern crate noise;

use noise::{RangeFunction, Worley};

mod debug;

fn main() {
    debug::render_noise_module("worley-linear.png", Worley::new(), 1024, 1024, 50);
    debug::render_noise_module("worley-linear-range.png",
                               Worley::new().enable_range(true),
                               1024,
                               1024,
                               50);
    debug::render_noise_module("worley-linear-squared.png",
                               Worley::new().set_range_function(RangeFunction::EuclideanSquared),
                               1024,
                               1024,
                               50);
    debug::render_noise_module("worley-manhattan.png",
                               Worley::new().set_range_function(RangeFunction::Manhattan),
                               1024,
                               1024,
                               50);
    debug::render_noise_module("worley-manhattan-range.png",
                               Worley::new()
                                   .enable_range(true)
                                   .set_range_function(RangeFunction::Manhattan),
                               1024,
                               1024,
                               50);
    debug::render_noise_module("worley-chebyshev.png",
                               Worley::new().set_range_function(RangeFunction::Chebyshev),
                               1024,
                               1024,
                               50);
    debug::render_noise_module("worley-chebyshev-range.png",
                               Worley::new()
                                   .enable_range(true)
                                   .set_range_function(RangeFunction::Chebyshev),
                               1024,
                               1024,
                               50);
    debug::render_noise_module("worley-quadratic.png",
                               Worley::new().set_range_function(RangeFunction::Quadratic),
                               1024,
                               1024,
                               50);
    debug::render_noise_module("worley-quadratic-range.png",
                               Worley::new()
                                   .enable_range(true)
                                   .set_range_function(RangeFunction::Quadratic),
                               1024,
                               1024,
                               50);
}
