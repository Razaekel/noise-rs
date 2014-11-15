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

use util::lerp;
use source::Source;

pub struct Line<'a, S:'a, F> {
    start: [F,..3],
    end: [F,..3],
    source: &'a S,
}

impl<'a, S:Source, F:Float> Line<'a, S, F> {

    pub fn new(src: &'a S) -> Line<'a, S, F> {
        Line {
            start: [Float::zero(), Float::zero(), Float::zero()],
            end: [Float::one(), Float::one(), Float::one()],
            source: src,
        }
    }

    pub fn get(&self, pos: F) -> F {
        let x = lerp(pos.clone(), self.start[0].clone(), self.end[0].clone());
        let y = lerp(pos.clone(), self.start[1].clone(), self.end[1].clone());
        let z = lerp(pos.clone(), self.start[2].clone(), self.end[2].clone());

        self.source.get(x, y, z)
    }
}

pub struct Plane<'a, S:'a> {
    source: &'a S
}

impl<'a, S:Source> Plane<'a, S> {
    pub fn new(src: &'a S) -> Plane<'a, S> {
        Plane { source: src}
    }

    pub fn get<F:Float>(&self, x: F, y: F) -> F {
        self.source.get(x, y, Float::zero())
    }
}
