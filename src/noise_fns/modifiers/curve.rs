use crate::{math::interpolate, noise_fns::NoiseFn};
use alloc::vec::Vec;
use core::marker::PhantomData;

/// Noise function that maps the output value from the source function onto an
/// arbitrary function curve.
///
/// This noise function maps the output value from the source function onto an
/// application-defined curve. The curve is defined by a number of _control
/// points_; each control point has an _input value_ that maps to an _output
/// value_.
///
/// To add control points to the curve, use the `add_control_point` method.
///
/// Since the curve is a cubic spline, an application must have a minimum of
/// four control points to the curve. If there is less than four control
/// points, the get() method panics. Each control point can have any input
/// and output value, although no two control points can have the same input.
pub struct Curve<T, Source, const DIM: usize>
where
    Source: NoiseFn<T, DIM>,
{
    /// Outputs a value.
    pub source: Source,

    /// Vec that stores the control points.
    control_points: Vec<ControlPoint<f64>>,

    phantom: PhantomData<T>,
}

struct ControlPoint<T> {
    input: T,
    output: T,
}

impl<T, Source, const DIM: usize> Curve<T, Source, DIM>
where
    Source: NoiseFn<T, DIM>,
{
    pub fn new(source: Source) -> Self {
        Self {
            source,
            control_points: Vec::with_capacity(4),
            phantom: PhantomData,
        }
    }

    pub fn add_control_point(mut self, input_value: f64, output_value: f64) -> Self {
        // check to see if the vector already contains the input point.
        if !self
            .control_points
            .iter()
            .any(|x| (x.input - input_value).abs() < f64::EPSILON)
        {
            // it doesn't, so find the correct position to insert the new
            // control point.
            let insertion_point = self
                .control_points
                .iter()
                .position(|x| x.input >= input_value)
                .unwrap_or(self.control_points.len());

            // add the new control point at the correct position.
            self.control_points.insert(
                insertion_point,
                ControlPoint {
                    input: input_value,
                    output: output_value,
                },
            );
        }

        self
    }
}

impl<T, Source, const DIM: usize> NoiseFn<T, DIM> for Curve<T, Source, DIM>
where
    Source: NoiseFn<T, DIM>,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        // confirm that there's at least 4 control points in the vector.
        assert!(self.control_points.len() >= 4);

        // get output value from the source function
        let source_value = self.source.get(point);

        // Find the first element in the control point array that has a input
        // value larger than the output value from the source function
        let index_pos = self
            .control_points
            .iter()
            .position(|x| x.input > source_value)
            .unwrap_or(self.control_points.len());

        // if index_pos < 2 {
        //     println!(
        //         "index_pos in curve was less than 2! source value was {}",
        //         source_value
        //     );
        // }

        // ensure that the index is at least 2 and less than control_points.len()
        let index_pos = index_pos.clamp(2, self.control_points.len());

        // Find the four nearest control points so that we can perform cubic
        // interpolation.
        let index0 = (index_pos - 2).clamp(0, self.control_points.len() - 1);
        let index1 = (index_pos - 1).clamp(0, self.control_points.len() - 1);
        let index2 = index_pos.clamp(0, self.control_points.len() - 1);
        let index3 = (index_pos + 1).clamp(0, self.control_points.len() - 1);

        // If some control points are missing (which occurs if the value from
        // the source function is greater than the largest input value or less
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
        interpolate::cubic(
            self.control_points[index0].output,
            self.control_points[index1].output,
            self.control_points[index2].output,
            self.control_points[index3].output,
            alpha,
        )
    }
}
