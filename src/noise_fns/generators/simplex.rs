use crate::{
    gradient,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::{NoiseHasher, PermutationTable},
};

/// Noise function that outputs N-dimensional Simplex noise.
///
/// The simplex noise code was adapted from code by Stefan Gustavson,
/// http://staffwww.itn.liu.se/~stegu/aqsis/aqsis-newnoise/sdnoise1234.c
///
/// This is Stefan Gustavson's original copyright notice:
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
///  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
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
    let h = hash & 15;
    let gx = (1 + (h & 7)) as f64; // Gradient value is one of 1.0, 2.0, ..., 8.0
    match h & 8 {
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

/// 1D Simplex Noise with Derivative
#[inline(always)]
pub fn simplex_1d(x: f64, hasher: &dyn NoiseHasher) -> (f64, f64) {
    let cell = x.floor() as isize;

    let near_distance = x - cell as f64;
    let far_distance = near_distance - 1.0;

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&[cell]);
    let gi1 = hasher.hash(&[cell + 1]);

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient: f64,
        x2: f64,
    }

    fn surflet(gradient_index: usize, x: f64) -> SurfletComponents {
        let x2 = x * x;
        let t = 1.0 - x2;

        // if t <= 0.0 { Never happens in 1D: x is always <= 1.
        // No influence
        // t0 = 0.0;
        // } else {
        let gradient = grad1((gradient_index % 0xff) as u8);
        let t2 = t * t;
        let t4 = t2 * t2;

        let value = t4 * gradient * x;

        SurfletComponents {
            value,
            t,
            t2,
            t4,
            gradient,
            x2,
        }
    }

    let corner0 = surflet(gi0, near_distance);

    let corner1 = surflet(gi1, far_distance);

    // The maximum value of this noise is 8*(3/4)^4 = 2.53125
    // A factor of 0.395 would scale to fit exactly within [-1,1], but
    // to better match classic Perlin noise, we scale it down some more.
    // ^-- Original note from Gustavson.

    // Since the objective of this library is to be as close to [-1, 1] as possible, we'll use the
    // 0.395 scale instead.
    let noise = 0.395 * (corner0.value + corner1.value);

    /* Compute derivative according to:
     *  dnoise_dx = -8.0 * t20 * t0 * x0 * (gx0 * x0) + t40 * gx0;
     *  dnoise_dx += -8.0 * t21 * t1 * x1 * (gx1 * x1) + t41 * gx1;
     */
    let mut dnoise_dx = corner0.t2 * corner0.t * corner0.gradient * corner0.x2;
    dnoise_dx += corner1.t2 * corner1.t * corner1.gradient * corner1.x2;
    dnoise_dx *= -8.0;
    dnoise_dx += corner0.t4 * corner0.gradient + corner1.t4 * corner1.gradient;
    dnoise_dx *= 0.395; /* Scale derivative to match the noise scaling */

    (noise, dnoise_dx)
}

