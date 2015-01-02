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

/*
    Note that this is NOT Ken Perlin's simplex noise, as that is patent encumbered.
    Instead, these functions use the OpenSimplex algorithm, as detailed here:
    http://uniblock.tumblr.com/post/97868843242/noise
*/

use std::num::{cast, Float};

use {math, Seed};
use gradients::{gradient2, gradient3};

const STRETCH_CONSTANT_2D: f64 = -0.211324865405187; //(1/sqrt(2+1)-1)/2;
const SQUISH_CONSTANT_2D: f64 = 0.366025403784439; //(sqrt(2+1)-1)/2;
const STRETCH_CONSTANT_3D: f64 = -1.0 / 6.0; //(1/Math.sqrt(3+1)-1)/3;
const SQUISH_CONSTANT_3D: f64 = 1.0 / 3.0; //(Math.sqrt(3+1)-1)/3;

const NORM_CONSTANT_2D: f32 = 14.0;
const NORM_CONSTANT_3D: f32 = 14.0;

pub fn simplex2<T: Float>(seed: &Seed, point: &::Point2<T>) -> T {
    fn gradient<T: Float>(seed: &Seed, xs_floor: T, ys_floor: T, dx: T, dy: T) -> T {
        let attn = math::cast::<_, T>(2u) - dx * dx - dy * dy;
        if attn > Float::zero() {
            let index = seed.get2(xs_floor.to_int().unwrap(), ys_floor.to_int().unwrap());
            let vec = gradient2::<T>(index);
            let attn2 = attn * attn;
            attn2 * attn2 * (dx * vec[0] + dy * vec[1])
        } else {
            Float::zero()
        }
    }

    let zero: T = math::cast(0u);
    let one: T = math::cast(1u);
    let two: T = math::cast(2u);
    let squish_constant: T = math::cast(SQUISH_CONSTANT_2D);

    //Place input coordinates onto grid.
    let stretch_offset = (point[0] + point[1]) * math::cast(STRETCH_CONSTANT_2D);
    let xs = point[0] + stretch_offset;
    let ys = point[1] + stretch_offset;

    //Floor to get grid coordinates of rhombus (stretched square) super-cell origin.
    let mut xs_floor = xs.floor();
    let mut ys_floor = ys.floor();

    //Skew out to get actual coordinates of rhombus origin. We'll need these later.
    let squish_offset = (xs_floor + ys_floor) * squish_constant;
    let x_floor = xs_floor + squish_offset;
    let y_floor = ys_floor + squish_offset;

    //Compute grid coordinates relative to rhombus origin.
    let xs_frac = xs - xs_floor;
    let ys_frac = ys - ys_floor;

    //Sum those together to get a value that determines which region we're in.
    let frac_sum = xs_frac + ys_frac;

    //Positions relative to origin point.
    let mut dx0 = point[0] - x_floor;
    let mut dy0 = point[1] - y_floor;

    let mut value: T = zero;

    //Contribution (1,0)
    let dx1 = dx0 - one - squish_constant;
    let dy1 = dy0 - zero - squish_constant;
    value = value + gradient(seed, xs_floor + one, ys_floor + zero, dx1, dy1);

    //Contribution (0,1)
    let dx2 = dx0 - zero - squish_constant;
    let dy2 = dy0 - one - squish_constant;
    value = value + gradient(seed, xs_floor + zero, ys_floor + one, dx2, dy2);

    let (dx_ext, dy_ext, xsv_ext, ysv_ext) = if frac_sum <= one {
        //We're inside the triangle (2-Simplex) at (0,0)
        let z_frac = one - frac_sum;
        if z_frac > xs_frac || z_frac > ys_frac { //(0,0) is one of the closest two triangular vertices
            if xs_frac > ys_frac {
                (dx0 - one, dy0 + one, xs_floor + one, ys_floor - one)
            } else {
                (dx0 + one, dy0 - one, xs_floor - one, ys_floor + one)
            }
        } else { //(1,0) and (0,1) are the closest two vertices.
            (dx0 - one - two * squish_constant, dy0 - one - two * squish_constant, xs_floor + one, ys_floor + one)
        }
    } else {
        //We're inside the triangle (2-Simplex) at (1,1)
        let z_frac = two - frac_sum;
        if z_frac < xs_frac || z_frac < ys_frac { //(0,0) is one of the closest two triangular vertices
            if xs_frac > ys_frac {
                (dx0 - two - two * squish_constant, dy0 + zero - two * squish_constant, xs_floor + two, ys_floor + zero)
            } else {
                (dx0 + zero - two * squish_constant, dy0 - two - two * squish_constant, xs_floor + zero, ys_floor + two)
            }
        } else { //(1,0) and (0,1) are the closest two vertices.
            (dx0, dy0, xs_floor, ys_floor)
        }
    };

    if frac_sum > one {
        xs_floor = xs_floor + one;
        ys_floor = ys_floor + one;
        dx0 = dx0 - one - two * squish_constant;
        dy0 = dy0 - one - two * squish_constant;
    }

    //Contribution (0,0) or (1,1)
    value = value + gradient(seed, xs_floor, ys_floor, dx0, dy0);

    //Extra Vertex
    value = value + gradient(seed, xsv_ext, ysv_ext, dx_ext, dy_ext);

    value / math::cast(NORM_CONSTANT_2D)
}

