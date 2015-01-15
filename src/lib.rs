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

#![feature(unboxed_closures)]
#![deny(missing_copy_implementations)]
#![allow(unstable)]

pub use seed::Seed;
pub use math::{Point2, Point3, Point4};
pub use perlin::{perlin2, perlin3, perlin4};
pub use open_simplex::{open_simplex2, open_simplex3};
pub use brownian::{Brownian2, Brownian3, Brownian4};

pub use cell::{range_sqr_euclidian2, range_sqr_euclidian3, range_sqr_euclidian4};
pub use cell::{cell2_seed_point, cell3_seed_point, cell4_seed_point};
pub use cell::{cell2_range, cell3_range, cell4_range};
pub use cell::{cell2_range_inv, cell3_range_inv, cell4_range_inv};

mod gradient;
mod math;
mod seed;

mod brownian;
mod perlin;
mod open_simplex;
mod cell;
