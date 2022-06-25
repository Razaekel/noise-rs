use crate::{
    gradient,
    math::vectors::{Vector, Vector2, Vector3, Vector4},
    permutationtable::NoiseHasher,
};
use num_traits::{Float, NumCast};

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
fn skew_factor<F>(n: usize) -> F
where
    F: Float,
{
    let n: F = NumCast::from(n).unwrap();

    ((n + F::one()).sqrt() - F::one()) / n
}

//  Unskew Value
//
//     1 - 1 / sqrt(n + 1)
// G = -------------------
//             n
fn unskew_factor<F>(n: usize) -> F
where
    F: Float,
{
    let n: F = NumCast::from(n).unwrap();

    (F::one() - (F::one() / (n + F::one()).sqrt())) / n
}

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
///
/// 1D Simplex Noise with Derivative
#[inline(always)]
pub fn simplex_1d<NH>(x: f64, hasher: &NH) -> (f64, f64)
where
    NH: NoiseHasher + ?Sized,
{
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
pub fn simplex_2d<NH>(point: [f64; 2], hasher: &NH) -> (f64, [f64; 2])
where
    NH: NoiseHasher + ?Sized,
{
    let f2: f64 = skew_factor(2);
    let g2: f64 = unskew_factor(2);

    let point = Vector2::from(point);

    /* Skew the input space to determine which simplex cell we're in */
    let skew = point.sum() * f2; /* Hairy factor for 2D */
    let skewed = point + Vector2::broadcast(skew);
    let cell: Vector2<isize> = skewed.floor().numcast().unwrap();

    let unskew: f64 = cell.sum() as f64 * g2;
    // Unskew the cell origin back to (x,y) space
    let unskewed = cell.numcast().unwrap() - Vector2::broadcast(unskew);
    // The x,y distances from the cell origin
    let distance = point - unskewed;

    // For the 2D case, the simplex shape is an equilateral triangle.
    // Determine which simplex we are in.
    let offset = if distance.x > distance.y {
        /* Offsets for second (middle) corner of simplex in (i,j) coords */
        // lower triangle, XY order: (0,0)->(1,0)->(1,1)
        Vector2::from([1, 0])
    } else {
        // upper triangle, YX order: (0,0)->(0,1)->(1,1)
        Vector2::from([0, 1])
    };

    /* A step of (1,0) in (i,j) means a step of (1-c,-c) in (x,y), and
     * a step of (0,1) in (i,j) means a step of (-c,1-c) in (x,y), where
     * c = (3-sqrt(3))/6   */
    // Offsets for middle corner in (x,y) unskewed coords */
    let distance1 = distance - offset.numcast().unwrap() + Vector2::broadcast(g2);
    /* Offsets for last corner in (x,y) unskewed coords */
    let distance2 = distance - Vector2::broadcast(1.0 + 2.0 * g2);

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&cell.into_array());
    let gi1 = hasher.hash(&(cell + distance1.numcast().unwrap()).into_array());
    let gi2 = hasher.hash(&(cell + Vector2::one()).into_array());

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient: Vector2<f64>,
    }

    impl SurfletComponents {
        fn zeros() -> Self {
            Self {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradient: Vector2::zero(),
            }
        }
    }

    fn surflet(gradient_index: usize, point: Vector2<f64>) -> SurfletComponents {
        // let t = 0.5 - (x * x + y * y);
        let t = 0.5 - point.magnitude_squared();

        if t > 0.0 {
            let gradient = Vector2::from(gradient::grad2(gradient_index));
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: t4 * gradient.dot(point),
                t,
                t2,
                t4,
                gradient,
            }
        } else {
            // No influence
            SurfletComponents::zeros()
        }
    }

    let corner0 = surflet(gi0, distance);
    let corner1 = surflet(gi1, distance1);
    let corner2 = surflet(gi2, distance2);

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
    let temp0 = corner0.t2 * corner0.t * corner0.gradient.dot(distance);
    let mut dnoise = distance + Vector2::broadcast(temp0);

    let temp1 = corner1.t2 * corner1.t * corner1.gradient.dot(distance1);
    dnoise += distance1 * temp1;

    let temp2 = corner2.t2 * corner2.t * corner2.gradient.dot(distance2);
    dnoise += distance2 * temp2;

    dnoise *= -8.0;

    dnoise += corner0.gradient * corner0.t4
        + corner1.gradient * corner1.t4
        + corner2.gradient * corner2.t4;

    dnoise *= 40.0; /* Scale derivative to match the noise scaling */

    (noise, dnoise.into())
}

