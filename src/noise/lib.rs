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

#[crate_id = "noise#0.1"];
#[comment = "Procedural noise generation library."];
#[license = "ASL2"];
#[crate_type = "rlib"];
#[crate_type = "dylib"];

extern crate rand;

mod gen;
pub mod util;

pub mod sources {
    pub use self::perlin::Perlin;

    pub mod perlin;
}

/// A source of noise values.
pub trait Source {
    /// For the given coordinate, return the value
    /// The value is between -1 and 1
    fn get<F:Float>(&self, x: F, y: F, z: F) -> F;
}

pub enum Quality {
    Fast,
    Standard,
    Best
}
