use crate::{
    gradient,
    math::vectors::{Vector2, Vector3, Vector4},
    permutationtable::NoiseHasher,
};
use num_traits::{Float, NumCast};

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

#[inline(always)]
pub fn simplex_2d<NH>(point: [f64; 2], hasher: &NH) -> (f64, [f64; 2])
where
    NH: NoiseHasher + ?Sized,
{
    let skew_factor: f64 = skew_factor(2);
    let unskew_factor: f64 = unskew_factor(2);

    let point = Vector2::from(point);

    // Skew the input space to determine which simplex cell we're in
    let skew = point.sum() * skew_factor;
    let skewed = point + skew;
    let cell = skewed.floor_to_isize();
    let floor = cell.numcast().unwrap();

    let unskew: f64 = floor.sum() * unskew_factor;
    // Unskew the cell origin back to (x,y) space
    let unskewed = floor - unskew;
    // The x,y distances from the cell origin
    let offset1 = point - unskewed;

    // For the 2D case, the simplex shape is an equilateral triangle.
    // Determine which simplex we are in.
    let order = if offset1.x > offset1.y {
        // Offsets for second (middle) corner of simplex in (i,j) coords
        // lower triangle, XY order: (0,0)->(1,0)->(1,1)
        Vector2::new(1.0, 0.0)
    } else {
        // upper triangle, YX order: (0,0)->(0,1)->(1,1)
        Vector2::new(0.0, 1.0)
    };

    // A step of (1,0) in (i,j) means a step of (1-c,-c) in (x,y), and
    // a step of (0,1) in (i,j) means a step of (-c,1-c) in (x,y), where
    // c = (3-sqrt(3))/6

    // Offsets for middle corner in (x,y) unskewed coords
    let offset2 = offset1 - order + unskew_factor;
    // Offsets for last corner in (x,y) unskewed coords
    let offset3 = offset1 - 1.0 + 2.0 * unskew_factor;

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&cell.into_array());
    let gi1 = hasher.hash(&(cell + order.numcast().unwrap()).into_array());
    let gi2 = hasher.hash(&(cell + 1).into_array());

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient: Vector2<f64>,
    }

    #[inline(always)]
    fn surflet(gradient_index: usize, point: Vector2<f64>) -> SurfletComponents {
        let t = 1.0 - point.magnitude_squared() * 2.0;

        if t > 0.0 {
            let gradient: Vector2<f64> = gradient::grad2(gradient_index).into();
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: (2.0 * t2 + t4) * point.dot(gradient),
                t,
                t2,
                t4,
                gradient,
            }
        } else {
            // No influence
            SurfletComponents {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradient: Vector2::zero(),
            }
        }
    }

    // Calculate the contribution from the three corners
    let corner0 = surflet(gi0, offset1);
    let corner1 = surflet(gi1, offset2);
    let corner2 = surflet(gi2, offset3);

    // Add contributions from each corner to get the final noise value.
    // The result is scaled to return values in the interval [-1, 1].
    let noise = corner0.value + corner1.value + corner2.value;

    // A straight, unoptimised calculation would be like:
    //   dnoise_dx = -8.0 * t20 * t0 * x0 * ( gx0 * x0 + gy0 * y0 ) + t40 * gx0;
    //   dnoise_dy = -8.0 * t20 * t0 * y0 * ( gx0 * x0 + gy0 * y0 ) + t40 * gy0;
    //   dnoise_dx += -8.0 * t21 * t1 * x1 * ( gx1 * x1 + gy1 * y1 ) + t41 * gx1;
    //   dnoise_dy += -8.0 * t21 * t1 * y1 * ( gx1 * x1 + gy1 * y1 ) + t41 * gy1;
    //   dnoise_dx += -8.0 * t22 * t2 * x2 * ( gx2 * x2 + gy2 * y2 ) + t42 * gx2;
    //   dnoise_dy += -8.0 * t22 * t2 * y2 * ( gx2 * x2 + gy2 * y2 ) + t42 * gy2;
    //
    let mut dnoise = offset1 + corner0.t2 * corner0.t * corner0.gradient.dot(offset1);
    dnoise += offset2 * corner1.t2 * corner1.t * corner1.gradient.dot(offset2);
    dnoise += offset3 * corner2.t2 * corner2.t * corner2.gradient.dot(offset3);

    dnoise *= -8.0;

    dnoise += corner0.gradient * corner0.t4
        + corner1.gradient * corner1.t4
        + corner2.gradient * corner2.t4;

    (noise, dnoise.into())
}

