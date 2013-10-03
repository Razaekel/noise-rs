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
use std::rand::{IsaacRng, Rng};
use std::vec;

/// A perlin noise generator
pub struct Perlin {
    // The permutation table used for generating the noise values
    priv ptable: ~[u8/*, ..512*/],
}

impl Perlin {
    /// Create a new perlin noise generator using the default permutation
    /// table.
    ///
    /// # Example
    ///
    /// ~~~rust
    /// let perlin = Perlin::new();
    /// ~~~
    ///
    pub fn new() -> Perlin {
        Perlin { ptable: DEFAULT_PERMUTAIONS.to_owned() }
    }

    /// Create a new perlin noise generator using the given seed.
    ///
    /// # Example
    ///
    /// ~~~rust
    /// let perlin = Perlin::from_seed(std::rand::seed());
    /// let perlin = Perlin::from_seed("Bananas!".into_bytes());
    /// ~~~
    ///
    pub fn from_seed(seed: &[u8]) -> Perlin {
        Perlin::from_rng(&mut IsaacRng::new_seeded(seed))
    }

    /// Create a new perlin noise generator using the given random number
    /// generator to create the initial permutation table.
    ///
    /// # Example
    ///
    /// ~~~rust
    /// let perlin = Perlin::from_seed(&mut std::rand::weak_rng());
    /// ~~~
    ///
    #[inline]
    pub fn from_rng<R: Rng>(rng: &mut R) -> Perlin {
        Perlin { ptable: rng.shuffle(vec::from_fn(512, |i| i as u8)) }
    }

    /// Generate a new perlin noise value based on a given 1, 2 or
    /// 3-dimensional coordinte.
    ///
    /// `pos` can be of the following coordinate types, where `T: Float`:
    ///
    /// - `[T, ..1]`
    /// - `[T, ..2]`
    /// - `[T, ..3]`
    /// - `&[T, ..1]`
    /// - `&[T, ..2]`
    /// - `&[T, ..3]`
    ///
    /// # Examples
    ///
    /// ~~~rust
    /// let perlin = Perlin::new();
    /// let a = perlin.gen([1.0]);
    /// let b = perlin.gen([2.0, 3.0, 4.0]);
    ///
    /// let v = [3.0, 4.0];
    /// let c = perlin.gen(&v);
    /// ~~~
    ///
    #[inline]
    pub fn gen<T: Float, G: Gen<T>>(&self, pos: G) -> T {
        pos.gen(self)
    }
}

/// Internal trait used for implementing the perlin noise generation algorithm
/// for various coordinate types. The `Gen::gen` method should not be accessed
/// directly - use the generic `Perlin::gen` method instead.
trait Gen<T> {
    fn gen(&self, ctx: &Perlin) -> T;
}

impl<'self, T: Float> Gen<T> for &'self [T, ..1] {
    fn gen(&self, ctx: &Perlin) -> T {
        let X = self[0].floor().to_uint() as u8;

        let x = self[0] - self[0].floor();

        let u = fade(x.clone());

        let A  = ctx.ptable[X    ];
        let AA = ctx.ptable[A    ];
        let B  = ctx.ptable[X + 1];
        let BA = ctx.ptable[B    ];

        lerp(u.clone(), grad(ctx.ptable[AA], x.clone(), cast(0), cast(0)),
                        grad(ctx.ptable[BA], x-cast(1), cast(0), cast(0)))
    }
}

impl<'self, T: Float> Gen<T> for &'self [T, ..2] {
    fn gen(&self, ctx: &Perlin) -> T {
        let X = self[0].floor().to_uint() as u8;
        let Y = self[1].floor().to_uint() as u8;

        let x = self[0] - self[0].floor();
        let y = self[1] - self[1].floor();

        let u = fade(x.clone());
        let v = fade(y.clone());

        let A  = ctx.ptable[X    ] + Y;
        let AA = ctx.ptable[A    ];
        let AB = ctx.ptable[A + 1];
        let B  = ctx.ptable[X + 1] + Y;
        let BA = ctx.ptable[B    ];
        let BB = ctx.ptable[B + 1];

        lerp(v.clone(), lerp(u.clone(), grad(ctx.ptable[AA], x.clone(), y.clone(), cast(0)),
                                        grad(ctx.ptable[BA], x-cast(1), y.clone(), cast(0))),
                        lerp(u.clone(), grad(ctx.ptable[AB], x.clone(), y-cast(1), cast(0)),
                                        grad(ctx.ptable[BB], x-cast(1), y-cast(1), cast(0))))
    }
}

impl<'self, T: Float> Gen<T> for &'self [T, ..3] {
    fn gen(&self, ctx: &Perlin) -> T {
        // Find the unit cube that contains the point
        let X = self[0].floor().to_uint() as u8;
        let Y = self[1].floor().to_uint() as u8;
        let Z = self[2].floor().to_uint() as u8;

        // Find the relative X, Y, Z of point in the cube
        let x = self[0] - self[0].floor();
        let y = self[1] - self[1].floor();
        let z = self[2] - self[2].floor();

        // Compute the fade curves for X, Y, Z
        let u = fade(x.clone());
        let v = fade(y.clone());
        let w = fade(z.clone());

        // Hash coordinates of the 8 cube corners
        let A  = ctx.ptable[X    ] + Y;
        let AA = ctx.ptable[A    ] + Z;
        let AB = ctx.ptable[A + 1] + Z;
        let B  = ctx.ptable[X + 1] + Y;
        let BA = ctx.ptable[B    ] + Z;
        let BB = ctx.ptable[B + 1] + Z;

        // Add the blended results from the 8 corners of the cube
        lerp(w, lerp(v.clone(), lerp(u.clone(), grad(ctx.ptable[AA    ], x.clone(), y.clone(), z.clone()),
                                                grad(ctx.ptable[BA    ], x-cast(1), y.clone(), z.clone())),
                                lerp(u.clone(), grad(ctx.ptable[AB    ], x.clone(), y-cast(1), z.clone()),
                                                grad(ctx.ptable[BB    ], x-cast(1), y-cast(1), z.clone()))),
                lerp(v.clone(), lerp(u.clone(), grad(ctx.ptable[AA + 1], x.clone(), y.clone(), z-cast(1)),
                                                grad(ctx.ptable[BA + 1], x-cast(1), y.clone(), z-cast(1))),
                                lerp(u.clone(), grad(ctx.ptable[AB + 1], x.clone(), y-cast(1), z-cast(1)),
                                                grad(ctx.ptable[BB + 1], x-cast(1), y-cast(1), z-cast(1)))))
    }
}