#[inline(always)]
pub fn simplex_2d(x: f64, y: f64, hasher: &dyn NoiseHasher) -> (f64, [f64; 2]) {
    let f2: f64 = skew_factor(2);
    let g2: f64 = unskew_factor(2);

    /* Skew the input space to determine which simplex cell we're in */
    let skew = (x + y) * f2; /* Hairy factor for 2D */
    let skewed_x = x + skew;
    let skewed_y = y + skew;
    let cell_x = skewed_x.floor() as isize;
    let cell_y = skewed_y.floor() as isize;

    let unskew = (cell_x + cell_y) as f64 * g2;
    let unskewed_x = cell_x as f64 - unskew; /* Unskew the cell origin back to (x,y) space */
    let unskewed_y = cell_y as f64 - unskew;
    let distance_x = x - unskewed_x; /* The x,y distances from the cell origin */
    let distance_y = y - unskewed_y;

    /* For the 2D case, the simplex shape is an equilateral triangle.
     * Determine which simplex we are in. */
    let [i1, j1] = /* Offsets for second (middle) corner of simplex in (i,j) coords */
    if distance_x > distance_y {
        // lower triangle, XY order: (0,0)->(1,0)->(1,1)
        [1, 0]
    } else {
        // upper triangle, YX order: (0,0)->(0,1)->(1,1)
        [0, 1]
    };

    /* A step of (1,0) in (i,j) means a step of (1-c,-c) in (x,y), and
     * a step of (0,1) in (i,j) means a step of (-c,1-c) in (x,y), where
     * c = (3-sqrt(3))/6   */
    let x1 = distance_x - i1 as f64 + g2; /* Offsets for middle corner in (x,y) unskewed coords */
    let y1 = distance_y - j1 as f64 + g2;
    let x2 = distance_x - 1.0 + 2.0 * g2; /* Offsets for last corner in (x,y) unskewed coords */
    let y2 = distance_y - 1.0 + 2.0 * g2;

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&[cell_x, cell_y]);
    let gi1 = hasher.hash(&[cell_x + i1, cell_y + j1]);
    let gi2 = hasher.hash(&[cell_x + 1, cell_y + 1]);

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient_x: f64,
        gradient_y: f64,
    }

    impl SurfletComponents {
        fn zeros() -> Self {
            Self {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradient_x: 0.0,
                gradient_y: 0.0,
            }
        }
    }

    fn surflet(gradient_index: usize, x: f64, y: f64) -> SurfletComponents {
        let t = 0.5 - (x * x + y * y);

        if t <= 0.0 {
            // No influence
            SurfletComponents::zeros()
        } else {
            let [gradient_x, gradient_y] = gradient::grad2(gradient_index);
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: t4 * (gradient_x * x + gradient_y * y),
                t,
                t2,
                t4,
                gradient_x,
                gradient_y,
            }
        }
    }

    let corner0 = surflet(gi0, distance_x, distance_y);

    let corner1 = surflet(gi1, x1, y1);

    let corner2 = surflet(gi2, x2, y2);

    /* Add contributions from each corner to get the final noise value.
     * The result is scaled to return values in the interval [-1, 1]. */
    let noise = 40.0 * (corner0.value + corner1.value + corner2.value);

    /*  A straight, unoptimised calculation would be like:
     *    dnoise_dx = -8.0 * t20 * t0 * x0 * ( gx0 * x0 + gy0 * y0 ) + t40 * gx0;
     *    dnoise_dy = -8.0 * t20 * t0 * y0 * ( gx0 * x0 + gy0 * y0 ) + t40 * gy0;
     *    dnoise_dx += -8.0 * t21 * t1 * x1 * ( gx1 * x1 + gy1 * y1 ) + t41 * gx1;
     *    dnoise_dy += -8.0 * t21 * t1 * y1 * ( gx1 * x1 + gy1 * y1 ) + t41 * gy1;
     *    dnoise_dx += -8.0 * t22 * t2 * x2 * ( gx2 * x2 + gy2 * y2 ) + t42 * gx2;
     *    dnoise_dy += -8.0 * t22 * t2 * y2 * ( gx2 * x2 + gy2 * y2 ) + t42 * gy2;
     */
    let temp0 = corner0.t2
        * corner0.t
        * (corner0.gradient_x * distance_x + corner0.gradient_y * distance_y);
    let mut dnoise_dx = temp0 * distance_x;
    let mut dnoise_dy = temp0 * distance_y;
    let temp1 = corner1.t2 * corner1.t * (corner1.gradient_x * x1 + corner1.gradient_y * y1);
    dnoise_dx += temp1 * x1;
    dnoise_dy += temp1 * y1;
    let temp2 = corner2.t2 * corner2.t * (corner2.gradient_x * x2 + corner2.gradient_y * y2);
    dnoise_dx += temp2 * x2;
    dnoise_dy += temp2 * y2;
    dnoise_dx *= -8.0;
    dnoise_dy *= -8.0;
    dnoise_dx += corner0.t4 * corner0.gradient_x
        + corner1.t4 * corner1.gradient_x
        + corner2.t4 * corner2.gradient_x;
    dnoise_dy += corner0.t4 * corner0.gradient_y
        + corner1.t4 * corner1.gradient_y
        + corner2.t4 * corner2.gradient_y;
    dnoise_dx *= 40.0; /* Scale derivative to match the noise scaling */
    dnoise_dy *= 40.0;

    (noise, [dnoise_dx, dnoise_dy])
}

