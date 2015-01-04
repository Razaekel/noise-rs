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

#![feature(macro_rules)]
#![feature(unboxed_closures)]
#![deny(missing_copy_implementations)]

pub use seed::Seed;
pub use math::{Point2, Point3, Point4};
pub use perlin::{perlin2, perlin3, perlin4};
pub use simplex::{simplex2, simplex3};
pub use simplectic::{simplectic2, simplectic3, simplectic4};
pub use brownian::{Brownian2, Brownian3, Brownian4};

mod gen;
mod gradient;

mod math;
pub mod source;

mod seed;
mod perlin;
mod simplex;
mod simplectic;
mod brownian;

#[derive(Copy, Clone)]
#[deprecated = "It has been determined that this parameter does little to affect the speed of the algorithm."]
pub enum Quality {
    Fast,
    Best,
}