#[inline(always)]
pub fn simplex_3d<NH>(point: [f64; 3], hasher: &NH) -> (f64, [f64; 3])
where
    NH: NoiseHasher + ?Sized,
{
    let f3: f64 = skew_factor(3);
    let g3: f64 = unskew_factor(3);

    let point = Vector3::from(point);

    /* Skew the input space to determine which simplex cell we're in */
    // let skew = (x + y + z) * f3; /* Very nice and simple skew factor for 3D */
    let skew = point.sum() * f3;
    let skewed = point + Vector3::broadcast(skew);
    let cell: Vector3<isize> = skewed.floor().numcast().unwrap();

    // let unskew = (cell_x + cell_y + cell_z) as f64 * g3;
    let unskew = cell.sum() as f64 * g3;
    /* Unskew the cell origin back to (x,y,z) space */
    let unskewed = cell.numcast().unwrap() - Vector3::broadcast(unskew);
    /* The x,y,z distances from the cell origin */
    let distance = point - unskewed;

    /* For the 3D case, the simplex shape is a slightly irregular tetrahedron.
     * Determine which simplex we are in. */
    /* TODO: This code would benefit from a backport from the GLSL version! */
    let (order1, order2): (Vector3<isize>, Vector3<isize>) = if distance.x >= distance.y {
        if distance.y >= distance.z {
            /* X Y Z order */
            (Vector3::new(1, 0, 0), Vector3::new(1, 1, 0))
        } else if distance.x >= distance.z {
            /* X Z Y order */
            (Vector3::new(1, 0, 0), Vector3::new(1, 0, 1))
        } else {
            /* Z X Y order */
            (Vector3::new(0, 0, 1), Vector3::new(1, 0, 1))
        }
    } else {
        // x0<y0
        if distance.y < distance.z {
            /* Z Y X order */
            (Vector3::new(0, 0, 1), Vector3::new(0, 1, 1))
        } else if distance.x < distance.z {
            /* Y Z X order */
            (Vector3::new(0, 1, 0), Vector3::new(0, 1, 1))
        } else {
            /* Y X Z order */
            (Vector3::new(0, 1, 0), Vector3::new(1, 1, 0))
        }
    };

    /* A step of (1,0,0) in (i,j,k) means a step of (1-c,-c,-c) in (x,y,z),
     * a step of (0,1,0) in (i,j,k) means a step of (-c,1-c,-c) in (x,y,z), and
     * a step of (0,0,1) in (i,j,k) means a step of (-c,-c,1-c) in (x,y,z), where
     * c = 1/6.   */

    let offset1 = distance - order1.numcast().unwrap() + Vector3::broadcast(g3);
    let offset2 = distance - order2.numcast().unwrap() + Vector3::broadcast(2.0 * g3);
    let offset3 = distance - Vector3::one() + Vector3::broadcast(3.0 * g3);

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&cell.into_array());
    let gi1 = hasher.hash(&(cell + order1).into_array());
    let gi2 = hasher.hash(&(cell + order2).into_array());
    let gi3 = hasher.hash(&(cell + Vector3::one()).into_array());

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient: Vector3<f64>,
    }

    impl SurfletComponents {
        fn zeros() -> Self {
            Self {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradient: Vector3::zero(),
            }
        }
    }

    fn surflet(gradient_index: usize, point: Vector3<f64>) -> SurfletComponents {
        let t = 0.5 - point.magnitude_squared();

        if t > 0.0 {
            let gradient = Vector3::from(gradient::grad3(gradient_index));
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: t4 * gradient.dot(point),
                t,
                t2,
                t4,
                gradient,
            }
        } else {
            // No influence
            SurfletComponents::zeros()
        }
    }

    /* Calculate the contribution from the four corners */
    let corner0 = surflet(gi0, distance);
    let corner1 = surflet(gi1, offset1);
    let corner2 = surflet(gi2, offset2);
    let corner3 = surflet(gi3, offset3);

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
    let temp0 = corner0.t2 * corner0.t * corner0.gradient.dot(distance);
    let mut dnoise = distance * temp0;

    let temp1 = corner1.t2 * corner1.t * corner1.gradient.dot(offset1);
    dnoise += offset1 * temp1;

    let temp2 = corner2.t2 * corner2.t * corner2.gradient.dot(offset2);
    dnoise += offset2 * temp2;

    let temp3 = corner3.t2 * corner3.t * corner3.gradient.dot(offset3);
    dnoise += offset3 * temp3;

    dnoise *= -8.0;

    dnoise += corner0.gradient * corner0.t4
        + corner1.gradient * corner1.t4
        + corner2.gradient * corner2.t4
        + corner3.gradient * corner3.t4;

    /* Scale derivative to match the noise scaling */
    dnoise *= 28.0;

    (noise, dnoise.into())
}

