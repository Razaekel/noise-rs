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

use std::num::cast;

pub trait Perlin<T> {
    fn perlin(&self, ctx: &PerlinContext<T>) -> T;
}

// impl<T: Clone + Float> Perlin<T> for T {
//     fn perlin(&self, ctx: &PerlinContext<T>) -> T {
//         ctx.gen1(self)
//     }
// }

impl<T: Clone + Float> Perlin<T> for (T,) {
    fn perlin(&self, ctx: &PerlinContext<T>) -> T {
        match *self {
            (ref x,) => ctx.gen1(x)
        }
    }
}

impl<T: Clone + Float> Perlin<T> for (T, T) {
    fn perlin(&self, ctx: &PerlinContext<T>) -> T {
        match *self {
            (ref x, ref y) => ctx.gen2(x,
                                       y)
        }
    }
}

impl<T: Clone + Float> Perlin<T> for (T, T, T) {
    fn perlin(&self, ctx: &PerlinContext<T>) -> T {
        match *self {
            (ref x, ref y, ref z) => ctx.gen3(x, y, z)
        }
    }
}

impl<T: Clone + Float> Perlin<T> for [T, ..1] {
    fn perlin(&self, ctx: &PerlinContext<T>) -> T {
        match *self {
            [ref x] => ctx.gen1(x),
        }
    }
}

impl<T: Clone + Float> Perlin<T> for [T, ..2] {
    fn perlin(&self, ctx: &PerlinContext<T>) -> T {
        match *self {
            [ref x, ref y] => ctx.gen2(x, y),
        }
    }
}

impl<T: Clone + Float> Perlin<T> for [T, ..3] {
    fn perlin(&self, ctx: &PerlinContext<T>) -> T {
        match *self {
            [ref x, ref y, ref z] => ctx.gen3(x, y, z),
        }
    }
}

/// A perlin noise generator
pub struct PerlinContext<T> {
    // permutation table
    priv ptable: [uint, ..512],
}

impl<T:Clone + Float> PerlinContext<T> {
    pub fn new() -> PerlinContext<T> {
        // TODO: generate random permutation table
        PerlinContext { ptable: P }
    }

    pub fn gen1(&self, x: &T) -> T {
        self.gen2(x, &cast(0))
    }

    pub fn gen2(&self, x: &T, y: &T) -> T {
        self.gen3(x, y, &cast(0))
    }

    pub fn gen3(&self, x: &T, y: &T, z: &T) -> T {
        // Find the unit cube that contains the point
        let X = x.floor().to_uint() & 255;
        let Y = y.floor().to_uint() & 255;
        let Z = z.floor().to_uint() & 255;

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

fn fade<T:Num + NumCast>(t: T) -> T {
    t * t * t * (t * (t * cast(6) - cast(15)) + cast(10))
}

fn lerp<T:Num + NumCast>(t: T, a: T, b: T) -> T {
    a + t * (b - a)
}

fn grad<T:Clone + Num + NumCast>(hash: uint, x: T, y: T, z: T) -> T {
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

/// Permutation table
static P: [uint, ..512] = [
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
