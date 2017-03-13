// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

//! Note that this is NOT Ken Perlin's simplex noise, as that is patent encumbered.
//! Instead, these functions use the OpenSimplex algorithm, as detailed here:
//! http://uniblock.tumblr.com/post/97868843242/noise

use {PermutationTable, gradient, math};
use num_traits::Float;
use std::ops::Add;

const STRETCH_CONSTANT_2D: f64 = -0.211324865405187; //(1/sqrt(2+1)-1)/2;
const SQUISH_CONSTANT_2D: f64 = 0.366025403784439; //(sqrt(2+1)-1)/2;
const STRETCH_CONSTANT_3D: f64 = -1.0 / 6.0; //(1/Math.sqrt(3+1)-1)/3;
const SQUISH_CONSTANT_3D: f64 = 1.0 / 3.0; //(Math.sqrt(3+1)-1)/3;
const STRETCH_CONSTANT_4D: f64 = -0.138196601125011; //(Math.sqrt(4+1)-1)/4;
const SQUISH_CONSTANT_4D: f64 = 0.309016994374947; //(Math.sqrt(4+1)-1)/4;

const NORM_CONSTANT_2D: f32 = 1.0 / 14.0;
const NORM_CONSTANT_3D: f32 = 1.0 / 14.0;
const NORM_CONSTANT_4D: f32 = 1.0 / 6.8699090070956625;

/// 2-dimensional [OpenSimplex Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than `noise::perlin2`.
pub fn open_simplex2<T: Float>(perm_table: &PermutationTable, point: &::Point2<T>) -> T {
    fn gradient<T: Float>(perm_table: &PermutationTable,
                          xs_floor: T,
                          ys_floor: T,
                          dx: T,
                          dy: T)
                          -> T {
        let zero: T = math::cast(0);

        let attn = math::cast::<_, T>(2.0_f64) - dx * dx - dy * dy;
        if attn > zero {
            let index = perm_table.get2::<isize>([math::cast(xs_floor), math::cast(ys_floor)]);
            let vec = gradient::get2::<T>(index);
            math::pow4(attn) * (dx * vec[0] + dy * vec[1])
        } else {
            zero
        }
    }

    let zero = T::zero();
    let one = T::one();
    let squish_constant: T = math::cast(SQUISH_CONSTANT_2D);

    // Place input coordinates onto grid.
    let stretch_offset = (point[0] + point[1]) * math::cast(STRETCH_CONSTANT_2D);
    let xs = point[0] + stretch_offset;
    let ys = point[1] + stretch_offset;

    // Floor to get grid coordinates of rhombus (stretched square) cell origin.
    let mut xs_floor = xs.floor();
    let mut ys_floor = ys.floor();

    // Skew out to get actual coordinates of rhombus origin. We'll need these later.
    let squish_offset = (xs_floor + ys_floor) * squish_constant;
    let x_floor = xs_floor + squish_offset;
    let y_floor = ys_floor + squish_offset;

    // Compute grid coordinates relative to rhombus origin.
    let xs_frac = xs - xs_floor;
    let ys_frac = ys - ys_floor;

    // Sum those together to get a value that determines which region we're in.
    let frac_sum = xs_frac + ys_frac;

    // Positions relative to origin point (0, 0).
    let mut dx0 = point[0] - x_floor;
    let mut dy0 = point[1] - y_floor;

    let mut value: T = zero;

    // (0, 0) --- (1, 0)
    // |   A     /     |
    // |       /       |
    // |     /     B   |
    // (0, 1) --- (1, 1)

    // Contribution (1, 0)
    let dx1 = dx0 - one - squish_constant;
    let dy1 = dy0 - squish_constant;
    value = value + gradient(perm_table, xs_floor + one, ys_floor, dx1, dy1);

    // Contribution (0, 1)
    let dx2 = dx1 + one;
    let dy2 = dy1 - one;
    value = value + gradient(perm_table, xs_floor, ys_floor + one, dx2, dy2);

    // See the graph for an intuitive explanation; the sum of `x` and `y` is
    // only greater than `1` if we're on Region B.
    if frac_sum > one {
        // Contribution (1, 1)
        xs_floor = xs_floor + one;
        ys_floor = ys_floor + one;
        // We are moving across the diagonal `/`, so we'll need to add by the
        // squish constant
        dx0 = dx1 - squish_constant;
        dy0 = dy2 - squish_constant;
    }

    // Point (0, 0) or (1, 1)
    value = value + gradient(perm_table, xs_floor, ys_floor, dx0, dy0);

    value * math::cast(NORM_CONSTANT_2D)
}

