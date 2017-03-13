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

/// Noise module that maps the output value from the source module onto an
/// arbitrary function curve.
///
/// This noise module maps the output value from the source module onto an
/// application-defined curve. The curve is defined by a number of _control
/// points_; each control point has an _input value_ that maps to an _output
/// value_.
///
/// To add control points to the curve, use the add_control_point method.
///
/// Since the curve is a cubic spline, an application must have a minumum of
/// four control points to the curve. If there is less than four control
/// points, the get() method panics. Each control point can have any input
/// and output value, although no two control points can have the same input
pub struct Curve<Source, T> {
    /// Outputs a value.
    pub source: Source,

    /// Vec that stores the control points.
    control_points: Vec<ControlPoint<T>>,
}

struct ControlPoint<T> {
    input: T,
    output: T,
}

impl<Source, T> Curve<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> Curve<Source, T> {
        Curve {
            source: source,
            control_points: Vec::with_capacity(4),
        }
    }

    pub fn add_control_point(mut self, input_value: T, output_value: T) -> Curve<Source, T> {
        // check to see if the vector already contains the input point.
        if !self.control_points.iter().any(|x| x.input == input_value) {
            // it doesn't, so find the correct position to insert the new
            // control point.
            let insertion_point = self.control_points
                .iter()
                .position(|x| x.input >= input_value)
                .unwrap_or(self.control_points.len());

            // add the new control point at the correct position.
            self.control_points.insert(insertion_point,
                                       ControlPoint {
                                           input: input_value,
                                           output: output_value,
                                       });
        }

        // create new Curve with updated control_points vector
        Curve { ..self }
    }
}

impl<Source, T, U> NoiseModule<T> for Curve<Source, U>
    where Source: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        // confirm that there's at least 4 control points in the vector.
        assert!(self.control_points.len() >= 4);

        // get output value from the source module
        let source_value = self.source.get(point);

        // Find the first element in the control point array that has a input
        // value larger than the output value from the source module
        let index_pos = self.control_points
            .iter()
            .position(|x| x.input >= source_value)
            .unwrap_or(self.control_points.len());

        // Find the four nearest control points so that we can perform cubic
        // interpolation.
        let index0 = clamp_index(index_pos as isize - 2, 0, self.control_points.len() - 1);
        let index1 = clamp_index(index_pos as isize - 1, 0, self.control_points.len() - 1);
        let index2 = clamp_index(index_pos as isize, 0, self.control_points.len() - 1);
        let index3 = clamp_index(index_pos as isize + 1, 0, self.control_points.len() - 1);

        // If some control points are missing (which occurs if the value from
        // the source module is greater than the largest input value or less
        // than the smallest input value of the control point array), get the
        // corresponding output value of the nearest control point and exit.
        if index1 == index2 {
            return self.control_points[index1].output;
        }

        // Compute the alpha value used for cubic interpolation
        let input0 = self.control_points[index1].input;
        let input1 = self.control_points[index2].input;
        let alpha = (source_value - input0) / (input1 - input0);

        // Now perform the cubic interpolation and return.
        interp::cubic(self.control_points[index0].output,
                      self.control_points[index1].output,
                      self.control_points[index2].output,
                      self.control_points[index3].output,
                      alpha)
    }
}

fn clamp_index(index: isize, min: usize, max: usize) -> usize {
    match () {
        _ if index <= min as isize => min,
        _ if index >= max as isize => max,
        _ => index as usize,
    }
}