pub fn simplex3<T: Float>(seed: &Seed, point: &::Point3<T>) -> T {
    fn gradient<T: Float>(seed: &Seed, xs_floor: T, ys_floor: T, zs_floor: T, dx: T, dy: T, dz: T) -> T {
        let attn = math::cast::<_, T>(2u) - dx * dx - dy * dy - dz * dz;
        if attn > Float::zero() {
            let index = seed.get3(xs_floor.to_int().unwrap(), ys_floor.to_int().unwrap(), zs_floor.to_int().unwrap());
            let vec = gradient3::<T>(index);
            let attn2 = attn * attn;
            attn2 * attn2 * (dx * vec[0] + dy * vec[1] + dz * vec[2])
        } else {
            Float::zero()
        }
    }

    let zero: T = math::cast(0.0f32);
    let one: T = math::cast(1.0f32);
    let two: T = math::cast(2.0f32);
    let three: T = math::cast(3.0f32);
    let squish_constant: T = math::cast(SQUISH_CONSTANT_3D);

    //Place input coordinates on simplectic honeycomb.
    let stretch_offset = (point[0] + point[1] + point[2]) * math::cast(STRETCH_CONSTANT_3D);
    let xs = point[0] + stretch_offset;
    let ys = point[1] + stretch_offset;
    let zs = point[2] + stretch_offset;

    //Floor to get simplectic honeycomb coordinates of rhombohedron (stretched cube) super-cell origin.
    let xsb = xs.floor();
    let ysb = ys.floor();
    let zsb = zs.floor();

    //Skew out to get actual coordinates of rhombohedron origin. We'll need these later.
    let squish_offset = (xsb + ysb + zsb) * squish_constant;
    let xb = xsb + squish_offset;
    let yb = ysb + squish_offset;
    let zb = zsb + squish_offset;

    //Compute simplectic honeycomb coordinates relative to rhombohedral origin.
    let xins = xs - xsb;
    let yins = ys - ysb;
    let zins = zs - zsb;

    //Sum those together to get a value that determines which region we're in.
    let in_sum = xins + yins + zins;

    //Positions relative to origin point.
    let mut dx0 = point[0] - xb;
    let mut dy0 = point[1] - yb;
    let mut dz0 = point[2] - zb;

    //We'll be defining these inside the next block and using them afterwards.
    let mut dx_ext0;
    let mut dy_ext0;
    let mut dz_ext0;
    let mut dx_ext1;
    let mut dy_ext1;
    let mut dz_ext1;
    let mut xsv_ext0;
    let mut ysv_ext0;
    let mut zsv_ext0;
    let mut xsv_ext1;
    let mut ysv_ext1;
    let mut zsv_ext1;

    let mut value = zero;
    if in_sum <= one { //We're inside the tetrahedron (3-Simplex) at (0,0,0)
        //Determine which two of (0,0,1), (0,1,0), (1,0,0) are closest.
        let mut a_point = 0x01u8;
        let mut a_score = xins;
        let mut b_point = 0x02u8;
        let mut b_score = yins;
        if a_score >= b_score && zins > b_score {
            b_score = zins;
            b_point = 0x04;
        } else if a_score < b_score && zins > a_score {
            a_score = zins;
            a_point = 0x04;
        }

        //Now we determine the two lattice points not part of the tetrahedron that may contribute.
        //This depends on the closest two tetrahedral vertices, including (0,0,0)
        let wins = one - in_sum;
        if wins > a_score || wins > b_score { //(0,0,0) is one of the closest two tetrahedral vertices.
            let c = if b_score > a_score { b_point } else { a_point }; //Our other closest vertex is the closest out of a and b.

            if (c & 0x01) == 0 {
                xsv_ext0 = xsb - one;
                xsv_ext1 = xsb;
                dx_ext0 = dx0 + one;
                dx_ext1 = dx0;
            } else {
                xsv_ext0 = xsb + one;
                xsv_ext1 =  xsv_ext0;
                dx_ext0 = dx0 - one;
                dx_ext1 = dx_ext0;
            }

            if (c & 0x02) == 0 {
                ysv_ext0 = ysb;
                ysv_ext1 = ysb;
                dy_ext0 = dy0;
                dy_ext1 = dy0;
                if (c & 0x01) == 0 {
                    ysv_ext1 = ysv_ext1 - one;
                    dy_ext1 = dy_ext1 + one;
                } else {
                    ysv_ext0 = ysv_ext0 - one;
                    dy_ext0 = dy_ext0 + one;
                }
            } else {
                ysv_ext0 = ysb + one;
                ysv_ext1 = ysv_ext0;
                dy_ext0 = dy0 - one;
                dy_ext1 =  dy_ext0;
            }

            if (c & 0x04) == 0 {
                zsv_ext0 = zsb;
                zsv_ext1 = zsb - one;
                dz_ext0 = dz0;
                dz_ext1 = dz0 + one;
            } else {
                zsv_ext0 = zsb + one;
                zsv_ext1 = zsv_ext0;
                dz_ext0 = dz0 - one;
                dz_ext1 = dz_ext0;
            }
        } else { //(0,0,0) is not one of the closest two tetrahedral vertices.
            let c = (a_point | b_point) as u8; //Our two extra vertices are determined by the closest two.

            if (c & 0x01) == 0 {
                xsv_ext0 = xsb;
                xsv_ext1 = xsb - one;
                dx_ext0 = dx0 - two * squish_constant;
                dx_ext1 = dx0 + one - squish_constant;
            } else {
                xsv_ext0 = xsb + one;
                xsv_ext1 = xsv_ext0;
                dx_ext0 = dx0 - one - two * squish_constant;
                dx_ext1 = dx0 - one - squish_constant;
            }

            if (c & 0x02) == 0 {
                ysv_ext0 = ysb;
                ysv_ext1 = ysb - one;
                dy_ext0 = dy0 - two * squish_constant;
                dy_ext1 = dy0 + one - squish_constant;
            } else {
                ysv_ext0 = ysb + one;
                ysv_ext1 = ysv_ext0;
                dy_ext0 = dy0 - one - two * squish_constant;
                dy_ext1 = dy0 - one - squish_constant;
            }

            if (c & 0x04) == 0 {
                zsv_ext0 = zsb;
                zsv_ext1 = zsb - one;
                dz_ext0 = dz0 - two * squish_constant;
                dz_ext1 = dz0 + one - squish_constant;
            } else {
                zsv_ext0 = zsb + one;
                zsv_ext1 = zsv_ext0;
                dz_ext0 = dz0 - one - two * squish_constant;
                dz_ext1 = dz0 - one - squish_constant;
            }
        }

        //Contribution (0,0,0)
        value = value + gradient(seed, xsb, ysb, zsb, dx0, dy0, dz0);

        //Contribution (1,0,0)
        let dx1 = dx0 - one - squish_constant;
        let dy1 = dy0 - zero - squish_constant;
        let dz1 = dz0 - zero - squish_constant;
        value = value + gradient(seed, xsb + one, ysb, zsb, dx1, dy1, dz1);

        //Contribution (0,1,0)
        let dx2 = dx0 - zero - squish_constant;
        let dy2 = dy0 - one - squish_constant;
        let dz2 = dz1;
        value = value + gradient(seed, xsb, ysb + one, zsb, dx2, dy2, dz2);

        //Contribution (0,0,1)
        let dx3 = dx2;
        let dy3 = dy1;
        let dz3 = dz0 - one - squish_constant;
        value = value + gradient(seed, xsb, ysb, zsb + one, dx3, dy3, dz3);
    } else if in_sum >= two { //We're inside the tetrahedron (3-Simplex) at (1,11)
        //Determine which two tetrahedral vertices are the closest, out of (1,1,0), (1,0,1), (0,1,1) but not (1,1,1).
        let mut a_point = 0x06u8;
        let mut a_score = xins;
        let mut b_point = 0x05u8;
        let mut b_score = yins;
        if a_score <= b_score && zins < b_score {
            b_score = zins;
            b_point = 0x03;
        } else if a_score > b_score && zins < a_score {
            a_score = zins;
            a_point = 0x03;
        }

        //Now we determine the two lattice points not part of the tetrahedron that may contribute.
        //This depends on the closest two tetrahedral vertices, including (1,1,1)
        let wins: T = three - in_sum;
        if wins < a_score || wins < b_score { //(1,1,1) is one of the closest two tetrahedral vertics.
            let c = if b_score < a_score { b_point } else { a_point }; //Our other closest vertex is the closest out of a and b.

            if (c & 0x01) != 0 {
                xsv_ext0 = xsb + two;
                xsv_ext1 = xsb + one;
                dx_ext0 = dx0 - two - three * squish_constant;
                dx_ext1 = dx0 - one - three * squish_constant;
            } else {
                xsv_ext0 = xsb;
                xsv_ext1 = xsv_ext0;
                dx_ext0 = dx0 - three * squish_constant;
                dx_ext1 = dx_ext0;
            }

            if (c & 0x02) != 0 {
                ysv_ext0 = ysb + one;
                ysv_ext1 = ysv_ext0;
                dy_ext0 = dy0 - one - three * squish_constant;
                dy_ext1 = dy_ext0;
                if (c & 0x01) != 0 {
                    ysv_ext1 = ysv_ext1 + one;
                    dy_ext1 = dy_ext1 - one;
                } else {
                    ysv_ext0 = ysv_ext0 + one;
                    dy_ext0 = dy_ext0 - one;
                }
            } else {
                ysv_ext0 = ysb;
                ysv_ext1 = ysb;
                dy_ext0 = dy0 - three * squish_constant;
                dy_ext1 = dy_ext0;
            }

            if (c & 0x04) != 0 {
                zsv_ext0 = zsb + one;
                zsv_ext1 = zsb + two;
                dz_ext0 = dz0 - one - three * squish_constant;
                dz_ext1 = dz0 - two - three * squish_constant;
            } else {
                zsv_ext0 = zsb;
                zsv_ext1 = zsb;
                dz_ext0 = dz0 - three * squish_constant;
                dz_ext1 = dz_ext0;
            }
        } else { //(1,1,1) is not one of the closest two tetrahedral vertices.
            let c = (a_point & b_point) as u8; //Our two extra vertices are determined by the closest two.

            if (c & 0x01) != 0 {
                xsv_ext0 = xsb + one;
                xsv_ext1 = xsb + two;
                dx_ext0 = dx0 - one - squish_constant;
                dx_ext1 = dx0 - two - two * squish_constant;
            } else {
                xsv_ext0 = xsb;
                xsv_ext1 = xsb;
                dx_ext0 = dx0 - squish_constant;
                dx_ext1 = dx0 - two * squish_constant;
            }

            if (c & 0x02) != 0 {
                ysv_ext0 = ysb + one;
                ysv_ext1 = ysb + two;
                dy_ext0 = dy0 - one - squish_constant;
                dy_ext1 = dy0 - two - two * squish_constant;
            } else {
                ysv_ext0 = ysb;
                ysv_ext1 = ysb;
                dy_ext0 = dy0 - squish_constant;
                dy_ext1 = dy0 - two * squish_constant;
            }

            if (c & 0x04) != 0 {
                zsv_ext0 = zsb + one;
                zsv_ext1 = zsb + two;
                dz_ext0 = dz0 - one - squish_constant;
                dz_ext1 = dz0 - two - two * squish_constant;
            } else {
                zsv_ext0 = zsb;
                zsv_ext1 = zsb;
                dz_ext0 = dz0 - squish_constant;
                dz_ext1 = dz0 - two * squish_constant;
            }
        }

        //Contribution (1,1,0)
        let dx3 = dx0 - one - two * squish_constant;
        let dy3 = dy0 - one - two * squish_constant;
        let dz3 = dz0 - zero - two * squish_constant;
        value = value + gradient(seed, xsb + one, ysb + one, zsb, dx3, dy3, dz3);

        //Contribution (1,0,1)
        let dx2 = dx3;
        let dy2 = dy0 - zero - two * squish_constant;
        let dz2 = dz0 - one - two * squish_constant;
        value = value + gradient(seed, xsb + one, ysb, zsb + one, dx2, dy2, dz2);

        //Contribution (0,1,1)
        let dx1 = dx0 - zero - two * squish_constant;
        let dy1 = dy3;
        let dz1 = dz2;
        value = value + gradient(seed, xsb, ysb + one, zsb + one, dx1, dy1, dz1);

        //Contribution (1,1,1)
        dx0 = dx0 - one - three * squish_constant;
        dy0 = dy0 - one - three * squish_constant;
        dz0 = dz0 - one - three * squish_constant;
        value = value + gradient(seed, xsb + one, ysb + one, zsb + one, dx0, dy0, dz0);
    } else { //We're inside the octahedron (Rectified 3-Simplex) in between.
        let mut a_score;
        let mut a_point: u8;
        let mut a_is_further_side;
        let mut b_score;
        let mut b_point: u8;
        let mut b_is_further_side;

        //Decide between point (0,0,1) and (1,1,0) as closest
        let p1 = xins + yins;
        if p1 > one {
            a_score = p1 - one;
            a_point = 0x03;
            a_is_further_side = true;
        } else {
            a_score = one - p1;
            a_point = 0x04;
            a_is_further_side = false;
        }

        //Decide between point (0,1,0) and (1,0,1) as closest
        let p2 = xins + zins;
        if p2 > one {
            b_score = p2 - one;
            b_point = 0x05;
            b_is_further_side = true;
        } else {
            b_score = one - p2;
            b_point = 0x02;
            b_is_further_side = false;
        }

        //The closest out of the two (1,0,0) and (0,1,1) will replace the furthest out of the two decided above, if closer.
        let p3 = yins + zins;
        if p3 > one {
            let score = p3 - one;
            if a_score <= b_score && a_score < score {
                a_point = 0x06;
                a_is_further_side = true;
            } else if a_score > b_score && b_score < score {
                b_point = 0x06;
                b_is_further_side = true;
            }
        } else {
            let score = one - p3;
            if a_score <= b_score && a_score < score {
                a_point = 0x01;
                a_is_further_side = false;
            } else if a_score > b_score && b_score < score {
                b_point = 0x01;
                b_is_further_side = false;
            }
        }

        //Where each of the two closest points are determines how the extra two vertices are calculated.
        if a_is_further_side == b_is_further_side {
            if a_is_further_side { //Both closest points on (1,1,1) sde
                //One of the two extra points is (1,1,1)
                dx_ext0 = dx0 - one - three * squish_constant;
                dy_ext0 = dy0 - one - three * squish_constant;
                dz_ext0 = dz0 - one - three * squish_constant;
                xsv_ext0 = xsb + one;
                ysv_ext0 = ysb + one;
                zsv_ext0 = zsb + one;

                //Other extra point is based on the shared axis.
                let c = (a_point & b_point) as u8;
                if (c & 0x01) != 0 {
                    dx_ext1 = dx0 - two - two * squish_constant;
                    dy_ext1 = dy0 - two * squish_constant;
                    dz_ext1 = dz0 - two * squish_constant;
                    xsv_ext1 = xsb + two;
                    ysv_ext1 = ysb;
                    zsv_ext1 = zsb;
                } else if (c & 0x02) != 0 {
                    dx_ext1 = dx0 - two * squish_constant;
                    dy_ext1 = dy0 - two - two * squish_constant;
                    dz_ext1 = dz0 - two * squish_constant;
                    xsv_ext1 = xsb;
                    ysv_ext1 = ysb + two;
                    zsv_ext1 = zsb;
                } else {
                    dx_ext1 = dx0 - two * squish_constant;
                    dy_ext1 = dy0 - two * squish_constant;
                    dz_ext1 = dz0 - two - two * squish_constant;
                    xsv_ext1 = xsb;
                    ysv_ext1 = ysb;
                    zsv_ext1 = zsb + two;
                }
            } else {//Both closest points on (0,0,0) side
                //One of the two extra points is (0,0,0)
                dx_ext0 = dx0;
                dy_ext0 = dy0;
                dz_ext0 = dz0;
                xsv_ext0 = xsb;
                ysv_ext0 = ysb;
                zsv_ext0 = zsb;

                //Other extra point is based on the omitted axis.
                let c = (a_point | b_point) as u8;
                if (c & 0x01) == 0 {
                    dx_ext1 = dx0 + one - squish_constant;
                    dy_ext1 = dy0 - one - squish_constant;
                    dz_ext1 = dz0 - one - squish_constant;
                    xsv_ext1 = xsb - one;
                    ysv_ext1 = ysb + one;
                    zsv_ext1 = zsb + one;
                } else if (c & 0x02) == 0 {
                    dx_ext1 = dx0 - one - squish_constant;
                    dy_ext1 = dy0 + one - squish_constant;
                    dz_ext1 = dz0 - one - squish_constant;
                    xsv_ext1 = xsb + one;
                    ysv_ext1 = ysb - one;
                    zsv_ext1 = zsb + one;
                } else {
                    dx_ext1 = dx0 - one - squish_constant;
                    dy_ext1 = dy0 - one - squish_constant;
                    dz_ext1 = dz0 + one - squish_constant;
                    xsv_ext1 = xsb + one;
                    ysv_ext1 = ysb + one;
                    zsv_ext1 = zsb - one;
                }
            }
        } else { //One point on (0,0,0) side, one point on (1,1,1) side
            let c1;
            let c2;
            if a_is_further_side {
                c1 = a_point;
                c2 = b_point;
            } else {
                c1 = b_point;
                c2 = a_point;
            }

            //One contribution is a permutation of (1,1,-1)
            if (c1 & 0x01) == 0 {
                dx_ext0 = dx0 + one - squish_constant;
                dy_ext0 = dy0 - one - squish_constant;
                dz_ext0 = dz0 - one - squish_constant;
                xsv_ext0 = xsb - one;
                ysv_ext0 = ysb + one;
                zsv_ext0 = zsb + one;
            } else if (c1 & 0x02) == 0 {
                dx_ext0 = dx0 - one - squish_constant;
                dy_ext0 = dy0 + one - squish_constant;
                dz_ext0 = dz0 - one - squish_constant;
                xsv_ext0 = xsb + one;
                ysv_ext0 = ysb - one;
                zsv_ext0 = zsb + one;
            } else {
                dx_ext0 = dx0 - one - squish_constant;
                dy_ext0 = dy0 - one - squish_constant;
                dz_ext0 = dz0 + one - squish_constant;
                xsv_ext0 = xsb + one;
                ysv_ext0 = ysb + one;
                zsv_ext0 = zsb - one;
            }

            //One contribution is a permutation of (0,0,2)
            dx_ext1 = dx0 - two * squish_constant;
            dy_ext1 = dy0 - two * squish_constant;
            dz_ext1 = dz0 - two * squish_constant;
            xsv_ext1 = xsb;
            ysv_ext1 = ysb;
            zsv_ext1 = zsb;
            if (c2 & 0x01) != 0 {
                dx_ext1 = dx_ext1 - two;
                xsv_ext1 = xsv_ext1 + two;
            } else if (c2 & 0x02) != 0 {
                dy_ext1 = dy_ext1 - two;
                ysv_ext1 = ysv_ext1 + two;
            } else {
                dz_ext1 = dz_ext1 - two;
                zsv_ext1 = zsv_ext1 + two;
            }
        }

        //Contribution (1,0,0)
        let dx1 = dx0 - one - squish_constant;
        let dy1 = dy0 - zero - squish_constant;
        let dz1 = dz0 - zero - squish_constant;
        value = value + gradient(seed, xsb + one, ysb, zsb, dx1, dy1, dz1);

        //Contribution (0,1,0)
        let dx2 = dx0 - zero - squish_constant;
        let dy2 = dy0 - one - squish_constant;
        let dz2 = dz1;
        value = value + gradient(seed, xsb, ysb + one, zsb, dx2, dy2, dz2);

        //Contribution (0,0,1)
        let dx3 = dx2;
        let dy3 = dy1;
        let dz3 = dz0 - one - squish_constant;
        value = value + gradient(seed, xsb, ysb, zsb + one, dx3, dy3, dz3);

        //Contribution (1,1,0)
        let dx4 = dx0 - one - two * squish_constant;
        let dy4 = dy0 - one - two * squish_constant;
        let dz4 = dz0 - zero - two * squish_constant;
        value = value + gradient(seed, xsb + one, ysb + one, zsb, dx4, dy4, dz4);

        //Contribution (1,0,1)
        let dx5 = dx4;
        let dy5 = dy0 - zero - two * squish_constant;
        let dz5 = dz0 - one - two * squish_constant;
        value = value + gradient(seed, xsb + one, ysb, zsb + one, dx5, dy5, dz5);

        //Contribution (0,1,1)
        let dx6 = dx0 - zero - two * squish_constant;
        let dy6 = dy4;
        let dz6 = dz5;
        value = value + gradient(seed, xsb, ysb + one, zsb + one, dx6, dy6, dz6);
    }

    //First extra vertex
    value = value + gradient(seed, xsv_ext0, ysv_ext0, zsv_ext0, dx_ext0, dy_ext0, dz_ext0);

    //Second extra vertex
    value = value + gradient(seed, xsv_ext1, ysv_ext1, zsv_ext1, dx_ext1, dy_ext1, dz_ext1);

    return value / math::cast(NORM_CONSTANT_3D);
}