#[inline(always)]
pub fn simplex_3d(x: f64, y: f64, z: f64, hasher: &dyn NoiseHasher) -> (f64, [f64; 3]) {
    let f3 = skew_factor(3);
    let g3 = unskew_factor(3);

    /* Skew the input space to determine which simplex cell we're in */
    let skew = (x + y + z) * f3; /* Very nice and simple skew factor for 3D */
    let skewed_x = x + skew;
    let skewed_y = y + skew;
    let skewed_z = z + skew;
    let cell_x = skewed_x.floor() as isize;
    let cell_y = skewed_y.floor() as isize;
    let cell_z = skewed_z.floor() as isize;

    let unskew = (cell_x + cell_y + cell_z) as f64 * g3;
    let unskewed_x = cell_x as f64 - unskew; /* Unskew the cell origin back to (x,y,z) space */
    let unskewed_y = cell_y as f64 - unskew;
    let unskewed_z = cell_z as f64 - unskew;
    let distance_x = x - unskewed_x; /* The x,y,z distances from the cell origin */
    let distance_y = y - unskewed_y;
    let distance_z = z - unskewed_z;

    /* For the 3D case, the simplex shape is a slightly irregular tetrahedron.
     * Determine which simplex we are in. */
    /* TODO: This code would benefit from a backport from the GLSL version! */
    let [[i1, j1, k1], [i2, j2, k2]] = if distance_x >= distance_y {
        if distance_y >= distance_z {
            /* X Y Z order */
            [[1, 0, 0], [1, 1, 0]]
        } else if distance_x >= distance_z {
            /* X Z Y order */
            [[1, 0, 0], [1, 0, 1]]
        } else {
            /* Z X Y order */
            [[0, 0, 1], [1, 0, 1]]
        }
    } else {
        // x0<y0
        if distance_y < distance_z {
            /* Z Y X order */
            [[0, 0, 1], [0, 1, 1]]
        } else if distance_x < distance_z {
            /* Y Z X order */
            [[0, 1, 0], [0, 1, 1]]
        } else {
            /* Y X Z order */
            [[0, 1, 0], [1, 1, 0]]
        }
    };

    /* A step of (1,0,0) in (i,j,k) means a step of (1-c,-c,-c) in (x,y,z),
     * a step of (0,1,0) in (i,j,k) means a step of (-c,1-c,-c) in (x,y,z), and
     * a step of (0,0,1) in (i,j,k) means a step of (-c,-c,1-c) in (x,y,z), where
     * c = 1/6.   */

    let x1 = distance_x - i1 as f64 + g3; /* Offsets for second corner in (x,y,z) coords */
    let y1 = distance_y - j1 as f64 + g3;
    let z1 = distance_z - k1 as f64 + g3;
    let x2 = distance_x - i2 as f64 + 2.0 * g3; /* Offsets for third corner in (x,y,z) coords */
    let y2 = distance_y - j2 as f64 + 2.0 * g3;
    let z2 = distance_z - k2 as f64 + 2.0 * g3;
    let x3 = distance_x - 1.0 + 3.0 * g3; /* Offsets for last corner in (x,y,z) coords */
    let y3 = distance_y - 1.0 + 3.0 * g3;
    let z3 = distance_z - 1.0 + 3.0 * g3;

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&[cell_x, cell_y, cell_z]);
    let gi1 = hasher.hash(&[cell_x + i1, cell_y + j1, cell_z + k1]);
    let gi2 = hasher.hash(&[cell_x + i2, cell_y + j2, cell_z + k2]);
    let gi3 = hasher.hash(&[cell_x + 1, cell_y + 1, cell_z + 1]);

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient_x: f64,
        gradient_y: f64,
        gradient_z: f64,
    }

    impl SurfletComponents {
        fn zeros() -> Self {
            Self {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradient_x: 0.0,
                gradient_y: 0.0,
                gradient_z: 0.0,
            }
        }
    }

    fn surflet(gradient_index: usize, x: f64, y: f64, z: f64) -> SurfletComponents {
        let t = 0.5 - (x * x + y * y + z * z);

        if t <= 0.0 {
            // No influence
            SurfletComponents::zeros()
        } else {
            let [gradient_x, gradient_y, gradient_z] = gradient::grad3(gradient_index);
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: t4 * (gradient_x * x + gradient_y * y + gradient_z * z),
                t,
                t2,
                t4,
                gradient_x,
                gradient_y,
                gradient_z,
            }
        }
    }

    /* Calculate the contribution from the four corners */
    let corner0 = surflet(gi0, distance_x, distance_y, distance_z);

    let corner1 = surflet(gi1, x1, y1, z1);

    let corner2 = surflet(gi2, x2, y2, z2);

    let corner3 = surflet(gi3, x3, y3, z3);

    /*  Add contributions from each corner to get the final noise value.
     * The result is scaled to return values in the range [-1,1] */
    let noise = 28.0 * (corner0.value + corner1.value + corner2.value + corner3.value);

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
    let temp0 = corner0.t2
        * corner0.t
        * (corner0.gradient_x * distance_x
            + corner0.gradient_y * distance_y
            + corner0.gradient_z * distance_z);
    let mut dnoise_dx = temp0 * distance_x;
    let mut dnoise_dy = temp0 * distance_y;
    let mut dnoise_dz = temp0 * distance_z;
    let temp1 = corner1.t2
        * corner1.t
        * (corner1.gradient_x * x1 + corner1.gradient_y * y1 + corner1.gradient_z * z1);
    dnoise_dx += temp1 * x1;
    dnoise_dy += temp1 * y1;
    dnoise_dz += temp1 * z1;
    let temp2 = corner2.t2
        * corner2.t
        * (corner2.gradient_x * x2 + corner2.gradient_y * y2 + corner2.gradient_z * z2);
    dnoise_dx += temp2 * x2;
    dnoise_dy += temp2 * y2;
    dnoise_dz += temp2 * z2;
    let temp3 = corner3.t2
        * corner3.t
        * (corner3.gradient_x * x3 + corner3.gradient_y * y3 + corner3.gradient_z * z3);
    dnoise_dx += temp3 * x3;
    dnoise_dy += temp3 * y3;
    dnoise_dz += temp3 * z3;
    dnoise_dx *= -8.0;
    dnoise_dy *= -8.0;
    dnoise_dz *= -8.0;
    dnoise_dx += corner0.t4 * corner0.gradient_x
        + corner1.t4 * corner1.gradient_x
        + corner2.t4 * corner2.gradient_x
        + corner3.t4 * corner3.gradient_x;
    dnoise_dy += corner0.t4 * corner0.gradient_y
        + corner1.t4 * corner1.gradient_y
        + corner2.t4 * corner2.gradient_y
        + corner3.t4 * corner3.gradient_y;
    dnoise_dz += corner0.t4 * corner0.gradient_z
        + corner1.t4 * corner1.gradient_z
        + corner2.t4 * corner2.gradient_z
        + corner3.t4 * corner3.gradient_z;
    dnoise_dx *= 28.0; /* Scale derivative to match the noise scaling */
    dnoise_dy *= 28.0;
    dnoise_dz *= 28.0;

    (noise, [dnoise_dx, dnoise_dy, dnoise_dz])
}

