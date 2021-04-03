use crate::{
    gradient, math,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::{NoiseHasher, PermutationTable},
};

/// Noise function that outputs N-dimensional Simplex noise.
///
/// The simplex noise code was adapted from code by Stefan Gustavson,
/// http://staffwww.itn.liu.se/~stegu/aqsis/aqsis-newnoise/sdnoise1234.c
///
/// This was Stefan Gustavson's original copyright notice:
///
/// /* sdnoise1234, Simplex noise with true analytic
///  * derivative in 1D to 4D.
///  *
///  * Copyright © 2003-2011, Stefan Gustavson
///  *
///  * Contact: stefan.gustavson@gmail.com
///  *
///  * This library is public domain software, released by the author
///  * into the public domain in February 2011. You may do anything
///  * you like with it. You may even remove all attributions,
///  * but of course I'd appreciate it if you kept my name somewhere.
///  *
///  * This library is distributed in the hope that it will be useful,
///  * but WITHOUT ANY WARRANTY; without even the implied warranty of
/// * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
///  * General Public License for more details.
///  */
#[derive(Clone, Copy, Debug)]
pub struct Simplex {
    seed: u32,
    hasher: PermutationTable,
}

impl Simplex {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new(seed: u32) -> Self {
        Simplex {
            seed,
            hasher: PermutationTable::new(seed),
        }
    }
}