#[inline(always)]
pub fn simplex_3d<NH>(point: [f64; 3], hasher: &NH) -> (f64, [f64; 3])
where
    NH: NoiseHasher + ?Sized,
{
    let skew_factor: f64 = skew_factor(3);
    let unskew_factor: f64 = unskew_factor(3);

    let point = Vector3::from(point);

    /* Skew the input space to determine which simplex cell we're in */
    // let skew = (x + y + z) * f3; /* Very nice and simple skew factor for 3D */
    let skew = point.sum() * skew_factor;
    let skewed = point + skew;
    let cell = skewed.floor_to_isize();
    let floor = cell.numcast().unwrap();

    let unskew: f64 = floor.sum() * unskew_factor;
    /* Unskew the cell origin back to (x,y,z) space */
    let unskewed = floor - unskew;
    /* The x,y,z distances from the cell origin */
    let offset1 = point - unskewed;

    /* For the 3D case, the simplex shape is a slightly irregular tetrahedron.
     * Determine which simplex we are in. */
    /* TODO: This code would benefit from a backport from the GLSL version! */
    let (order1, order2): (Vector3<isize>, Vector3<isize>) = if offset1.x >= offset1.y {
        if offset1.y >= offset1.z {
            /* X Y Z order */
            (Vector3::new(1, 0, 0), Vector3::new(1, 1, 0))
        } else if offset1.x >= offset1.z {
            /* X Z Y order */
            (Vector3::new(1, 0, 0), Vector3::new(1, 0, 1))
        } else {
            /* Z X Y order */
            (Vector3::new(0, 0, 1), Vector3::new(1, 0, 1))
        }
    } else {
        // x0<y0
        if offset1.y < offset1.z {
            /* Z Y X order */
            (Vector3::new(0, 0, 1), Vector3::new(0, 1, 1))
        } else if offset1.x < offset1.z {
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

    let offset2 = offset1 - order1.numcast().unwrap() + unskew_factor;
    let offset3 = offset1 - order2.numcast().unwrap() + 2.0 * unskew_factor;
    let offset4 = offset1 - Vector3::one() + 3.0 * unskew_factor;

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&cell.into_array());
    let gi1 = hasher.hash(&(cell + order1).into_array());
    let gi2 = hasher.hash(&(cell + order2).into_array());
    let gi3 = hasher.hash(&(cell + 1).into_array());

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient: Vector3<f64>,
    }

    fn surflet(gradient_index: usize, point: Vector3<f64>) -> SurfletComponents {
        let t = 1.0 - point.magnitude_squared() * 2.0;

        if t > 0.0 {
            let gradient = gradient::grad3(gradient_index).into();
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: (2.0 * t2 + t4) * point.dot(gradient),
                t,
                t2,
                t4,
                gradient,
            }
        } else {
            // No influence
            SurfletComponents {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradient: Vector3::zero(),
            }
        }
    }

    /* Calculate the contribution from the four corners */
    let corner0 = surflet(gi0, offset1);
    let corner1 = surflet(gi1, offset2);
    let corner2 = surflet(gi2, offset3);
    let corner3 = surflet(gi3, offset4);

    /*  Add contributions from each corner to get the final noise value.
     * The result is scaled to return values in the range [-1,1] */
    let noise = corner0.value + corner1.value + corner2.value + corner3.value;

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
    let mut dnoise = offset1 * corner0.t2 * corner0.t * corner0.gradient.dot(offset1);
    dnoise += offset2 * corner1.t2 * corner1.t * corner1.gradient.dot(offset2);
    dnoise += offset3 * corner2.t2 * corner2.t * corner2.gradient.dot(offset3);
    dnoise += offset4 * corner3.t2 * corner3.t * corner3.gradient.dot(offset4);

    dnoise *= -8.0;

    dnoise += corner0.gradient * corner0.t4
        + corner1.gradient * corner1.t4
        + corner2.gradient * corner2.t4
        + corner3.gradient * corner3.t4;

    (noise, dnoise.into())
}

