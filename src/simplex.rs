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

use seed::Seed;
use gradients::GRADIENT2;

const STRETCH_CONSTANT_2D: f64 = -0.211324865405187; //(1/sqrt(2+1)-1)/2;
const SQUISH_CONSTANT_2D: f64 = 0.366025403784439; //(sqrt(2+1)-1)/2;

const NORM_CONSTANT_2D: f32 = 14.0;

fn get_simplex2_gradient<T: Float>(seed: &Seed, xs_floor: T, ys_floor: T, dx: T, dy: T) -> T {
    let two: T = cast(2.0f32).unwrap();
    let attn = two - dx * dx - dy * dy;
    if attn > cast(0.0f32).unwrap() {
        let index = seed.get2(xs_floor.to_int().unwrap(), ys_floor.to_int().unwrap()) % GRADIENT2.len();
        let vec = GRADIENT2[index];
        let attn2 = attn * attn;
        return attn2 * attn2 * (dx * cast(vec[0]).unwrap() + dy * cast(vec[1]).unwrap());
    }
    return cast(0.0f32).unwrap();
}

pub fn simplex2<T: Float>(seed: &Seed, point: &::Point2<T>) -> f32 {
    let zero: T = cast(0.0f32).unwrap();
    let one: T = cast(1.0f32).unwrap();
    let two: T = cast(2.0f32).unwrap();
    let squish_constant: T = cast(SQUISH_CONSTANT_2D).unwrap();

    //Place input coordinates onto grid.
    let stretch_offset = (point[0] + point[1]) * cast(STRETCH_CONSTANT_2D).unwrap();
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

    let mut value = zero;

    //Contribution (1,0)
    let dx1 = dx0 - one - squish_constant;
    let dy1 = dy0 - zero - squish_constant;
    value = value + get_simplex2_gradient(seed, xs_floor + one, ys_floor + zero, dx1, dy1);

    //Contribution (0,1)
    let dx2 = dx0 - zero - squish_constant;
    let dy2 = dy0 - one - squish_constant;
    value = value + get_simplex2_gradient(seed, xs_floor + zero, ys_floor + one, dx2, dy2);

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
    value = value + get_simplex2_gradient(seed, xs_floor, ys_floor, dx0, dy0);

    //Extra Vertex
    value = value + get_simplex2_gradient(seed, xsv_ext, ysv_ext, dx_ext, dy_ext);

    return value.to_f32().unwrap() / NORM_CONSTANT_2D;
}

#[cfg(test)]
mod tests {
    use seed::Seed;
    use simplex::simplex2;

    const EPSILON: f32 = 0.000001;

    #[test]
    fn test_simplex2() {
        let seed = Seed::new(0);

        assert_approx_eq!( 0.000000, simplex2(&seed, &[0.0f32, 0.0]), EPSILON);
        assert_approx_eq!( 0.341389, simplex2(&seed, &[0.5f32, 0.0]), EPSILON);
        assert_approx_eq!( 0.433783, simplex2(&seed, &[1.0f32, 0.0]), EPSILON);
        assert_approx_eq!( 0.137262, simplex2(&seed, &[1.5f32, 0.0]), EPSILON);
        assert_approx_eq!(-0.309079, simplex2(&seed, &[2.0f32, 0.0]), EPSILON);

        assert_approx_eq!( 0.309427, simplex2(&seed, &[0.0f32, 0.5]), EPSILON);
        assert_approx_eq!( 0.426084, simplex2(&seed, &[0.5f32, 0.5]), EPSILON);
        assert_approx_eq!( 0.166750, simplex2(&seed, &[1.0f32, 0.5]), EPSILON);
        assert_approx_eq!(-0.206059, simplex2(&seed, &[1.5f32, 0.5]), EPSILON);
        assert_approx_eq!(-0.459946, simplex2(&seed, &[2.0f32, 0.5]), EPSILON);

        assert_approx_eq!( 0.332767, simplex2(&seed, &[0.0f32, 1.0]), EPSILON);
        assert_approx_eq!( 0.166750, simplex2(&seed, &[0.5f32, 1.0]), EPSILON);
        assert_approx_eq!(-0.124372, simplex2(&seed, &[1.0f32, 1.0]), EPSILON);
        assert_approx_eq!(-0.348575, simplex2(&seed, &[1.5f32, 1.0]), EPSILON);
        assert_approx_eq!(-0.517394, simplex2(&seed, &[2.0f32, 1.0]), EPSILON);

        assert_approx_eq!( 0.105300, simplex2(&seed, &[0.0f32, 1.5]), EPSILON);
        assert_approx_eq!(-0.196746, simplex2(&seed, &[0.5f32, 1.5]), EPSILON);
        assert_approx_eq!(-0.143812, simplex2(&seed, &[1.0f32, 1.5]), EPSILON);
        assert_approx_eq!(-0.030082, simplex2(&seed, &[1.5f32, 1.5]), EPSILON);
        assert_approx_eq!(-0.344659, simplex2(&seed, &[2.0f32, 1.5]), EPSILON);

        assert_approx_eq!(-0.231452, simplex2(&seed, &[0.0f32, 2.0]), EPSILON);
        assert_approx_eq!(-0.042004, simplex2(&seed, &[0.5f32, 2.0]), EPSILON);
        assert_approx_eq!( 0.255193, simplex2(&seed, &[1.0f32, 2.0]), EPSILON);
        assert_approx_eq!( 0.342329, simplex2(&seed, &[1.5f32, 2.0]), EPSILON);
        assert_approx_eq!( 0.016486, simplex2(&seed, &[2.0f32, 2.0]), EPSILON);
    }
}
