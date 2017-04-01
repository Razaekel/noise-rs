// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math::interp;
use modules::NoiseModule;
use num_traits::Float;

/// Noise module that maps the output value from the source module onto a
/// terrace-forming curve.
///
/// This noise module maps the output value from the source module onto a
/// terrace-forming curve. The start of the curve has a slode of zero; it's
/// slope then smoothly increases. This curve also contains _control points_
/// which resets the slope to zero at that point, producing a "terracing"
/// effect.
///
/// To add control points to the curve, use the add_control_point method.
///
/// An application must add a minimum of two control points to the curve. If
/// there are less than two control points, the get() method panics. The
/// control points can have any value, although no two control points can
/// have the same value. There is no limit to the number of control points
/// that can be added to the curve.
///
/// The noise module clamps the output value from the source module if that
/// value is less than the value of the lowest control point or greater than
/// the value of the highest control point.
///
/// This noise module is often used to generate terrain features such as the
/// stereotypical desert canyon.
pub struct Terrace<Source, T> {
    /// Outputs a value.
    pub source: Source,

    /// Determines if the terrace-forming curve between all control points is
    /// inverted.
    pub invert_terraces: bool,

    /// Vec that stores the control points.
    control_points: Vec<T>,
}

impl<Source, T> Terrace<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> Terrace<Source, T> {
        Terrace {
            source: source,
            invert_terraces: false,
            control_points: Vec::with_capacity(2),
        }
    }

    /// Adds a control point to the terrace-forming curve.
    ///
    /// Two or more control points define the terrace-forming curve. The start
    /// of this curve has a slope of zero; its slope then smoothly increases.
    /// At the control points, its slope resets to zero.
    ///
    /// It does not matter which order these points are added in.
    pub fn add_control_point(mut self, control_point: T) -> Terrace<Source, T> {
        // check to see if the vector already contains the input point.
        if !self.control_points.iter().any(|&x| x == control_point) {
            // it doesn't, so find the correct position to insert the new
            // control point.
            let insertion_point = self.control_points
                .iter()
                .position(|&x| x >= control_point)
                .unwrap_or(self.control_points.len());

            // add the new control point at the correct position.
            self.control_points.insert(insertion_point, control_point);
        }

        // create new Terrace with updated control_points vector
        Terrace { ..self }
    }

    /// Enables or disables the inversion of the terrain-forming curve between
    /// the control points.
    pub fn invert_terraces(self, invert_terraces: bool) -> Terrace<Source, T> {
        Terrace { invert_terraces: invert_terraces, ..self }
    }
}

impl<Source, T, U> NoiseModule<T> for Terrace<Source, U>
    where Source: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        // confirm that there's at least 2 control points in the vector.
        assert!(self.control_points.len() >= 2);

        // get output value from the source module
        let source_value = self.source.get(point);

        // Find the first element in the control point array that has a input
        // value larger than the output value from the source module
        let index_pos = self.control_points
            .iter()
            .position(|&x| x >= source_value)
            .unwrap_or(self.control_points.len());

        // Find the two nearest control points so that we can map their values
        // onto a quadratic curve.
        let index0 = clamp_index(index_pos as isize - 1, 0, self.control_points.len() - 1);
        let index1 = clamp_index(index_pos as isize, 0, self.control_points.len() - 1);

        // If some control points are missing (which occurs if the value from
        // the source module is greater than the largest input value or less
        // than the smallest input value of the control point array), get the
        // corresponding output value of the nearest control point and exit.
        if index0 == index1 {
            return self.control_points[index1];
        }

        // Compute the alpha value used for cubic interpolation
        let mut input0 = self.control_points[index0];
        let mut input1 = self.control_points[index1];
        let mut alpha = (source_value - input0) / (input1 - input0);

        if self.invert_terraces {
            alpha = U::one() - alpha;
            let temp = input0;
            input0 = input1;
            input1 = temp;
        }

        // Squaring the alpha produces the terrace effect.
        alpha = alpha * alpha;

        // Now perform the cubic interpolation and return.
        interp::linear(input0, input1, alpha)
    }
}

fn clamp_index(index: isize, min: usize, max: usize) -> usize {
    match () {
        _ if index <= min as isize => min,
        _ if index >= max as isize => max,
        _ => index as usize,
    }
}