#[inline(always)]
pub fn simplex_4d<NH>(point: [f64; 4], hasher: &NH) -> (f64, [f64; 4])
where
    NH: NoiseHasher + ?Sized,
{
    let skew_factor: f64 = skew_factor(4);
    let unskew_factor: f64 = unskew_factor(4);

    let point = Vector4::from(point);

    // Skew the (x,y,z,w) space to determine which cell of 24 simplices we're in
    // Factor for 4D skewing
    let skew = point.sum() * skew_factor;
    let skewed = point + skew;
    let cell: Vector4<isize> = skewed.floor_to_isize();
    let floor = cell.numcast().unwrap();

    // Factor for 4D unskewing
    let unskew: f64 = floor.sum() * unskew_factor;
    // Unskew the cell origin back to (x,y,z,w) space
    let unskewed = floor - unskew;
    // The x,y,z,w distances from the cell origin
    let offset1 = point - unskewed;

    // For the 4D case, the simplex is a 4D shape I won't even try to describe.
    // To find out which of the 24 possible simplices we're in, we need to
    // determine the magnitude ordering of x0, y0, z0 and w0.
    // The method below is a reasonable way of finding the ordering of x,y,z,w
    // and then find the correct traversal order for the simplex we're in.
    // First, six pair-wise comparisons are performed between each possible pair
    // of the four coordinates, and then the results are used to add up binary
    // bits for an integer index into a precomputed lookup table, simplex[].
    let c1 = (offset1.x > offset1.y) as usize * 32;
    let c2 = (offset1.x > offset1.z) as usize * 16;
    let c3 = (offset1.y > offset1.z) as usize * 8;
    let c4 = (offset1.x > offset1.w) as usize * 4;
    let c5 = (offset1.y > offset1.w) as usize * 2;
    let c6 = (offset1.z > offset1.w) as usize;
    let c = c1 | c2 | c3 | c4 | c5 | c6; // '|' is mostly faster than '+'

    // simplex[c] is a 4-vector with the numbers 0, 1, 2 and 3 in some order.
    // Many values of c will never occur, since e.g. x>y>z>w makes x<z, y<w and x<w
    // impossible. Only the 24 indices which have non-zero entries make any sense.
    // We use a thresholding to set the coordinates in turn from the largest magnitude.
    // The number 3 in the "simplex" array is at the position of the largest coordinate.
    let order1 = Vector4::from(SIMPLEX[c]).map(|n| if n >= 3 { 1 } else { 0 });
    // The number 2 in the "simplex" array is at the second largest coordinate.
    let order2 = Vector4::from(SIMPLEX[c]).map(|n| if n >= 2 { 1 } else { 0 });
    // The number 1 in the "simplex" array is at the second smallest coordinate.
    let order3 = Vector4::from(SIMPLEX[c]).map(|n| if n >= 1 { 1 } else { 0 });
    // The fifth corner has all coordinate offsets = 1, so no need to look that up.

    // Offsets for second corner in (x,y,z,w) coords
    let offset2 = offset1 - order1.numcast().unwrap() + unskew_factor;
    // Offsets for third corner in (x,y,z,w) coords
    let offset3 = offset1 - order2.numcast().unwrap() + 2.0 * unskew_factor;
    // Offsets for fourth corner in (x,y,z,w) coords
    let offset4 = offset1 - order3.numcast().unwrap() + 3.0 * unskew_factor;
    // Offsets for last corner in (x,y,z,w) coords
    let offset5 = offset1 - 1.0 + 4.0 * unskew_factor;

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&cell.into_array());
    let gi1 = hasher.hash(&(cell + order1).into_array());
    let gi2 = hasher.hash(&(cell + order2).into_array());
    let gi3 = hasher.hash(&(cell + order3).into_array());
    let gi4 = hasher.hash(&(cell + 1).into_array());

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient: Vector4<f64>,
    }

    fn surflet(gradient_index: usize, point: Vector4<f64>) -> SurfletComponents {
        let t = 1.0 - point.magnitude_squared() * 2.0;

        if t > 0.0 {
            let gradient = gradient::grad4(gradient_index).into();
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: (2.0 * t2 + t4) * point.dot(gradient),
                t,
                t2,
                t4,
                gradient,
            }
        } else {
            // No influence
            SurfletComponents {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradient: Vector4::zero(),
            }
        }
    }

    /* Calculate the contribution from the five corners */
    let corner1 = surflet(gi0, offset1);
    let corner2 = surflet(gi1, offset2);
    let corner3 = surflet(gi2, offset3);
    let corner4 = surflet(gi3, offset4);
    let corner5 = surflet(gi4, offset5);

    // Sum up and scale the result to cover the range [-1,1]
    let noise = corner1.value + corner2.value + corner3.value + corner4.value + corner5.value;

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
    let mut dnoise = offset1 * corner1.t2 * corner1.t * corner1.gradient.dot(offset1);
    dnoise += offset2 * corner2.t2 * corner2.t * corner2.gradient.dot(offset2);
    dnoise += offset3 * corner3.t2 * corner3.t * corner3.gradient.dot(offset3);
    dnoise += offset4 * corner4.t2 * corner4.t * corner4.gradient.dot(offset4);
    dnoise += offset5 * corner5.t2 * corner5.t * corner5.gradient.dot(offset5);

    dnoise *= -8.0;

    dnoise += corner1.gradient * corner1.t4
        + corner2.gradient * corner2.t4
        + corner3.gradient * corner3.t4
        + corner4.gradient * corner4.t4
        + corner5.gradient * corner5.t4;

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
