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

pub trait Simplex<T> {
    fn simplex(&self, ctx: &SimplexContext<T>) -> T;
}

impl<T: Float> Simplex<T> for (T, T) {
    fn simplex(&self, ctx: &SimplexContext<T>) -> T {
        match *self {
            (ref x, ref y) => ctx.gen2(x, y)
        }
    }
}

impl<T: Float> Simplex<T> for (T, T, T) {
    fn simplex(&self, ctx: &SimplexContext<T>) -> T {
        match *self {
            (ref x, ref y, ref z) => ctx.gen3(x, y, z)
        }
    }
}

impl<T: Float> Simplex<T> for (T, T, T, T) {
    fn simplex(&self, ctx: &SimplexContext<T>) -> T {
        match *self {
            (ref x, ref y, ref z, ref w) => ctx.gen4(x, y, z, w)
        }
    }
}

impl<T: Float> Simplex<T> for [T, ..2] {
    fn simplex(&self, ctx: &SimplexContext<T>) -> T {
        match *self {
            [ref x, ref y] => ctx.gen2(x, y),
        }
    }
}

impl<T: Float> Simplex<T> for [T, ..3] {
    fn simplex(&self, ctx: &SimplexContext<T>) -> T {
        match *self {
            [ref x, ref y, ref z] => ctx.gen3(x, y, z),
        }
    }
}

impl<T: Float> Simplex<T> for [T, ..4] {
    fn simplex(&self, ctx: &SimplexContext<T>) -> T {
        match *self {
            [ref x, ref y, ref z, ref w] => ctx.gen4(x, y, z, w),
        }
    }
}

pub struct SimplexContext<T>;

impl<T> SimplexContext<T> {
    pub fn new() -> SimplexContext<T> {
        fail!("Not yet implemented!")
    }

    pub fn gen2(&self, _x: &T, _y: &T) -> T {
        fail!("Not yet implemented!")
    }

    pub fn gen3(&self, _x: &T, _y: &T, _x: &T) -> T {
        fail!("Not yet implemented!")
    }

    pub fn gen4(&self, _x: &T, _y: &T, _z: &T, _w: &T) -> T {
        fail!("Not yet implemented!")
    }
}