impl Default for Simplex {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl Seedable for Simplex {
    /// Sets the seed value for Simplex noise
    fn set_seed(self, seed: u32) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Simplex {
            seed,
            hasher: PermutationTable::new(seed),
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

fn grad1(hash: u8) -> f64 {
    let h = hash % 15;
    let gx = (1 + (h % 7)) as f64; // Gradient value is one of 1.0, 2.0, ..., 8.0
    match h % 8 {
        0 => -gx,
        1 => gx, // Make half of the gradients negative
        _ => unreachable!(),
    }
}

// Skew Value
//
//     sqrt(n + 1) - 1
// F = ---------------
//            n
pub fn skew_factor(n: usize) -> f64 {
    let n = n as f64;

    ((n + 1.0).sqrt() - 1.0) / n
}

//  Unskew Value
//
//     1 - 1 / sqrt(n + 1)
// G = -------------------
//             n
pub fn unskew_factor(n: usize) -> f64 {
    let n = n as f64;

    (1.0 - (1.0 / (n + 1.0).sqrt())) / n
}

/*
 * Permutation table. This is just a random jumble of all numbers 0-255,
 * repeated twice to avoid wrapping the index at 255 for each lookup.
 */
const PERM: [u8; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

/// 1D Simplex Noise with Derivative
pub fn simplex_1d(x: f64, with_derivatives: bool) -> (f64, Option<f64>) {
    let i0 = x.floor() as isize;
    let i1 = i0 + 1;

    let x0 = x - i0 as f64;
    let x1 = x0 - 1.0;

    let (mut gx0, mut gx1): (f64, f64) = (0.0, 0.0);
    let (n0, n1);
    let (t20, t40, t21, t41);

    let x20 = x0 * x0;
    let t0 = 1.0 - x20;
    //  if(t0 < 0.0) t0 = 0.0; // Never happens for 1D: x0<=1 always
    t20 = t0 * t0;
    t40 = t20 * t20;
    gx0 = grad1(PERM[(i0 % 0xff) as usize]);
    n0 = t40 * gx0 * x0;

    let x21 = x1 * x1;
    let t1 = 1.0 - x21;
    //  if(t1 < 0.0) t1 = 0.0; // Never happens for 1D: |x1|<=1 always
    t21 = t1 * t1;
    t41 = t21 * t21;
    gx1 = grad1(PERM[(i1 % 0xff) as usize]);
    n1 = t41 * gx1 * x1;

    let noise = 0.25 * (n0 + n1);

    if !with_derivatives {
        (noise, None)
    } else {
        /* Compute derivative according to:
         *  dnoise_dx = -8.0 * t20 * t0 * x0 * (gx0 * x0) + t40 * gx0;
         *  dnoise_dx += -8.0 * t21 * t1 * x1 * (gx1 * x1) + t41 * gx1;
         */
        let mut dnoise_dx = t20 * t0 * gx0 * x20;
        dnoise_dx += t21 * t1 * gx1 * x21;
        dnoise_dx *= -8.0;
        dnoise_dx += t40 * gx0 + t41 * gx1;
        dnoise_dx *= 0.25; /* Scale derivative to match the noise scaling */

        // The maximum value of this noise is 8*(3/4)^4 = 2.53125
        // A factor of 0.395 would scale to fit exactly within [-1,1], but
        // to better match classic Perlin noise, we scale it down some more.
        (noise, Some(dnoise_dx))
    }
}

pub fn simplex_2d(x: f64, y: f64, with_derivatives: bool) -> (f64, Option<[f64; 2]>) {
    let [n0, n1, n2]: [f64; 3]; /* Noise contributions from the three simplex corners */
    let [mut gx0, mut gy0, mut gx1, mut gy1, mut gx2, mut gy2]: [f64; 6] = [0.0; 6]; /* Gradients at simplex corners */

    let f2: f64 = skew_factor(2);
    let g2: f64 = unskew_factor(2);

    /* Skew the input space to determine which simplex cell we're in */
    let s = (x + y) * f2; /* Hairy factor for 2D */
    let xs = x + s;
    let ys = y + s;
    let i = xs.floor() as isize;
    let j = ys.floor() as isize;

    let t = (i + j) as f64 * g2;
    let X0 = i as f64 - t; /* Unskew the cell origin back to (x,y) space */
    let Y0 = j as f64 - t;
    let x0 = x - X0; /* The x,y distances from the cell origin */
    let y0 = y - Y0;

    /* For the 2D case, the simplex shape is an equilateral triangle.
     * Determine which simplex we are in. */
    let (i1, j1): (u8, u8); /* Offsets for second (middle) corner of simplex in (i,j) coords */
    if x0 > y0 {
        i1 = 1;
        j1 = 0;
    }
    /* lower triangle, XY order: (0,0)->(1,0)->(1,1) */
    else {
        i1 = 0;
        j1 = 1;
    } /* upper triangle, YX order: (0,0)->(0,1)->(1,1) */

    /* A step of (1,0) in (i,j) means a step of (1-c,-c) in (x,y), and
     * a step of (0,1) in (i,j) means a step of (-c,1-c) in (x,y), where
     * c = (3-sqrt(3))/6   */
    let x1 = x0 - i1 as f64 + g2; /* Offsets for middle corner in (x,y) unskewed coords */
    let y1 = y0 - j1 as f64 + g2;
    let x2 = x0 - 1.0 + 2.0 * g2; /* Offsets for last corner in (x,y) unskewed coords */
    let y2 = y0 - 1.0 + 2.0 * g2;

    /* Wrap the integer indices at 256, to avoid indexing PERM[] out of bounds */
    let ii = (i & 0xff) as u8;
    let jj = (j & 0xff) as u8;

    /* Calculate the contribution from the three corners */
    let mut t0 = 0.5 - x0 * x0 - y0 * y0;
    let (t20, t40): (f64, f64);
    if t0 < 0.0 {
        t40 = 0.0;
        t20 = 0.0;
        t0 = 0.0;
        n0 = 0.0;
        gx0 = 0.0;
        gy0 = 0.0; /* No influence */
    } else {
        {
            let [lhs0, lhs1] = gradient::grad2((PERM[(ii + PERM[jj as usize]) as usize]) as usize);
            gx0 = lhs0;
            gy0 = lhs1;
        }
        t20 = t0 * t0;
        t40 = t20 * t20;
        n0 = t40 * (gx0 * x0 + gy0 * y0);
    }

    let mut t1 = 0.5 - x1 * x1 - y1 * y1;
    let (t21, t41): (f64, f64);
    if t1 < 0.0 {
        t21 = 0.0;
        t41 = 0.0;
        t1 = 0.0;
        n1 = 0.0;
        gx1 = 0.0;
        gy1 = 0.0; /* No influence */
    } else {
        {
            let [lhs0, lhs1] =
                gradient::grad2((PERM[(ii + i1 + PERM[(jj + j1) as usize]) as usize]) as usize);
            gx1 = lhs0;
            gy1 = lhs1;
        }
        t21 = t1 * t1;
        t41 = t21 * t21;
        n1 = t41 * (gx1 * x1 + gy1 * y1);
    }

    let mut t2 = 0.5 - x2 * x2 - y2 * y2;
    let (t22, t42): (f64, f64);
    if t2 < 0.0 {
        t22 = 0.0;
        t42 = 0.0;
        t2 = 0.0;
        n2 = 0.0;
        gx2 = 0.0;
        gy2 = 0.0; /* No influence */
    } else {
        {
            let [lhs0, lhs1] =
                gradient::grad2((PERM[(ii + 1 + PERM[(jj + 1) as usize]) as usize]) as usize);
            gx2 = lhs0;
            gy2 = lhs1;
        }
        t22 = t2 * t2;
        t42 = t22 * t22;
        n2 = t42 * (gx2 * x2 + gy2 * y2);
    }

    /* Add contributions from each corner to get the final noise value.
     * The result is scaled to return values in the interval [-1, 1]. */
    let noise = 40.0 * (n0 + n1 + n2);

    if !with_derivatives {
        (noise, None)
    } else {
        /*  A straight, unoptimised calculation would be like:
         *    dnoise_dx = -8.0 * t20 * t0 * x0 * ( gx0 * x0 + gy0 * y0 ) + t40 * gx0;
         *    dnoise_dy = -8.0 * t20 * t0 * y0 * ( gx0 * x0 + gy0 * y0 ) + t40 * gy0;
         *    dnoise_dx += -8.0 * t21 * t1 * x1 * ( gx1 * x1 + gy1 * y1 ) + t41 * gx1;
         *    dnoise_dy += -8.0 * t21 * t1 * y1 * ( gx1 * x1 + gy1 * y1 ) + t41 * gy1;
         *    dnoise_dx += -8.0 * t22 * t2 * x2 * ( gx2 * x2 + gy2 * y2 ) + t42 * gx2;
         *    dnoise_dy += -8.0 * t22 * t2 * y2 * ( gx2 * x2 + gy2 * y2 ) + t42 * gy2;
         */
        let temp0 = t20 * t0 * (gx0 * x0 + gy0 * y0);
        let mut dnoise_dx = temp0 * x0;
        let mut dnoise_dy = temp0 * y0;
        let temp1 = t21 * t1 * (gx1 * x1 + gy1 * y1);
        dnoise_dx += temp1 * x1;
        dnoise_dy += temp1 * y1;
        let temp2 = t22 * t2 * (gx2 * x2 + gy2 * y2);
        dnoise_dx += temp2 * x2;
        dnoise_dy += temp2 * y2;
        dnoise_dx *= -8.0;
        dnoise_dy *= -8.0;
        dnoise_dx += t40 * gx0 + t41 * gx1 + t42 * gx2;
        dnoise_dy += t40 * gy0 + t41 * gy1 + t42 * gy2;
        dnoise_dx *= 40.0; /* Scale derivative to match the noise scaling */
        dnoise_dy *= 40.0;

        (noise, Some([dnoise_dx, dnoise_dy]))
    }
}

pub fn simplex_3d(x: f64, y: f64, z: f64, with_derivatives: bool) -> (f64, Option<[f64; 3]>) {
    let [n0, n1, n2, n3]: [f64; 4]; /* Noise contributions from the four simplex corners */
    let [gx0, gy0, gz0, gx1, gy1, gz1]: [f64; 6]; /* Gradients at simplex corners */
    let [gx2, gy2, gz2, gx3, gy3, gz3]: [f64; 6];

    let f3 = skew_factor(3);
    let g3 = unskew_factor(3);

    /* Skew the input space to determine which simplex cell we're in */
    let s = (x + y + z) * f3; /* Very nice and simple skew factor for 3D */
    let xs = x + s;
    let ys = y + s;
    let zs = z + s;
    let i = xs.floor() as isize;
    let j = ys.floor() as isize;
    let k = zs.floor() as isize;

    let t = (i + j + k) as f64 * g3;
    let X0 = i as f64 - t; /* Unskew the cell origin back to (x,y,z) space */
    let Y0 = j as f64 - t;
    let Z0 = k as f64 - t;
    let x0 = x - X0; /* The x,y,z distances from the cell origin */
    let y0 = y - Y0;
    let z0 = z - Z0;

    /* For the 3D case, the simplex shape is a slightly irregular tetrahedron.
     * Determine which simplex we are in. */
    let [i1, j1, k1]: [usize; 3]; /* Offsets for second corner of simplex in (i,j,k) coords */
    let [i2, j2, k2]: [usize; 3]; /* Offsets for third corner of simplex in (i,j,k) coords */

    /* TODO: This code would benefit from a backport from the GLSL version! */
    if x0 >= y0 {
        if y0 >= z0 {
            i1 = 1;
            j1 = 0;
            k1 = 0;
            i2 = 1;
            j2 = 1;
            k2 = 0;
        }
        /* X Y Z order */
        else if x0 >= z0 {
            i1 = 1;
            j1 = 0;
            k1 = 0;
            i2 = 1;
            j2 = 0;
            k2 = 1;
        }
        /* X Z Y order */
        else {
            i1 = 0;
            j1 = 0;
            k1 = 1;
            i2 = 1;
            j2 = 0;
            k2 = 1;
        } /* Z X Y order */
    } else {
        // x0<y0
        if y0 < z0 {
            i1 = 0;
            j1 = 0;
            k1 = 1;
            i2 = 0;
            j2 = 1;
            k2 = 1;
        }
        /* Z Y X order */
        else if x0 < z0 {
            i1 = 0;
            j1 = 1;
            k1 = 0;
            i2 = 0;
            j2 = 1;
            k2 = 1;
        }
        /* Y Z X order */
        else {
            i1 = 0;
            j1 = 1;
            k1 = 0;
            i2 = 1;
            j2 = 1;
            k2 = 0;
        } /* Y X Z order */
    }

    /* A step of (1,0,0) in (i,j,k) means a step of (1-c,-c,-c) in (x,y,z),
     * a step of (0,1,0) in (i,j,k) means a step of (-c,1-c,-c) in (x,y,z), and
     * a step of (0,0,1) in (i,j,k) means a step of (-c,-c,1-c) in (x,y,z), where
     * c = 1/6.   */

    let x1 = x0 - i1 as f64 + g3; /* Offsets for second corner in (x,y,z) coords */
    let y1 = y0 - j1 as f64 + g3;
    let z1 = z0 - k1 as f64 + g3;
    let x2 = x0 - i2 as f64 + 2.0 * g3; /* Offsets for third corner in (x,y,z) coords */
    let y2 = y0 - j2 as f64 + 2.0 * g3;
    let z2 = z0 - k2 as f64 + 2.0 * g3;
    let x3 = x0 - 1.0 + 3.0 * g3; /* Offsets for last corner in (x,y,z) coords */
    let y3 = y0 - 1.0 + 3.0 * g3;
    let z3 = z0 - 1.0 + 3.0 * g3;

    /* Wrap the integer indices at 256, to avoid indexing PERM[] out of bounds */
    let ii = (i & 0xff) as usize;
    let jj = (j & 0xff) as usize;
    let kk = (k & 0xff) as usize;

    /* Calculate the contribution from the four corners */
    let mut t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0;
    let [t20, t40]: [f64; 2];
    if t0 < 0.0 {
        n0 = 0.0;
        t0 = 0.0;
        t20 = 0.0;
        t40 = 0.0;
        gx0 = 0.0;
        gy0 = 0.0;
        gz0 = 0.0;
    } else {
        {
            let [lhs0, lhs1, lhs2] =
                gradient::grad3(PERM[ii + (PERM[jj + (PERM[kk]) as usize]) as usize] as usize);
            gx0 = lhs0;
            gy0 = lhs1;
            gz0 = lhs2;
        }
        t20 = t0 * t0;
        t40 = t20 * t20;
        n0 = t40 * (gx0 * x0 + gy0 * y0 + gz0 * z0);
    }

    let mut t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
    let [t21, t41]: [f64; 2];
    if t1 < 0.0 {
        n1 = 0.0;
        t1 = 0.0;
        t21 = 0.0;
        t41 = 0.0;
        gx1 = 0.0;
        gy1 = 0.0;
        gz1 = 0.0;
    } else {
        {
            let [lhs0, lhs1, lhs2] = gradient::grad3(
                PERM[ii + i1 + PERM[jj + j1 + PERM[kk + k1] as usize] as usize] as usize,
            );
            gx1 = lhs0;
            gy1 = lhs1;
            gz1 = lhs2;
        }
        t21 = t1 * t1;
        t41 = t21 * t21;
        n1 = t41 * (gx1 * x1 + gy1 * y1 + gz1 * z1);
    }

    let mut t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
    let [t22, t42]: [f64; 2];
    if t2 < 0.0 {
        n2 = 0.0;
        t2 = 0.0;
        t22 = 0.0;
        t42 = 0.0;
        gx2 = 0.0;
        gy2 = 0.0;
        gz2 = 0.0;
    } else {
        {
            let [lhs0, lhs1, lhs2] = gradient::grad3(
                PERM[ii + i2 + PERM[jj + j2 + PERM[kk + k2] as usize] as usize] as usize,
            );
            gx2 = lhs0;
            gy2 = lhs1;
            gz2 = lhs2;
        }

        t22 = t2 * t2;
        t42 = t22 * t22;
        n2 = t42 * (gx2 * x2 + gy2 * y2 + gz2 * z2);
    }

    let mut t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
    let [t23, t43]: [f64; 2];
    if t3 < 0.0 {
        n3 = 0.0;
        t3 = 0.0;
        t23 = 0.0;
        t43 = 0.0;
        gx3 = 0.0;
        gy3 = 0.0;
        gz3 = 0.0;
    } else {
        {
            let [lhs0, lhs1, lhs2] = gradient::grad3(
                PERM[ii + 1 + PERM[jj + 1 + PERM[kk + 1] as usize] as usize] as usize,
            );
            gx3 = lhs0;
            gy3 = lhs1;
            gz3 = lhs2;
        }
        t23 = t3 * t3;
        t43 = t23 * t23;
        n3 = t43 * (gx3 * x3 + gy3 * y3 + gz3 * z3);
    }

    /*  Add contributions from each corner to get the final noise value.
     * The result is scaled to return values in the range [-1,1] */
    let noise = 28.0 * (n0 + n1 + n2 + n3);

    if !with_derivatives {
        (noise, None)
    } else {
        /* Compute derivative, if requested by supplying non-null pointers
         * for the last three arguments */
        // if ((dnoise_dx != 0) && (dnoise_dy != 0) && (dnoise_dz != 0)) {
        /*  A straight, unoptimised calculation would be like:
         *    dnoise_dx = -8.0 * t20 * t0 * x0 * dot(gx0, gy0, gz0, x0, y0, z0) + t40 * gx0;
         *    dnoise_dy = -8.0 * t20 * t0 * y0 * dot(gx0, gy0, gz0, x0, y0, z0) + t40 * gy0;
         *    dnoise_dz = -8.0 * t20 * t0 * z0 * dot(gx0, gy0, gz0, x0, y0, z0) + t40 * gz0;
         *    dnoise_dx += -8.0 * t21 * t1 * x1 * dot(gx1, gy1, gz1, x1, y1, z1) + t41 * gx1;
         *    dnoise_dy += -8.0 * t21 * t1 * y1 * dot(gx1, gy1, gz1, x1, y1, z1) + t41 * gy1;
         *    dnoise_dz += -8.0 * t21 * t1 * z1 * dot(gx1, gy1, gz1, x1, y1, z1) + t41 * gz1;
         *    dnoise_dx += -8.0 * t22 * t2 * x2 * dot(gx2, gy2, gz2, x2, y2, z2) + t42 * gx2;
         *    dnoise_dy += -8.0 * t22 * t2 * y2 * dot(gx2, gy2, gz2, x2, y2, z2) + t42 * gy2;
         *    dnoise_dz += -8.0 * t22 * t2 * z2 * dot(gx2, gy2, gz2, x2, y2, z2) + t42 * gz2;
         *    dnoise_dx += -8.0 * t23 * t3 * x3 * dot(gx3, gy3, gz3, x3, y3, z3) + t43 * gx3;
         *    dnoise_dy += -8.0 * t23 * t3 * y3 * dot(gx3, gy3, gz3, x3, y3, z3) + t43 * gy3;
         *    dnoise_dz += -8.0 * t23 * t3 * z3 * dot(gx3, gy3, gz3, x3, y3, z3) + t43 * gz3;
         */
        let temp0 = t20 * t0 * (gx0 * x0 + gy0 * y0 + gz0 * z0);
        let mut dnoise_dx = temp0 * x0;
        let mut dnoise_dy = temp0 * y0;
        let mut dnoise_dz = temp0 * z0;
        let temp1 = t21 * t1 * (gx1 * x1 + gy1 * y1 + gz1 * z1);
        dnoise_dx += temp1 * x1;
        dnoise_dy += temp1 * y1;
        dnoise_dz += temp1 * z1;
        let temp2 = t22 * t2 * (gx2 * x2 + gy2 * y2 + gz2 * z2);
        dnoise_dx += temp2 * x2;
        dnoise_dy += temp2 * y2;
        dnoise_dz += temp2 * z2;
        let temp3 = t23 * t3 * (gx3 * x3 + gy3 * y3 + gz3 * z3);
        dnoise_dx += temp3 * x3;
        dnoise_dy += temp3 * y3;
        dnoise_dz += temp3 * z3;
        dnoise_dx *= -8.0;
        dnoise_dy *= -8.0;
        dnoise_dz *= -8.0;
        dnoise_dx += t40 * gx0 + t41 * gx1 + t42 * gx2 + t43 * gx3;
        dnoise_dy += t40 * gy0 + t41 * gy1 + t42 * gy2 + t43 * gy3;
        dnoise_dz += t40 * gz0 + t41 * gz1 + t42 * gz2 + t43 * gz3;
        dnoise_dx *= 28.0; /* Scale derivative to match the noise scaling */
        dnoise_dy *= 28.0;
        dnoise_dz *= 28.0;

        (noise, Some([dnoise_dx, dnoise_dy, dnoise_dz]))
    }
}

pub fn simplex_4d(
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    with_derivatives: bool,
) -> (f64, Option<[f64; 4]>) {
    let [n0, n1, n2, n3, n4]: [f64; 5]; // Noise contributions from the five corners
    let [gx0, gy0, gz0, gw0, gx1, gy1, gz1, gw1]: [f64; 8]; /* Gradients at simplex corners */
    let [gx2, gy2, gz2, gw2, gx3, gy3, gz3, gw3, gx4, gy4, gz4, gw4]: [f64; 12];
    let [t20, t21, t22, t23, t24]: [f64; 5];
    let [t40, t41, t42, t43, t44]: [f64; 5];

    let f4 = skew_factor(4);
    let g4 = unskew_factor(4);

    // Skew the (x,y,z,w) space to determine which cell of 24 simplices we're in
    let s = (x + y + z + w) * f4; // Factor for 4D skewing
    let xs = x + s;
    let ys = y + s;
    let zs = z + s;
    let ws = w + s;
    let i = xs.floor() as isize;
    let j = ys.floor() as isize;
    let k = zs.floor() as isize;
    let l = ws.floor() as isize;

    let t = (i + j + k + l) as f64 * g4; // Factor for 4D unskewing
    let X0 = i as f64 - t; // Unskew the cell origin back to (x,y,z,w) space
    let Y0 = j as f64 - t;
    let Z0 = k as f64 - t;
    let W0 = l as f64 - t;

    let x0 = x - X0; // The x,y,z,w distances from the cell origin
    let y0 = y - Y0;
    let z0 = z - Z0;
    let w0 = w - W0;

    // For the 4D case, the simplex is a 4D shape I won't even try to describe.
    // To find out which of the 24 possible simplices we're in, we need to
    // determine the magnitude ordering of x0, y0, z0 and w0.
    // The method below is a reasonable way of finding the ordering of x,y,z,w
    // and then find the correct traversal order for the simplex we're in.
    // First, six pair-wise comparisons are performed between each possible pair
    // of the four coordinates, and then the results are used to add up binary
    // bits for an integer index into a precomputed lookup table, simplex[].
    let c1 = if x0 > y0 { 32 } else { 0 };
    let c2 = if x0 > z0 { 16 } else { 0 };
    let c3 = if y0 > z0 { 8 } else { 0 };
    let c4 = if x0 > w0 { 4 } else { 0 };
    let c5 = if y0 > w0 { 2 } else { 0 };
    let c6 = if z0 > w0 { 1 } else { 0 };
    let c = c1 | c2 | c3 | c4 | c5 | c6; // '|' is mostly faster than '+'

    let [i1, j1, k1, l1]: [usize; 4]; // The integer offsets for the second simplex corner
    let [i2, j2, k2, l2]: [usize; 4]; // The integer offsets for the third simplex corner
    let [i3, j3, k3, l3]: [usize; 4]; // The integer offsets for the fourth simplex corner

    // simplex[c] is a 4-vector with the numbers 0, 1, 2 and 3 in some order.
    // Many values of c will never occur, since e.g. x>y>z>w makes x<z, y<w and x<w
    // impossible. Only the 24 indices which have non-zero entries make any sense.
    // We use a thresholding to set the coordinates in turn from the largest magnitude.
    // The number 3 in the "simplex" array is at the position of the largest coordinate.
    i1 = if SIMPLEX[c][0] >= 3 { 1 } else { 0 };
    j1 = if SIMPLEX[c][1] >= 3 { 1 } else { 0 };
    k1 = if SIMPLEX[c][2] >= 3 { 1 } else { 0 };
    l1 = if SIMPLEX[c][3] >= 3 { 1 } else { 0 };
    // The number 2 in the "simplex" array is at the second largest coordinate.
    i2 = if SIMPLEX[c][0] >= 2 { 1 } else { 0 };
    j2 = if SIMPLEX[c][1] >= 2 { 1 } else { 0 };
    k2 = if SIMPLEX[c][2] >= 2 { 1 } else { 0 };
    l2 = if SIMPLEX[c][3] >= 2 { 1 } else { 0 };
    // The number 1 in the "simplex" array is at the second smallest coordinate.
    i3 = if SIMPLEX[c][0] >= 1 { 1 } else { 0 };
    j3 = if SIMPLEX[c][1] >= 1 { 1 } else { 0 };
    k3 = if SIMPLEX[c][2] >= 1 { 1 } else { 0 };
    l3 = if SIMPLEX[c][3] >= 1 { 1 } else { 0 };
    // The fifth corner has all coordinate offsets = 1, so no need to look that up.

    let x1 = x0 - i1 as f64 + g4; // Offsets for second corner in (x,y,z,w) coords
    let y1 = y0 - j1 as f64 + g4;
    let z1 = z0 - k1 as f64 + g4;
    let w1 = w0 - l1 as f64 + g4;
    let x2 = x0 - i2 as f64 + 2.0 * g4; // Offsets for third corner in (x,y,z,w) coords
    let y2 = y0 - j2 as f64 + 2.0 * g4;
    let z2 = z0 - k2 as f64 + 2.0 * g4;
    let w2 = w0 - l2 as f64 + 2.0 * g4;
    let x3 = x0 - i3 as f64 + 3.0 * g4; // Offsets for fourth corner in (x,y,z,w) coords
    let y3 = y0 - j3 as f64 + 3.0 * g4;
    let z3 = z0 - k3 as f64 + 3.0 * g4;
    let w3 = w0 - l3 as f64 + 3.0 * g4;
    let x4 = x0 - 1.0 + 4.0 * g4; // Offsets for last corner in (x,y,z,w) coords
    let y4 = y0 - 1.0 + 4.0 * g4;
    let z4 = z0 - 1.0 + 4.0 * g4;
    let w4 = w0 - 1.0 + 4.0 * g4;

    // Wrap the integer indices at 256, to avoid indexing PERM[] out of bounds
    let ii = (i & 0xff) as usize;
    let jj = (j & 0xff) as usize;
    let kk = (k & 0xff) as usize;
    let ll = (l & 0xff) as usize;

    // Calculate the contribution from the five corners
    let mut t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0 - w0 * w0;
    if t0 < 0.0 {
        n0 = 0.0;
        t0 = 0.0;
        t20 = 0.0;
        t40 = 0.0;
        gx0 = 0.0;
        gy0 = 0.0;
        gz0 = 0.0;
        gw0 = 0.0;
    } else {
        t20 = t0 * t0;
        t40 = t20 * t20;
        {
            let [lhs0, lhs1, lhs2, lhs3] = gradient::grad4(
                PERM[ii + PERM[jj + PERM[kk + PERM[ll] as usize] as usize] as usize] as usize,
            );
            gx0 = lhs0;
            gy0 = lhs1;
            gz0 = lhs2;
            gw0 = lhs3;
        }
        n0 = t40 * (gx0 * x0 + gy0 * y0 + gz0 * z0 + gw0 * w0);
    }

    let mut t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1 - w1 * w1;
    if t1 < 0.0 {
        n1 = 0.0;
        t1 = 0.0;
        t21 = 0.0;
        t41 = 0.0;
        gx1 = 0.0;
        gy1 = 0.0;
        gz1 = 0.0;
        gw1 = 0.0;
    } else {
        t21 = t1 * t1;
        t41 = t21 * t21;
        {
            let [lhs0, lhs1, lhs2, lhs3] = gradient::grad4(
                PERM[ii
                    + i1
                    + PERM[jj + j1 + PERM[kk + k1 + PERM[ll + l1] as usize] as usize] as usize]
                    as usize,
            );
            gx1 = lhs0;
            gy1 = lhs1;
            gz1 = lhs2;
            gw1 = lhs3;
        }
        n1 = t41 * (gx1 * x1 + gy1 * y1 + gz1 * z1 + gw1 * w1);
    }

    let mut t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2 - w2 * w2;
    if t2 < 0.0 {
        n2 = 0.0;
        t2 = 0.0;
        t22 = 0.0;
        t42 = 0.0;
        gx2 = 0.0;
        gy2 = 0.0;
        gz2 = 0.0;
        gw2 = 0.0;
    } else {
        t22 = t2 * t2;
        t42 = t22 * t22;
        {
            let [lhs0, lhs1, lhs2, lhs3] = gradient::grad4(
                PERM[ii
                    + i2
                    + PERM[jj + j2 + PERM[kk + k2 + PERM[ll + l2] as usize] as usize] as usize]
                    as usize,
            );
            gx2 = lhs0;
            gy2 = lhs1;
            gz2 = lhs2;
            gw2 = lhs3;
        }
        n2 = t42 * (gx2 * x2 + gy2 * y2 + gz2 * z2 + gw2 * w2);
    }

    let mut t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3 - w3 * w3;
    if t3 < 0.0 {
        n3 = 0.0;
        t3 = 0.0;
        t23 = 0.0;
        t43 = 0.0;
        gx3 = 0.0;
        gy3 = 0.0;
        gz3 = 0.0;
        gw3 = 0.0;
    } else {
        t23 = t3 * t3;
        t43 = t23 * t23;
        {
            let [lhs0, lhs1, lhs2, lhs3] = gradient::grad4(
                PERM[ii
                    + i3
                    + PERM[jj + j3 + PERM[kk + k3 + PERM[ll + l3] as usize] as usize] as usize]
                    as usize,
            );
            gx3 = lhs0;
            gy3 = lhs1;
            gz3 = lhs2;
            gw3 = lhs3;
        }
        n3 = t43 * (gx3 * x3 + gy3 * y3 + gz3 * z3 + gw3 * w3);
    }

    let mut t4 = 0.6 - x4 * x4 - y4 * y4 - z4 * z4 - w4 * w4;
    if t4 < 0.0 {
        n4 = 0.0;
        t4 = 0.0;
        t24 = 0.0;
        t44 = 0.0;
        gx4 = 0.0;
        gy4 = 0.0;
        gz4 = 0.0;
        gw4 = 0.0;
    } else {
        t24 = t4 * t4;
        t44 = t24 * t24;
        {
            let [lhs0, lhs1, lhs2, lhs3] = gradient::grad4(
                PERM[ii + 1 + PERM[jj + 1 + PERM[kk + 1 + PERM[ll + 1] as usize] as usize] as usize]
                    as usize,
            );
            gx4 = lhs0;
            gy4 = lhs1;
            gz4 = lhs2;
            gw4 = lhs3;
        }
        n4 = t44 * (gx4 * x4 + gy4 * y4 + gz4 * z4 + gw4 * w4);
    }

    // Sum up and scale the result to cover the range [-1,1]
    let noise = 27.0 * (n0 + n1 + n2 + n3 + n4); // TODO: The scale factor is preliminary!

    if !with_derivatives {
        (noise, None)
    } else {
        /* Compute derivative, if requested by supplying non-null pointers
         * for the last four arguments */
        // if( ( dnoise_dx != 0 ) && ( dnoise_dy != 0 ) && ( dnoise_dz != 0 ) && ( dnoise_dw != 0 ) )
        // {
        /*  A straight, unoptimised calculation would be like:
         *     dnoise_dx = -8.0 * t20 * t0 * x0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gx0;
         *    dnoise_dy = -8.0 * t20 * t0 * y0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gy0;
         *    dnoise_dz = -8.0 * t20 * t0 * z0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gz0;
         *    dnoise_dw = -8.0 * t20 * t0 * w0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gw0;
         *    dnoise_dx += -8.0 * t21 * t1 * x1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gx1;
         *    dnoise_dy += -8.0 * t21 * t1 * y1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gy1;
         *    dnoise_dz += -8.0 * t21 * t1 * z1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gz1;
         *    dnoise_dw = -8.0 * t21 * t1 * w1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gw1;
         *    dnoise_dx += -8.0 * t22 * t2 * x2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gx2;
         *    dnoise_dy += -8.0 * t22 * t2 * y2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gy2;
         *    dnoise_dz += -8.0 * t22 * t2 * z2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gz2;
         *    dnoise_dw += -8.0 * t22 * t2 * w2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gw2;
         *    dnoise_dx += -8.0 * t23 * t3 * x3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gx3;
         *    dnoise_dy += -8.0 * t23 * t3 * y3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gy3;
         *    dnoise_dz += -8.0 * t23 * t3 * z3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gz3;
         *    dnoise_dw += -8.0 * t23 * t3 * w3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gw3;
         *    dnoise_dx += -8.0 * t24 * t4 * x4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gx4;
         *    dnoise_dy += -8.0 * t24 * t4 * y4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gy4;
         *    dnoise_dz += -8.0 * t24 * t4 * z4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gz4;
         *    dnoise_dw += -8.0 * t24 * t4 * w4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gw4;
         */
        let temp0 = t20 * t0 * (gx0 * x0 + gy0 * y0 + gz0 * z0 + gw0 * w0);
        let mut dnoise_dx = temp0 * x0;
        let mut dnoise_dy = temp0 * y0;
        let mut dnoise_dz = temp0 * z0;
        let mut dnoise_dw = temp0 * w0;
        let temp1 = t21 * t1 * (gx1 * x1 + gy1 * y1 + gz1 * z1 + gw1 * w1);
        dnoise_dx += temp1 * x1;
        dnoise_dy += temp1 * y1;
        dnoise_dz += temp1 * z1;
        dnoise_dw += temp1 * w1;
        let temp2 = t22 * t2 * (gx2 * x2 + gy2 * y2 + gz2 * z2 + gw2 * w2);
        dnoise_dx += temp2 * x2;
        dnoise_dy += temp2 * y2;
        dnoise_dz += temp2 * z2;
        dnoise_dw += temp2 * w2;
        let temp3 = t23 * t3 * (gx3 * x3 + gy3 * y3 + gz3 * z3 + gw3 * w3);
        dnoise_dx += temp3 * x3;
        dnoise_dy += temp3 * y3;
        dnoise_dz += temp3 * z3;
        dnoise_dw += temp3 * w3;
        let temp4 = t24 * t4 * (gx4 * x4 + gy4 * y4 + gz4 * z4 + gw4 * w4);
        dnoise_dx += temp4 * x4;
        dnoise_dy += temp4 * y4;
        dnoise_dz += temp4 * z4;
        dnoise_dw += temp4 * w4;
        dnoise_dx *= -8.0;
        dnoise_dy *= -8.0;
        dnoise_dz *= -8.0;
        dnoise_dw *= -8.0;
        dnoise_dx += t40 * gx0 + t41 * gx1 + t42 * gx2 + t43 * gx3 + t44 * gx4;
        dnoise_dy += t40 * gy0 + t41 * gy1 + t42 * gy2 + t43 * gy3 + t44 * gy4;
        dnoise_dz += t40 * gz0 + t41 * gz1 + t42 * gz2 + t43 * gz3 + t44 * gz4;
        dnoise_dw += t40 * gw0 + t41 * gw1 + t42 * gw2 + t43 * gw3 + t44 * gw4;

        dnoise_dx *= 28.0; /* Scale derivative to match the noise scaling */
        dnoise_dy *= 28.0;
        dnoise_dz *= 28.0;
        dnoise_dw *= 28.0;

        (noise, Some([dnoise_dx, dnoise_dy, dnoise_dz, dnoise_dw]))
    }
}

// A lookup table to traverse the simplex around a given point in 4D.
// Details can be found where this table is used, in the 4D noise method.
/* TODO: This should not be required, backport it from Bill's GLSL code! */
#[rustfmt::skip]
const SIMPLEX: [[u8; 4]; 64] = [
    [0, 1, 2, 3], [0, 1, 3, 2], [0, 0, 0, 0], [0, 2, 3, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [1, 2, 3, 0],
    [0, 2, 1, 3], [0, 0, 0, 0], [0, 3, 1, 2], [0, 3, 2, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [1, 3, 2, 0],
    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
    [1, 2, 0, 3], [0, 0, 0, 0], [1, 3, 0, 2], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [2, 3, 0, 1], [2, 3, 1, 0],
    [1, 0, 2, 3], [1, 0, 3, 2], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [2, 0, 3, 1], [0, 0, 0, 0], [2, 1, 3, 0],
    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
    [2, 0, 1, 3], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [3, 0, 1, 2], [3, 0, 2, 1], [0, 0, 0, 0], [3, 1, 2, 0],
    [2, 1, 0, 3], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [3, 1, 0, 2], [0, 0, 0, 0], [3, 2, 0, 1],[3, 2, 1, 0],
];

/// 2-dimensional Simplex noise
impl NoiseFn<f64, 2> for Simplex {
    fn get(&self, point: [f64; 2]) -> f64 {
        let (result, _) = simplex_2d(point[0], point[1], false);

        result
    }
    // fn get(&self, point: [f64; 2]) -> f64 {
    //     #[inline]
    //     fn surflet(gradient_index: usize, distance: [f64; 2]) -> f64 {
    //         let mut t = 0.5 - distance[0] * distance[0] - distance[1] * distance[1];
    //
    //         if t < 0.0 {
    //             0.0
    //         } else {
    //             t *= t;
    //             t * t * math::dot2(gradient::grad2(gradient_index), distance)
    //         }
    //     }
    //
    //     /// Skew the input point per the following formula:
    //     /// x' = x + (x + y) * F
    //     /// y' = y + (x + y) * F
    //     fn skew_point(point: [f64; 2], factor: f64) -> [f64; 2] {
    //         math::add2(point, [(point[0] + point[1]) * factor; 2])
    //     }
    //
    //     /// Unskew the input point per the following formula:
    //     /// x = x' - (x' + y') * G
    //     /// y = y' - (x' + y`) * G
    //     fn unskew_point(skewed_point: [f64; 2], factor: f64) -> [f64; 2] {
    //         math::sub2(
    //             skewed_point,
    //             [(skewed_point[0] + skewed_point[1]) * factor; 2],
    //         )
    //     }
    //
    //     let skew = skew_factor(2);
    //     let unskew = unskew_factor(2);
    //
    //     let skewed_input = skew_point(point, skew);
    //
    //     // Floor the skewed coordinate to determine which skewed unit cell the point is in.
    //     let floored = math::to_isize2(math::map2(skewed_input, f64::floor));
    //
    //     let cell = unskew_point(math::to_f64_2(floored), unskew);
    //
    //     // Calculate the vector from the cell's minimum corner to the point.
    //     let distance = math::sub2(point, cell);
    //
    //     // Sort the coordinates in decreasing order based on the largest component of the distance
    //     // vector.
    //     let offsets = if distance[0] > distance[1] {
    //         [1, 0]
    //     } else {
    //         [0, 1]
    //     };
    //
    //     let corner2 = math::add2(math::sub2(distance, math::to_f64_2(offsets)), [unskew; 2]);
    //     let corner3 = math::add2(math::sub2(distance, [1.0; 2]), [2.0 * unskew; 2]);
    //
    //     let gi0 = self.hasher.hash(&floored);
    //     let gi1 = self.hasher.hash(&math::add2(floored, offsets));
    //     let gi2 = self.hasher.hash(&math::add2(floored, [1; 2]));
    //
    //     let n0 = surflet(gi0, distance);
    //     let n1 = surflet(gi1, corner2);
    //     let n2 = surflet(gi2, corner3);
    //
    //     // TODO: Determine actual range for simplex noise and use correct scale value here
    //     70.0 * (n0 + n1 + n2)
    // }
}

/// 3-dimensional Simplex noise
impl NoiseFn<f64, 3> for Simplex {
    fn get(&self, point: [f64; 3]) -> f64 {
        let (result, _) = simplex_3d(point[0], point[1], point[2], false);

        result
    }
    // fn get(&self, point: [f64; 3]) -> f64 {
    //     #[inline]
    //     fn surflet(gradient_index: usize, distance: [f64; 3]) -> f64 {
    //         let mut t = 0.5
    //             - distance[0] * distance[0]
    //             - distance[1] * distance[1]
    //             - distance[2] * distance[2];
    //
    //         if t < 0.0 {
    //             0.0
    //         } else {
    //             t *= t;
    //             t * t * math::dot3(gradient::grad3(gradient_index), distance)
    //         }
    //     }
    //
    //     /// Skew the input point per the following formula:
    //     /// x' = x + (x + y + ...) * F
    //     /// y' = y + (x + y + ...) * F
    //     /// :
    //     fn skew_point(point: [f64; 3], factor: f64) -> [f64; 3] {
    //         math::add3(point, [(point[0] + point[1] + point[2]) * factor; 3])
    //     }
    //
    //     /// Unskew the input point per the following formula:
    //     /// x = x' - (x' + y' + ...) * G
    //     /// y = y' - (x' + y` + ...) * G
    //     /// :
    //     fn unskew_point(skewed_point: [f64; 3], factor: f64) -> [f64; 3] {
    //         math::sub3(
    //             skewed_point,
    //             [(skewed_point[0] + skewed_point[1] + skewed_point[2]) * factor; 3],
    //         )
    //     }
    //
    //     // Skew Value
    //     let skew = skew_factor(3);
    //     // Unskew value
    //     let unskew = unskew_factor(3);
    //
    //     let skewed_input = skew_point(point, skew);
    //
    //     let floored = math::to_isize3(math::map3(skewed_input, f64::floor));
    //
    //     let cell = unskew_point(math::to_f64_3(floored), unskew);
    //
    //     let distance = math::sub3(point, cell);
    //
    //     let offset1;
    //     let offset2;
    //
    //     if distance[0] >= distance[1] {
    //         if distance[1] >= distance[2] {
    //             offset1 = [1, 0, 0];
    //             offset2 = [1, 1, 0];
    //         } else if distance[0] >= distance[2] {
    //             offset1 = [1, 0, 0];
    //             offset2 = [1, 0, 1];
    //         } else {
    //             offset1 = [0, 0, 1];
    //             offset2 = [1, 0, 1];
    //         }
    //     } else if distance[2] >= distance[1] {
    //         offset1 = [0, 0, 1];
    //         offset2 = [0, 1, 1];
    //     } else if distance[2] >= distance[0] {
    //         offset1 = [0, 1, 0];
    //         offset2 = [0, 1, 1];
    //     } else {
    //         offset1 = [0, 1, 0];
    //         offset2 = [1, 1, 0];
    //     }
    //
    //     let offset3 = [1; 3];
    //
    //     let corner2 = math::add3(math::sub3(distance, math::to_f64_3(offset1)), [unskew; 3]);
    //
    //     let corner3 = math::add3(
    //         math::sub3(distance, math::to_f64_3(offset2)),
    //         [2.0 * unskew; 3],
    //     );
    //
    //     let corner4 = math::add3(math::sub3(distance, [1.0; 3]), [3.0 * unskew; 3]);
    //
    //     let gi0 = self.hasher.hash(&floored);
    //     let gi1 = self.hasher.hash(&math::add3(floored, offset1));
    //     let gi2 = self.hasher.hash(&math::add3(floored, offset2));
    //     let gi3 = self.hasher.hash(&math::add3(floored, offset3));
    //
    //     let n0 = surflet(gi0, distance);
    //     let n1 = surflet(gi1, corner2);
    //     let n2 = surflet(gi2, corner3);
    //     let n3 = surflet(gi3, corner4);
    //
    //     32.0 * (n0 + n1 + n2 + n3)
    // }
}

/// 4-dimensional Simplex noise
impl NoiseFn<f64, 4> for Simplex {
    fn get(&self, point: [f64; 4]) -> f64 {
        let (result, _) = simplex_4d(point[0], point[1], point[2], point[3], false);

        result
    }
    // fn get(&self, point: [f64; 4]) -> f64 {
    //     #[inline]
    //     fn surflet(gradient_index: usize, distance: [f64; 4]) -> f64 {
    //         let mut t = 0.5
    //             - distance[0] * distance[0]
    //             - distance[1] * distance[1]
    //             - distance[2] * distance[2]
    //             - distance[3] * distance[3];
    //
    //         if t < 0.0 {
    //             0.0
    //         } else {
    //             t *= t;
    //             t * t * math::dot4(gradient::grad4(gradient_index), distance)
    //         }
    //     }
    //
    //     /// Skew the input point per the following formula:
    //     /// x' = x + (x + y + ...) * F
    //     /// y' = y + (x + y + ...) * F
    //     /// :
    //     fn skew_point(point: [f64; 4], factor: f64) -> [f64; 4] {
    //         math::add4(
    //             point,
    //             [(point[0] + point[1] + point[2] + point[3]) * factor; 4],
    //         )
    //     }
    //
    //     /// Unskew the input point per the following formula:
    //     /// x = x' - (x' + y' + ...) * G
    //     /// y = y' - (x' + y` + ...) * G
    //     /// :
    //     fn unskew_point(skewed_point: [f64; 4], factor: f64) -> [f64; 4] {
    //         math::sub4(
    //             skewed_point,
    //             [(skewed_point[0] + skewed_point[1] + skewed_point[2] + skewed_point[3]) * factor;
    //                 4],
    //         )
    //     }
    //
    //     // Skew Value
    //     let skew: f64 = skew_factor(4);
    //     // Unskew Value
    //     let unskew: f64 = unskew_factor(4);
    //
    //     let skewed_input = skew_point(point, skew);
    //
    //     let floored = math::to_isize4(math::map4(skewed_input, f64::floor));
    //
    //     let cell = unskew_point(math::to_f64_4(floored), unskew);
    //
    //     let distance = math::sub4(point, cell);
    //
    //     let mut rank_x: u8 = 0;
    //     let mut rank_y: u8 = 0;
    //     let mut rank_z: u8 = 0;
    //     let mut rank_w: u8 = 0;
    //
    //     if distance[0] > distance[1] {
    //         rank_x += 1;
    //     } else {
    //         rank_y += 1;
    //     };
    //     if distance[0] > distance[2] {
    //         rank_x += 1;
    //     } else {
    //         rank_z += 1;
    //     };
    //     if distance[0] > distance[3] {
    //         rank_x += 1;
    //     } else {
    //         rank_w += 1;
    //     };
    //     if distance[1] > distance[2] {
    //         rank_y += 1;
    //     } else {
    //         rank_z += 1;
    //     };
    //     if distance[1] > distance[3] {
    //         rank_y += 1;
    //     } else {
    //         rank_w += 1;
    //     };
    //     if distance[2] > distance[3] {
    //         rank_z += 1;
    //     } else {
    //         rank_w += 1;
    //     };
    //
    //     let mut offset1 = [0; 4];
    //     let mut offset2 = [0; 4];
    //     let mut offset3 = [0; 4];
    //
    //     if rank_x >= 3 {
    //         offset1[0] = 1
    //     };
    //     if rank_y >= 3 {
    //         offset1[1] = 1
    //     };
    //     if rank_z >= 3 {
    //         offset1[2] = 1
    //     };
    //     if rank_w >= 3 {
    //         offset1[3] = 1
    //     };
    //
    //     if rank_x >= 2 {
    //         offset2[0] = 1
    //     };
    //     if rank_y >= 2 {
    //         offset2[1] = 1
    //     };
    //     if rank_z >= 2 {
    //         offset2[2] = 1
    //     };
    //     if rank_w >= 2 {
    //         offset2[3] = 1
    //     };
    //
    //     if rank_x >= 1 {
    //         offset3[0] = 1
    //     };
    //     if rank_y >= 1 {
    //         offset3[1] = 1
    //     };
    //     if rank_z >= 1 {
    //         offset3[2] = 1
    //     };
    //     if rank_w >= 1 {
    //         offset3[3] = 1
    //     };
    //
    //     let offset4 = [1; 4];
    //
    //     let corner2 = math::add4(math::sub4(distance, math::to_f64_4(offset1)), [unskew; 4]);
    //
    //     let corner3 = math::add4(
    //         math::sub4(distance, math::to_f64_4(offset2)),
    //         [2.0 * unskew; 4],
    //     );
    //
    //     let corner4 = math::add4(
    //         math::sub4(distance, math::to_f64_4(offset3)),
    //         [3.0 * unskew; 4],
    //     );
    //
    //     let corner5 = math::add4(math::sub4(distance, [1.0; 4]), [4.0 * unskew; 4]);
    //
    //     let gi0 = self.hasher.hash(&floored);
    //     let gi1 = self.hasher.hash(&math::add4(floored, offset1));
    //     let gi2 = self.hasher.hash(&math::add4(floored, offset2));
    //     let gi3 = self.hasher.hash(&math::add4(floored, offset3));
    //     let gi4 = self.hasher.hash(&math::add4(floored, offset4));
    //
    //     let n0 = surflet(gi0, distance);
    //     let n1 = surflet(gi1, corner2);
    //     let n2 = surflet(gi2, corner3);
    //     let n3 = surflet(gi3, corner4);
    //     let n4 = surflet(gi4, corner5);
    //
    //     27.0 * (n0 + n1 + n2 + n3 + n4)
    // }
}