#[allow(clippy::many_single_char_names)]
#[inline(always)]
pub fn simplex_4d(x: f64, y: f64, z: f64, w: f64, hasher: &dyn NoiseHasher) -> (f64, [f64; 4]) {
    let f4 = skew_factor(4);
    let g4 = unskew_factor(4);

    // Skew the (x,y,z,w) space to determine which cell of 24 simplices we're in
    let skewed = (x + y + z + w) * f4; // Factor for 4D skewing
    let skewed_x = x + skewed;
    let skewed_y = y + skewed;
    let skewed_z = z + skewed;
    let skewed_w = w + skewed;
    let cell_x = skewed_x.floor() as isize;
    let cell_y = skewed_y.floor() as isize;
    let cell_z = skewed_z.floor() as isize;
    let cell_w = skewed_w.floor() as isize;

    let unskew = (cell_x + cell_y + cell_z + cell_w) as f64 * g4; // Factor for 4D unskewing
    let unskewed_x = cell_x as f64 - unskew; // Unskew the cell origin back to (x,y,z,w) space
    let unskewed_y = cell_y as f64 - unskew;
    let unskewed_z = cell_z as f64 - unskew;
    let unskewed_w = cell_w as f64 - unskew;

    let distance_x = x - unskewed_x; // The x,y,z,w distances from the cell origin
    let distance_y = y - unskewed_y;
    let distance_z = z - unskewed_z;
    let distance_w = w - unskewed_w;

    // For the 4D case, the simplex is a 4D shape I won't even try to describe.
    // To find out which of the 24 possible simplices we're in, we need to
    // determine the magnitude ordering of x0, y0, z0 and w0.
    // The method below is a reasonable way of finding the ordering of x,y,z,w
    // and then find the correct traversal order for the simplex we're in.
    // First, six pair-wise comparisons are performed between each possible pair
    // of the four coordinates, and then the results are used to add up binary
    // bits for an integer index into a precomputed lookup table, simplex[].
    let c1 = if distance_x > distance_y { 32 } else { 0 };
    let c2 = if distance_x > distance_z { 16 } else { 0 };
    let c3 = if distance_y > distance_z { 8 } else { 0 };
    let c4 = if distance_x > distance_w { 4 } else { 0 };
    let c5 = if distance_y > distance_w { 2 } else { 0 };
    let c6 = if distance_z > distance_w { 1 } else { 0 };
    let c = c1 | c2 | c3 | c4 | c5 | c6; // '|' is mostly faster than '+'

    let [i1, j1, k1, l1]: [isize; 4]; // The integer offsets for the second simplex corner
    let [i2, j2, k2, l2]: [isize; 4]; // The integer offsets for the third simplex corner
    let [i3, j3, k3, l3]: [isize; 4]; // The integer offsets for the fourth simplex corner

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

    let x1 = distance_x - i1 as f64 + g4; // Offsets for second corner in (x,y,z,w) coords
    let y1 = distance_y - j1 as f64 + g4;
    let z1 = distance_z - k1 as f64 + g4;
    let w1 = distance_w - l1 as f64 + g4;
    let x2 = distance_x - i2 as f64 + 2.0 * g4; // Offsets for third corner in (x,y,z,w) coords
    let y2 = distance_y - j2 as f64 + 2.0 * g4;
    let z2 = distance_z - k2 as f64 + 2.0 * g4;
    let w2 = distance_w - l2 as f64 + 2.0 * g4;
    let x3 = distance_x - i3 as f64 + 3.0 * g4; // Offsets for fourth corner in (x,y,z,w) coords
    let y3 = distance_y - j3 as f64 + 3.0 * g4;
    let z3 = distance_z - k3 as f64 + 3.0 * g4;
    let w3 = distance_w - l3 as f64 + 3.0 * g4;
    let x4 = distance_x - 1.0 + 4.0 * g4; // Offsets for last corner in (x,y,z,w) coords
    let y4 = distance_y - 1.0 + 4.0 * g4;
    let z4 = distance_z - 1.0 + 4.0 * g4;
    let w4 = distance_w - 1.0 + 4.0 * g4;

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&[cell_x, cell_y, cell_z, cell_w]);
    let gi1 = hasher.hash(&[cell_x + i1, cell_y + j1, cell_z + k1, cell_w + l1]);
    let gi2 = hasher.hash(&[cell_x + i2, cell_y + j2, cell_z + k2, cell_w + l2]);
    let gi3 = hasher.hash(&[cell_x + i3, cell_y + j3, cell_z + k3, cell_w + l3]);
    let gi4 = hasher.hash(&[cell_x + 1, cell_y + 1, cell_z + 1, cell_w + 1]);

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient_x: f64,
        gradient_y: f64,
        gradient_z: f64,
        gradient_w: f64,
    }

    impl SurfletComponents {
        fn zeros() -> Self {
            Self {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradient_x: 0.0,
                gradient_y: 0.0,
                gradient_z: 0.0,
                gradient_w: 0.0,
            }
        }
    }

    fn surflet(gradient_index: usize, x: f64, y: f64, z: f64, w: f64) -> SurfletComponents {
        let t = 0.6 - (x * x + y * y + z * z + w * w);

        if t <= 0.0 {
            // No influence
            SurfletComponents::zeros()
        } else {
            let [gradient_x, gradient_y, gradient_z, gradient_w] = gradient::grad4(gradient_index);
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: t4 * (gradient_x * x + gradient_y * y + gradient_z * z + gradient_w * w),
                t,
                t2,
                t4,
                gradient_x,
                gradient_y,
                gradient_z,
                gradient_w,
            }
        }
    }

    /* Calculate the contribution from the five corners */
    let corner0 = surflet(gi0, distance_x, distance_y, distance_z, distance_w);

    let corner1 = surflet(gi1, x1, y1, z1, w1);

    let corner2 = surflet(gi2, x2, y2, z2, w2);

    let corner3 = surflet(gi3, x3, y3, z3, w3);

    let corner4 = surflet(gi4, x4, y4, z4, w4);

    // Sum up and scale the result to cover the range [-1,1]
    let noise =
        27.0 * (corner0.value + corner1.value + corner2.value + corner3.value + corner4.value); // TODO: The scale factor is preliminary!

    /*  A straight, unoptimised calculation would be like:
     *    dnoise_dx = -8.0 * t20 * t0 * x0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gx0;
     *    dnoise_dy = -8.0 * t20 * t0 * y0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gy0;
     *    dnoise_dz = -8.0 * t20 * t0 * z0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gz0;
     *    dnoise_dw = -8.0 * t20 * t0 * w0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gw0;
     *    dnoise_dx += -8.0 * t21 * t1 * x1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gx1;
     *    dnoise_dy += -8.0 * t21 * t1 * y1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gy1;
     *    dnoise_dz += -8.0 * t21 * t1 * z1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gz1;
     *    dnoise_dw += -8.0 * t21 * t1 * w1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gw1;
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
    let temp0 = corner0.t2
        * corner0.t
        * (corner0.gradient_x * distance_x
            + corner0.gradient_y * distance_y
            + corner0.gradient_z * distance_z
            + corner0.gradient_w * distance_w);
    let mut dnoise_dx = temp0 * distance_x;
    let mut dnoise_dy = temp0 * distance_y;
    let mut dnoise_dz = temp0 * distance_z;
    let mut dnoise_dw = temp0 * distance_w;
    let temp1 = corner1.t2
        * corner1.t
        * (corner1.gradient_x * x1
            + corner1.gradient_y * y1
            + corner1.gradient_z * z1
            + corner1.gradient_w * w1);
    dnoise_dx += temp1 * x1;
    dnoise_dy += temp1 * y1;
    dnoise_dz += temp1 * z1;
    dnoise_dw += temp1 * w1;
    let temp2 = corner2.t2
        * corner2.t
        * (corner2.gradient_x * x2
            + corner2.gradient_y * y2
            + corner2.gradient_z * z2
            + corner2.gradient_w * w2);
    dnoise_dx += temp2 * x2;
    dnoise_dy += temp2 * y2;
    dnoise_dz += temp2 * z2;
    dnoise_dw += temp2 * w2;
    let temp3 = corner3.t2
        * corner3.t
        * (corner3.gradient_x * x3
            + corner3.gradient_y * y3
            + corner3.gradient_z * z3
            + corner3.gradient_w * w3);
    dnoise_dx += temp3 * x3;
    dnoise_dy += temp3 * y3;
    dnoise_dz += temp3 * z3;
    dnoise_dw += temp3 * w3;
    let temp4 = corner4.t2
        * corner4.t
        * (corner4.gradient_x * x4
            + corner4.gradient_y * y4
            + corner4.gradient_z * z4
            + corner4.gradient_w * w4);
    dnoise_dx += temp4 * x4;
    dnoise_dy += temp4 * y4;
    dnoise_dz += temp4 * z4;
    dnoise_dw += temp4 * w4;
    dnoise_dx *= -8.0;
    dnoise_dy *= -8.0;
    dnoise_dz *= -8.0;
    dnoise_dw *= -8.0;
    dnoise_dx += corner0.t4 * corner0.gradient_x
        + corner1.t4 * corner1.gradient_x
        + corner2.t4 * corner2.gradient_x
        + corner3.t4 * corner3.gradient_x
        + corner4.t4 * corner4.gradient_x;
    dnoise_dy += corner0.t4 * corner0.gradient_y
        + corner1.t4 * corner1.gradient_y
        + corner2.t4 * corner2.gradient_y
        + corner3.t4 * corner3.gradient_y
        + corner4.t4 * corner4.gradient_y;
    dnoise_dz += corner0.t4 * corner0.gradient_z
        + corner1.t4 * corner1.gradient_z
        + corner2.t4 * corner2.gradient_z
        + corner3.t4 * corner3.gradient_z
        + corner4.t4 * corner4.gradient_z;
    dnoise_dw += corner0.t4 * corner0.gradient_w
        + corner1.t4 * corner1.gradient_w
        + corner2.t4 * corner2.gradient_w
        + corner3.t4 * corner3.gradient_w
        + corner4.t4 * corner4.gradient_w;

    dnoise_dx *= 28.0; /* Scale derivative to match the noise scaling */
    dnoise_dy *= 28.0;
    dnoise_dz *= 28.0;
    dnoise_dw *= 28.0;

    (noise, [dnoise_dx, dnoise_dy, dnoise_dz, dnoise_dw])
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
    [2, 1, 0, 3], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [3, 1, 0, 2], [0, 0, 0, 0], [3, 2, 0, 1], [3, 2, 1, 0],
];

/// 2-dimensional Simplex noise
impl NoiseFn<f64, 2> for Simplex {
    fn get(&self, point: [f64; 2]) -> f64 {
        let (result, _) = simplex_2d(point[0], point[1], &self.hasher);

        result
    }
}

/// 3-dimensional Simplex noise
impl NoiseFn<f64, 3> for Simplex {
    fn get(&self, point: [f64; 3]) -> f64 {
        let (result, _) = simplex_3d(point[0], point[1], point[2], &self.hasher);

        result
    }
}

/// 4-dimensional Simplex noise
impl NoiseFn<f64, 4> for Simplex {
    fn get(&self, point: [f64; 4]) -> f64 {
        let (result, _) = simplex_4d(point[0], point[1], point[2], point[3], &self.hasher);

        result
    }
}
