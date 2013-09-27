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

//! An implementation of Ken Perlin's [Improved Noise]
//! (http://mrl.nyu.edu/~perlin/noise/) algorithm.

use std::num::{cast, mul_add};
use std::rand::Rng;

pub trait Perlin<T> {
    fn perlin(&self, ctx: &PerlinContext) -> T;
}

// impl<T: Float> Perlin<T> for T {
//     fn perlin(&self, ctx: &PerlinContext) -> T {
//         ctx.gen1(self)
//     }
// }

impl<T: Float> Perlin<T> for (T,) {
    fn perlin(&self, ctx: &PerlinContext) -> T {
        match *self {
            (ref x,) => ctx.gen1(x)
        }
    }
}

impl<T: Float> Perlin<T> for (T, T) {
    fn perlin(&self, ctx: &PerlinContext) -> T {
        match *self {
            (ref x, ref y) => ctx.gen2(x, y)
        }
    }
}

impl<T: Float> Perlin<T> for (T, T, T) {
    fn perlin(&self, ctx: &PerlinContext) -> T {
        match *self {
            (ref x, ref y, ref z) => ctx.gen3(x, y, z)
        }
    }
}

impl<T: Float> Perlin<T> for [T, ..1] {
    fn perlin(&self, ctx: &PerlinContext) -> T {
        match *self {
            [ref x] => ctx.gen1(x),
        }
    }
}

impl<T: Float> Perlin<T> for [T, ..2] {
    fn perlin(&self, ctx: &PerlinContext) -> T {
        match *self {
            [ref x, ref y] => ctx.gen2(x, y),
        }
    }
}

impl<T: Float> Perlin<T> for [T, ..3] {
    fn perlin(&self, ctx: &PerlinContext) -> T {
        match *self {
            [ref x, ref y, ref z] => ctx.gen3(x, y, z),
        }
    }
}

/// A perlin noise generator
pub struct PerlinContext {
    // permutation table
    priv ptable: ~[u8]//[uint, ..512],
}

impl PerlinContext {
    pub fn new<R: Rng>(rng: &mut R) -> PerlinContext {
        PerlinContext { ptable: rng.gen_vec::<u8>(512) }
    }

    pub fn gen1<T:Float>(&self, x: &T) -> T {
        let X = x.floor().to_uint() as u8;
        let x_ = *x - x.floor();
        let u = fade(x_.clone());

        let A  = self.ptable[X    ];
        let AA = self.ptable[A    ];
        let B  = self.ptable[X + 1];
        let BA = self.ptable[B    ];

        lerp(u.clone(), grad(self.ptable[AA], x_.clone(), cast(0), cast(0)),
                        grad(self.ptable[BA], x_-cast(1), cast(0), cast(0)))
    }

    pub fn gen2<T:Float>(&self, x: &T, y: &T) -> T {
        let X = x.floor().to_uint() as u8;
        let Y = y.floor().to_uint() as u8;

        let x_ = *x - x.floor();
        let y_ = *y - y.floor();

        let u = fade(x_.clone());
        let v = fade(y_.clone());

        let A  = self.ptable[X    ] + Y;
        let AA = self.ptable[A    ];
        let AB = self.ptable[A + 1];
        let B  = self.ptable[X + 1] + Y;
        let BA = self.ptable[B    ];
        let BB = self.ptable[B + 1];

        lerp(v.clone(), lerp(u.clone(), grad(self.ptable[AA], x_.clone(), y_.clone(), cast(0)),
                                        grad(self.ptable[BA], x_-cast(1), y_.clone(), cast(0))),
                        lerp(u.clone(), grad(self.ptable[AB], x_.clone(), y_-cast(1), cast(0)),
                                        grad(self.ptable[BB], x_-cast(1), y_-cast(1), cast(0))))
    }

    pub fn gen3<T:Float>(&self, x: &T, y: &T, z: &T) -> T {
        // Find the unit cube that contains the point
        let X = x.floor().to_uint() as u8;
        let Y = y.floor().to_uint() as u8;
        let Z = z.floor().to_uint() as u8;

        // Find the relative X, Y, Z of point in the cube
        let x_ = *x - x.floor();
        let y_ = *y - y.floor();
        let z_ = *z - z.floor();

        // Compute the fade curves for X, Y, Z
        let u = fade(x_.clone());
        let v = fade(y_.clone());
        let w = fade(z_.clone());

        // Hash coordinates of the 8 cube corners
        let A  = self.ptable[X    ] + Y;
        let AA = self.ptable[A    ] + Z;
        let AB = self.ptable[A + 1] + Z;
        let B  = self.ptable[X + 1] + Y;
        let BA = self.ptable[B    ] + Z;
        let BB = self.ptable[B + 1] + Z;

        // Add the blended results from the 8 corners of the cube
        lerp(w, lerp(v.clone(), lerp(u.clone(), grad(self.ptable[AA    ], x_.clone(), y_.clone(), z_.clone()),
                                                grad(self.ptable[BA    ], x_-cast(1), y_.clone(), z_.clone())),
                                lerp(u.clone(), grad(self.ptable[AB    ], x_.clone(), y_-cast(1), z_.clone()),
                                                grad(self.ptable[BB    ], x_-cast(1), y_-cast(1), z_.clone()))),
                lerp(v.clone(), lerp(u.clone(), grad(self.ptable[AA + 1], x_.clone(), y_.clone(), z_-cast(1)),
                                                grad(self.ptable[BA + 1], x_-cast(1), y_.clone(), z_-cast(1))),
                                lerp(u.clone(), grad(self.ptable[AB + 1], x_.clone(), y_-cast(1), z_-cast(1)),
                                                grad(self.ptable[BB + 1], x_-cast(1), y_-cast(1), z_-cast(1)))))
    }
}

#[inline]
fn fade<T: Float>(t: T) -> T {
    t * t * t * (t * (t * cast(6) - cast(15)) + cast(10))
}

#[inline]
fn lerp<T: Float>(t: T, a: T, b: T) -> T {
    mul_add(t, b - a, a)
}

fn grad<T: Float>(hash: u8, x: T, y: T, z: T) -> T {
    let h = hash & 15;

    let u = match h {
        0..7 => x.clone(),
        _    => y.clone(),
    };
    let v = match h {
        0..3    => y.clone(),
        12 | 14 => x.clone(),
        _       => z.clone(),
    };

    (if (h & 1) == 0 { u } else { -u }) +
    (if (h & 2) == 0 { v } else { -v })
}