#[inline(always)]
pub fn simplex_4d<NH>(point: [f64; 4], hasher: &NH) -> (f64, [f64; 4])
where
    NH: NoiseHasher + ?Sized,
{
    let f4: f64 = skew_factor(4);
    let g4: f64 = unskew_factor(4);

    let point = Vector4::from(point);

    // Skew the (x,y,z,w) space to determine which cell of 24 simplices we're in
    // Factor for 4D skewing
    let skew = point.sum() * f4;
    let skewed = point + Vector4::broadcast(skew);
    let cell: Vector4<isize> = skewed.numcast().unwrap();

    // Factor for 4D unskewing
    let unskew = cell.sum() as f64 * g4;
    // Unskew the cell origin back to (x,y,z,w) space
    let unskewed = cell.numcast().unwrap() - Vector4::broadcast(unskew);

    // let distance_x = x - unskewed_x; // The x,y,z,w distances from the cell origin
    // let distance_y = y - unskewed_y;
    // let distance_z = z - unskewed_z;
    // let distance_w = w - unskewed_w;
    let distance = point - unskewed;

    // For the 4D case, the simplex is a 4D shape I won't even try to describe.
    // To find out which of the 24 possible simplices we're in, we need to
    // determine the magnitude ordering of x0, y0, z0 and w0.
    // The method below is a reasonable way of finding the ordering of x,y,z,w
    // and then find the correct traversal order for the simplex we're in.
    // First, six pair-wise comparisons are performed between each possible pair
    // of the four coordinates, and then the results are used to add up binary
    // bits for an integer index into a precomputed lookup table, simplex[].
    let c1 = if distance.x > distance.y { 32 } else { 0 };
    let c2 = if distance.x > distance.z { 16 } else { 0 };
    let c3 = if distance.y > distance.z { 8 } else { 0 };
    let c4 = if distance.x > distance.w { 4 } else { 0 };
    let c5 = if distance.y > distance.w { 2 } else { 0 };
    let c6 = if distance.z > distance.w { 1 } else { 0 };
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

    let order1 = Vector4::new(i1, j1, k1, l1);
    let order2 = Vector4::new(i2, j2, k2, l2);
    let order3 = Vector4::new(i3, j3, k3, l3);

    // Offsets for second corner in (x,y,z,w) coords
    let offset1 = distance - order1.numcast().unwrap() + Vector4::broadcast(g4);
    // Offsets for third corner in (x,y,z,w) coords
    let offset2 = distance - order2.numcast().unwrap() + Vector4::broadcast(2.0 * g4);
    // Offsets for fourth corner in (x,y,z,w) coords
    let offset3 = distance - order3.numcast().unwrap() + Vector4::broadcast(3.0 * g4);
    // Offsets for last corner in (x,y,z,w) coords
    let offset4 = distance - Vector4::one() + Vector4::broadcast(4.0 * g4);

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&cell.into_array());
    let gi1 = hasher.hash(&(cell + order1).into_array());
    let gi2 = hasher.hash(&(cell + order2).into_array());
    let gi3 = hasher.hash(&(cell + order2).into_array());
    let gi4 = hasher.hash(&(cell + Vector4::one()).into_array());

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient: Vector4<f64>,
    }

    impl SurfletComponents {
        fn zeros() -> Self {
            Self {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradient: Vector4::zero(),
            }
        }
    }

    fn surflet(gradient_index: usize, point: Vector4<f64>) -> SurfletComponents {
        let t = 0.6 - point.magnitude_squared();

        if t > 0.0 {
            let gradient = Vector4::from(gradient::grad4(gradient_index));
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: t4 * gradient.dot(point),
                t,
                t2,
                t4,
                gradient,
            }
        } else {
            // No influence
            SurfletComponents::zeros()
        }
    }

    /* Calculate the contribution from the five corners */
    let corner0 = surflet(gi0, distance);
    let corner1 = surflet(gi1, offset1);
    let corner2 = surflet(gi2, offset2);
    let corner3 = surflet(gi3, offset3);
    let corner4 = surflet(gi4, offset4);

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
    let temp0 = corner0.t2 * corner0.t * (corner0.gradient.dot(distance));
    let mut dnoise = distance * temp0;

    let temp1 = corner1.t2 * corner1.t * (corner1.gradient.dot(offset1));
    dnoise += offset1 * temp1;

    let temp2 = corner2.t2 * corner2.t * (corner2.gradient.dot(offset2));
    dnoise += offset2 * temp2;

    let temp3 = corner3.t2 * corner3.t * (corner3.gradient.dot(offset3));
    dnoise += offset3 * temp3;

    let temp4 = corner4.t2 * corner4.t * (corner4.gradient.dot(offset4));
    dnoise += offset4 * temp4;

    dnoise *= -8.0;

    dnoise += corner0.gradient * corner0.t4
        + corner1.gradient * corner1.t4
        + corner2.gradient * corner2.t4
        + corner3.gradient * corner3.t4
        + corner4.gradient * corner4.t4;

    // Scale derivative to match the noise scaling
    dnoise *= 28.0;

    (noise, dnoise.into())
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