impl<T: Float> Gen<T> for [T, ..1] {
    #[inline] fn gen(&self, ctx: &Perlin) -> T { ctx.gen(self) }
}

impl<T: Float> Gen<T> for [T, ..2] {
    #[inline] fn gen(&self, ctx: &Perlin) -> T { ctx.gen(self) }
}

impl<T: Float> Gen<T> for [T, ..3] {
    #[inline] fn gen(&self, ctx: &Perlin) -> T { ctx.gen(self) }
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

/// The default permutation table found at Ken Perlin's [Improved Noise page]
/// (http://mrl.nyu.edu/~perlin/noise/).
static DEFAULT_PERMUTAIONS: &'static [u8/*, ..512*/] = &'static [
    151, 160, 137,  91,  90,  15, 131,  13, 201,  95,  96,  53, 194, 233,   7, 225,
    140,  36, 103,  30,  69, 142,   8,  99,  37, 240,  21,  10,  23, 190,   6, 148,
    247, 120, 234,  75,   0,  26, 197,  62,  94, 252, 219, 203, 117,  35,  11,  32,
     57, 177,  33,  88, 237, 149,  56,  87, 174,  20, 125, 136, 171, 168,  68, 175,
     74, 165,  71, 134, 139,  48,  27, 166,  77, 146, 158, 231,  83, 111, 229, 122,
     60, 211, 133, 230, 220, 105,  92,  41,  55,  46, 245,  40, 244, 102, 143,  54,
     65,  25,  63, 161,   1, 216,  80,  73, 209,  76, 132, 187, 208,  89,  18, 169,
    200, 196, 135, 130, 116, 188, 159,  86, 164, 100, 109, 198, 173, 186,   3,  64,
     52, 217, 226, 250, 124, 123,   5, 202,  38, 147, 118, 126, 255,  82,  85, 212,
    207, 206,  59, 227,  47,  16,  58,  17, 182, 189,  28,  42, 223, 183, 170, 213,
    119, 248, 152,   2,  44, 154, 163,  70, 221, 153, 101, 155, 167,  43, 172,   9,
    129,  22,  39, 253,  19,  98, 108, 110,  79, 113, 224, 232, 178, 185, 112, 104,
    218, 246,  97, 228, 251,  34, 242, 193, 238, 210, 144,  12, 191, 179, 162, 241,
     81,  51, 145, 235, 249,  14, 239, 107,  49, 192, 214,  31, 181, 199, 106, 157,
    184,  84, 204, 176, 115, 121,  50,  45, 127,   4, 150, 254, 138, 236, 205,  93,
    222, 114,  67,  29,  24,  72, 243, 141, 128, 195,  78,  66, 215,  61, 156, 180,
    151, 160, 137,  91,  90,  15, 131,  13, 201,  95,  96,  53, 194, 233,   7, 225,
    140,  36, 103,  30,  69, 142,   8,  99,  37, 240,  21,  10,  23, 190,   6, 148,
    247, 120, 234,  75,   0,  26, 197,  62,  94, 252, 219, 203, 117,  35,  11,  32,
     57, 177,  33,  88, 237, 149,  56,  87, 174,  20, 125, 136, 171, 168,  68, 175,
     74, 165,  71, 134, 139,  48,  27, 166,  77, 146, 158, 231,  83, 111, 229, 122,
     60, 211, 133, 230, 220, 105,  92,  41,  55,  46, 245,  40, 244, 102, 143,  54,
     65,  25,  63, 161,   1, 216,  80,  73, 209,  76, 132, 187, 208,  89,  18, 169,
    200, 196, 135, 130, 116, 188, 159,  86, 164, 100, 109, 198, 173, 186,   3,  64,
     52, 217, 226, 250, 124, 123,   5, 202,  38, 147, 118, 126, 255,  82,  85, 212,
    207, 206,  59, 227,  47,  16,  58,  17, 182, 189,  28,  42, 223, 183, 170, 213,
    119, 248, 152,   2,  44, 154, 163,  70, 221, 153, 101, 155, 167,  43, 172,   9,
    129,  22,  39, 253,  19,  98, 108, 110,  79, 113, 224, 232, 178, 185, 112, 104,
    218, 246,  97, 228, 251,  34, 242, 193, 238, 210, 144,  12, 191, 179, 162, 241,
     81,  51, 145, 235, 249,  14, 239, 107,  49, 192, 214,  31, 181, 199, 106, 157,
    184,  84, 204, 176, 115, 121,  50,  45, 127,   4, 150, 254, 138, 236, 205,  93,
    222, 114,  67,  29,  24,  72, 243, 141, 128, 195,  78,  66, 215,  61, 156, 180,
];