/// 3-dimensional [OpenSimplex Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than `noise::perlin3`.
pub fn open_simplex3<T: Float>(perm_table: &PermutationTable, point: &::Point3<T>) -> T {
    fn gradient<T: Float>(perm_table: &PermutationTable,
                          xs_floor: T,
                          ys_floor: T,
                          zs_floor: T,
                          dx: T,
                          dy: T,
                          dz: T)
                          -> T {
        let zero: T = math::cast(0);

        let attn = math::cast::<_, T>(2.0_f64) - dx * dx - dy * dy - dz * dz;
        if attn > zero {
            let index =
                perm_table.get3::<isize>([math::cast(xs_floor),
                                          math::cast(ys_floor),
                                          math::cast(zs_floor)]);
            let vec = gradient::get3::<T>(index);
            math::pow4(attn) * (dx * vec[0] + dy * vec[1] + dz * vec[2])
        } else {
            zero
        }
    }

    let zero = T::zero();
    let one = T::one();
    let two: T = math::cast(2);
    let squish_constant: T = math::cast(SQUISH_CONSTANT_3D);

    // Place input coordinates on simplectic honeycomb.
    let stretch_offset = (point[0] + point[1] + point[2]) * math::cast(STRETCH_CONSTANT_3D);
    let xs = point[0] + stretch_offset;
    let ys = point[1] + stretch_offset;
    let zs = point[2] + stretch_offset;

    // Floor to get simplectic honeycomb coordinates of rhombohedron
    // (stretched cube) super-cell origin.
    let xsb = xs.floor();
    let ysb = ys.floor();
    let zsb = zs.floor();

    // Skew out to get actual coordinates of rhombohedron origin. We'll need
    // these later.
    let squish_offset = (xsb + ysb + zsb) * squish_constant;
    let xb = xsb + squish_offset;
    let yb = ysb + squish_offset;
    let zb = zsb + squish_offset;

    // Compute simplectic honeycomb coordinates relative to rhombohedral origin.
    let xs_frac = xs - xsb;
    let ys_frac = ys - ysb;
    let zs_frac = zs - zsb;

    // Sum those together to get a value that determines which region we're in.
    let frac_sum = xs_frac + ys_frac + zs_frac;

    // Positions relative to origin point.
    let mut dx0 = point[0] - xb;
    let mut dy0 = point[1] - yb;
    let mut dz0 = point[2] - zb;

    let mut value = zero;

    if frac_sum <= one {
        // We're inside the tetrahedron (3-Simplex) at (0, 0, 0)

        // Contribution at (0, 0, 0)
        value = value + gradient(perm_table, xsb, ysb, zsb, dx0, dy0, dz0);

        // Contribution at (1, 0, 0)
        let dx1 = dx0 - one - squish_constant;
        let dy1 = dy0 - squish_constant;
        let dz1 = dz0 - squish_constant;
        value = value + gradient(perm_table, xsb + one, ysb, zsb, dx1, dy1, dz1);

        // Contribution at (0, 1, 0)
        let dx2 = dx0 - squish_constant;
        let dy2 = dy1 - one;
        let dz2 = dz1;
        value = value + gradient(perm_table, xsb, ysb + one, zsb, dx2, dy2, dz2);

        // Contribution at (0, 0, 1)
        let dx3 = dx2;
        let dy3 = dy1;
        let dz3 = dz1 - one;
        value = value + gradient(perm_table, xsb, ysb, zsb + one, dx3, dy3, dz3);
    } else if frac_sum >= two {
        // We're inside the tetrahedron (3-Simplex) at (1, 1, 1)
        let c0 = one + two * squish_constant;

        // Contribution at (1, 1, 0)
        let dx3 = dx0 - c0;
        let dy3 = dy0 - c0;
        let dz3 = dz0 - c0 + one;
        value = value + gradient(perm_table, xsb + one, ysb + one, zsb, dx3, dy3, dz3);

        // Contribution at (1, 0, 1)
        let dx2 = dx3;
        let dy2 = dy3 + one;
        let dz2 = dz3 - one;
        value = value + gradient(perm_table, xsb + one, ysb, zsb + one, dx2, dy2, dz2);

        // Contribution at (0, 1, 1)
        let dx1 = dx3 + one;
        let dy1 = dy3;
        let dz1 = dz2;
        value = value + gradient(perm_table, xsb, ysb + one, zsb + one, dx1, dy1, dz1);

        // Contribution at (1, 1, 1)
        dx0 = dx3 - squish_constant;
        dy0 = dy3 - squish_constant;
        dz0 = dz2 - squish_constant;
        value = value + gradient(perm_table, xsb + one, ysb + one, zsb + one, dx0, dy0, dz0);
    } else {
        // We're inside the octahedron (Rectified 3-Simplex) inbetween.

        // Contribution at (1, 0, 0)
        let dx1 = dx0 - one - squish_constant;
        let dy1 = dy0 - squish_constant;
        let dz1 = dz0 - squish_constant;
        value = value + gradient(perm_table, xsb + one, ysb, zsb, dx1, dy1, dz1);

        // Contribution at (0, 1, 0)
        let dx2 = dx1 + one;
        let dy2 = dy1 - one;
        let dz2 = dz1;
        value = value + gradient(perm_table, xsb, ysb + one, zsb, dx2, dy2, dz2);

        // Contribution at (0, 0, 1)
        let dx3 = dx2;
        let dy3 = dy1;
        let dz3 = dz1 - one;
        value = value + gradient(perm_table, xsb, ysb, zsb + one, dx3, dy3, dz3);

        // Contribution at (1, 1, 0)
        let dx4 = dx1 - squish_constant;
        let dy4 = dy2 - squish_constant;
        let dz4 = dz1 - squish_constant;
        value = value + gradient(perm_table, xsb + one, ysb + one, zsb, dx4, dy4, dz4);

        // Contribution at (1, 0, 1)
        let dx5 = dx4;
        let dy5 = dy4 + one;
        let dz5 = dz4 - one;
        value = value + gradient(perm_table, xsb + one, ysb, zsb + one, dx5, dy5, dz5);

        // Contribution at (0, 1, 1)
        let dx6 = dx4 + one;
        let dy6 = dy4;
        let dz6 = dz5;
        value = value + gradient(perm_table, xsb, ysb + one, zsb + one, dx6, dy6, dz6);
    }

    return value * math::cast(NORM_CONSTANT_3D);
}

/// 4-dimensional [OpenSimplex Noise](http://uniblock.tumblr.com/post/97868843242/noise)
///
/// This is a slower but higher quality form of gradient noise than
/// `noise::perlin4`.
pub fn open_simplex4<T: Float>(perm_table: &PermutationTable, point: &math::Point4<T>) -> T {
    fn gradient<T: Float>(perm_table: &PermutationTable,
                          vertex: &math::Point4<T>,
                          pos: &math::Point4<T>)
                          -> T {
        let zero = T::zero();
        let attn = math::cast::<_, T>(2.0_f64) - math::dot4(*pos, *pos);
        if attn > zero {
            let index = perm_table.get4::<isize>(math::cast4::<_, isize>(*vertex));
            let vec = gradient::get4::<T>(index);
            math::pow4(attn) * math::dot4(*pos, vec)
        } else {
            zero
        }
    }

    // Constants.
    let stretch_constant: T = math::cast(STRETCH_CONSTANT_4D);
    let squish_constant: T = math::cast(SQUISH_CONSTANT_4D);
    let zero = T::zero();
    let one = T::one();
    let two: T = math::cast(2.0);
    let three: T = math::cast(3.0);

    // Place input coordinates on simplectic honeycomb.
    let stretch_offset = math::fold4(*point, Add::add) * stretch_constant;
    let stretched = math::map4(*point, |v| v + stretch_offset);

    // Floor to get simplectic honeycomb coordinates of rhombo-hypercube
    // super-cell origin.
    let stretched_floor = math::map4(stretched, Float::floor);

    // Skew out to get actual coordinates of stretched rhombo-hypercube origin.
    // We'll need these later.
    let squish_offset = math::fold4(stretched_floor, Add::add) * squish_constant;
    let skewed_floor = math::map4(stretched_floor, |v| v + squish_offset);

    // Compute simplectic honeycomb coordinates relative to rhombo-hypercube
    // origin.
    let rel_coords = math::sub4(stretched, stretched_floor);

    // Sum those together to get a value that determines which region
    // we're in.
    let region_sum = math::fold4(rel_coords, Add::add);

    // Position relative to origin point.
    let mut pos0 = math::sub4(*point, skewed_floor);

    let mut value = zero;
    if region_sum <= one {
        // We're inside the pentachoron (4-Simplex) at (0, 0, 0, 0)

        // Contribution at (0, 0, 0, 0)
        value = value + gradient(perm_table, &stretched_floor, &pos0);

        // Contribution at (1, 0, 0, 0)
        let pos1;
        {
            let vertex = math::add4(stretched_floor, [one, zero, zero, zero]);
            pos1 = math::sub4(pos0,
                              [one + squish_constant,
                               squish_constant,
                               squish_constant,
                               squish_constant]);
            value = value + gradient(perm_table, &vertex, &pos1);
        }

        // Contribution at (0, 1, 0, 0)
        let pos2;
        {
            let vertex = math::add4(stretched_floor, [zero, one, zero, zero]);
            pos2 = [pos1[0] + one, pos1[1] - one, pos1[2], pos1[3]];
            value = value + gradient(perm_table, &vertex, &pos2);
        }

        // Contribution at (0, 0, 1, 0)
        let pos3;
        {
            let vertex = math::add4(stretched_floor, [zero, zero, one, zero]);
            pos3 = [pos2[0], pos1[1], pos1[2] - one, pos1[3]];
            value = value + gradient(perm_table, &vertex, &pos3);
        }

        // Contribution at (0, 0, 0, 1)
        let pos4;
        {
            let vertex = math::add4(stretched_floor, [zero, zero, zero, one]);
            pos4 = [pos2[0], pos1[1], pos1[2], pos1[3] - one];
            value = value + gradient(perm_table, &vertex, &pos4);
        }
    } else if region_sum >= three {
        // We're inside the pentachoron (4-Simplex) at (1, 1, 1, 1)
        let squish_constant_3 = three * squish_constant;

        // Contribution at (1, 1, 1, 0)
        let pos4;
        {
            let vertex = math::add4(stretched_floor, [one, one, one, zero]);
            pos4 = math::sub4(pos0,
                              [one + squish_constant_3,
                               one + squish_constant_3,
                               one + squish_constant_3,
                               squish_constant_3]);
            value = value + gradient(perm_table, &vertex, &pos4);
        }

        // Contribution at (1, 1, 0, 1)
        let pos3;
        {
            let vertex = math::add4(stretched_floor, [one, one, zero, one]);
            pos3 = [pos4[0], pos4[1], pos4[2] + one, pos4[3] - one];
            value = value + gradient(perm_table, &vertex, &pos3);
        }

        // Contribution at (1, 0, 1, 1)
        let pos2;
        {
            let vertex = math::add4(stretched_floor, [one, zero, one, one]);
            pos2 = [pos4[0], pos4[1] + one, pos4[2], pos3[3]];
            value = value + gradient(perm_table, &vertex, &pos2);
        }

        // Contribution at (0, 1, 1, 1)
        let pos1;
        {
            let vertex = math::add4(stretched_floor, [zero, one, one, one]);
            pos1 = [pos0[0] - squish_constant_3, pos4[1], pos4[2], pos3[3]];
            value = value + gradient(perm_table, &vertex, &pos1);
        }

        // Contribution at (1, 1, 1, 1)
        {
            let vertex = math::add4(stretched_floor, [one, one, one, one]);
            pos0[0] = pos4[0] - squish_constant;
            pos0[1] = pos4[1] - squish_constant;
            pos0[2] = pos4[2] - squish_constant;
            pos0[3] = pos3[3] - squish_constant;
            value = value + gradient(perm_table, &vertex, &pos0);
        }
    } else if region_sum <= two {
        // We're inside the first dispentachoron (Rectified 4-Simplex)

        // Contribution at (1, 0, 0, 0)
        let pos1;
        {
            let vertex = math::add4(stretched_floor, [one, zero, zero, zero]);
            pos1 = math::sub4(pos0,
                              [one + squish_constant,
                               squish_constant,
                               squish_constant,
                               squish_constant]);
            value = value + gradient(perm_table, &vertex, &pos1);
        }

        // Contribution at (0, 1, 0, 0)
        let pos2;
        {
            let vertex = math::add4(stretched_floor, [zero, one, zero, zero]);
            pos2 = [pos1[0] + one, pos1[1] - one, pos1[2], pos1[3]];
            value = value + gradient(perm_table, &vertex, &pos2);
        }

        // Contribution at (0, 0, 1, 0)
        let pos3;
        {
            let vertex = math::add4(stretched_floor, [zero, zero, one, zero]);
            pos3 = [pos2[0], pos1[1], pos1[2] - one, pos1[3]];
            value = value + gradient(perm_table, &vertex, &pos3);
        }

        // Contribution at (0, 0, 0, 1)
        let pos4;
        {
            let vertex = math::add4(stretched_floor, [zero, zero, zero, one]);
            pos4 = [pos2[0], pos1[1], pos1[2], pos1[3] - one];
            value = value + gradient(perm_table, &vertex, &pos4);
        }

        // Contribution at (1, 1, 0, 0)
        let pos5;
        {
            let vertex = math::add4(stretched_floor, [one, one, zero, zero]);
            pos5 = [pos1[0] - squish_constant,
                    pos2[1] - squish_constant,
                    pos1[2] - squish_constant,
                    pos1[3] - squish_constant];
            value = value + gradient(perm_table, &vertex, &pos5);
        }

        // Contribution at (1, 0, 1, 0)
        let pos6;
        {
            let vertex = math::add4(stretched_floor, [one, zero, one, zero]);
            pos6 = [pos5[0], pos5[1] + one, pos5[2] - one, pos5[3]];
            value = value + gradient(perm_table, &vertex, &pos6);
        }

        // Contribution at (1, 0, 0, 1)
        let pos7;
        {
            let vertex = math::add4(stretched_floor, [one, zero, zero, one]);
            pos7 = [pos5[0], pos6[1], pos5[2], pos5[3] - one];
            value = value + gradient(perm_table, &vertex, &pos7);
        }

        // Contribution at (0, 1, 1, 0)
        let pos8;
        {
            let vertex = math::add4(stretched_floor, [zero, one, one, zero]);
            pos8 = [pos5[0] + one, pos5[1], pos6[2], pos5[3]];
            value = value + gradient(perm_table, &vertex, &pos8);
        }

        // Contribution at (0, 1, 0, 1)
        let pos9;
        {
            let vertex = math::add4(stretched_floor, [zero, one, zero, one]);
            pos9 = [pos8[0], pos5[1], pos5[2], pos7[3]];
            value = value + gradient(perm_table, &vertex, &pos9);
        }

        // Contribution at (0, 0, 1, 1)
        let pos10;
        {
            let vertex = math::add4(stretched_floor, [zero, zero, one, one]);
            pos10 = [pos8[0], pos6[1], pos6[2], pos7[3]];
            value = value + gradient(perm_table, &vertex, &pos10);
        }
    } else {
        // We're inside the second dispentachoron (Rectified 4-Simplex)
        let squish_constant_3 = three * squish_constant;

        // Contribution at (1, 1, 1, 0)
        let pos4;
        {
            let vertex = math::add4(stretched_floor, [one, one, one, zero]);
            pos4 = math::sub4(pos0,
                              [one + squish_constant_3,
                               one + squish_constant_3,
                               one + squish_constant_3,
                               squish_constant_3]);
            value = value + gradient(perm_table, &vertex, &pos4);
        }

        // Contribution at (1, 1, 0, 1)
        let pos3;
        {
            let vertex = math::add4(stretched_floor, [one, one, zero, one]);
            pos3 = [pos4[0], pos4[1], pos4[2] + one, pos4[3] - one];
            value = value + gradient(perm_table, &vertex, &pos3);
        }

        // Contribution at (1, 0, 1, 1)
        let pos2;
        {
            let vertex = math::add4(stretched_floor, [one, zero, one, one]);
            pos2 = [pos4[0], pos4[1] + one, pos4[2], pos3[3]];
            value = value + gradient(perm_table, &vertex, &pos2);
        }

        // Contribution at (0, 1, 1, 1)
        let pos1;
        {
            let vertex = math::add4(stretched_floor, [zero, one, one, one]);
            pos1 = [pos4[0] + one, pos4[1], pos4[2], pos3[3]];
            value = value + gradient(perm_table, &vertex, &pos1);
        }

        // Contribution at (1, 1, 0, 0)
        let pos5;
        {
            let vertex = math::add4(stretched_floor, [one, one, zero, zero]);
            pos5 = [pos4[0] + squish_constant,
                    pos4[1] + squish_constant,
                    pos3[2] + squish_constant,
                    pos4[3] + squish_constant];
            value = value + gradient(perm_table, &vertex, &pos5);
        }

        // Contribution at (1, 0, 1, 0)
        let pos6;
        {
            let vertex = math::add4(stretched_floor, [one, zero, one, zero]);
            pos6 = [pos5[0], pos5[1] + one, pos5[2] - one, pos5[3]];
            value = value + gradient(perm_table, &vertex, &pos6);
        }

        // Contribution at (1, 0, 0, 1)
        let pos7;
        {
            let vertex = math::add4(stretched_floor, [one, zero, zero, one]);
            pos7 = [pos5[0], pos6[1], pos5[2], pos5[3] - one];
            value = value + gradient(perm_table, &vertex, &pos7);
        }

        // Contribution at (0, 1, 1, 0)
        let pos8;
        {
            let vertex = math::add4(stretched_floor, [zero, one, one, zero]);
            pos8 = [pos5[0] + one, pos5[1], pos6[2], pos5[3]];
            value = value + gradient(perm_table, &vertex, &pos8);
        }

        // Contribution at (0, 1, 0, 1)
        let pos9;
        {
            let vertex = math::add4(stretched_floor, [zero, one, zero, one]);
            pos9 = [pos8[0], pos5[1], pos5[2], pos7[3]];
            value = value + gradient(perm_table, &vertex, &pos9);
        }

        // Contribution at (0, 0, 1, 1)
        let pos10;
        {
            let vertex = math::add4(stretched_floor, [zero, zero, one, one]);
            pos10 = [pos8[0], pos6[1], pos6[2], pos7[3]];
            value = value + gradient(perm_table, &vertex, &pos10);
        }
    }

    value * math::cast(NORM_CONSTANT_4D)
}
