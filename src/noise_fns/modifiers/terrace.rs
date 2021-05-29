use crate::{math::interpolate, noise_fns::NoiseFn, MultiFractal, Seedable};

/// Noise function that maps the output value from the source function onto a
/// terrace-forming curve.
///
/// This noise function maps the output value from the source function onto a
/// terrace-forming curve. The start of the curve has a slode of zero; it's
/// slope then smoothly increases. This curve also contains _control points_
/// which resets the slope to zero at that point, producing a "terracing"
/// effect.
///
/// To add control points to the curve, use the `add_control_point` method.
///
/// An application must add a minimum of two control points to the curve. If
/// there are less than two control points, the get() method panics. The
/// control points can have any value, although no two control points can
/// have the same value. There is no limit to the number of control points
/// that can be added to the curve.
///
/// The noise function clamps the output value from the source function if that
/// value is less than the value of the lowest control point or greater than
/// the value of the highest control point.
///
/// This noise function is often used to generate terrain features such as the
/// stereotypical desert canyon.
pub struct Terrace<T, const DIM: usize>
where
    T: NoiseFn<DIM>,
{
    /// Outputs a value.
    pub source: T,

    /// Determines if the terrace-forming curve between all control points is
    /// inverted.
    pub invert_terraces: bool,

    /// Vec that stores the control points.
    control_points: Vec<f64>,
}

impl<T, const DIM: usize> Terrace<T, DIM>
where
    T: NoiseFn<DIM>,
{
    pub fn new(source: T) -> Self {
        Terrace {
            source,
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
    pub fn add_control_point(mut self, control_point: f64) -> Self {
        // check to see if the vector already contains the input point.
        if !self
            .control_points
            .iter()
            .any(|&x| (x - control_point).abs() < std::f64::EPSILON)
        {
            // it doesn't, so find the correct position to insert the new
            // control point.
            let insertion_point = self
                .control_points
                .iter()
                .position(|&x| x >= control_point)
                .unwrap_or_else(|| self.control_points.len());

            // add the new control point at the correct position.
            self.control_points.insert(insertion_point, control_point);
        }

        // create new Terrace with updated control_points vector
        Terrace { ..self }
    }

    /// Enables or disables the inversion of the terrain-forming curve between
    /// the control points.
    pub fn invert_terraces(self, invert_terraces: bool) -> Self {
        Terrace {
            invert_terraces,
            ..self
        }
    }
}

impl<T, const DIM: usize> NoiseFn<DIM> for Terrace<T, DIM>
where
    T: NoiseFn<DIM>,
{
    fn get(&self, point: [f64; DIM]) -> f64 {
        // confirm that there's at least 2 control points in the vector.
        assert!(self.control_points.len() >= 2);

        // get output value from the source function
        let source_value = self.source.get(point);

        // Find the first element in the control point array that has a input
        // value larger than the output value from the source function
        let index_pos = self
            .control_points
            .iter()
            .position(|&x| x >= source_value)
            .unwrap_or_else(|| self.control_points.len());

        // Find the two nearest control points so that we can map their values
        // onto a quadratic curve.
        let index0 = clamp_index(index_pos as isize - 1, 0, self.control_points.len() - 1);
        let index1 = clamp_index(index_pos as isize, 0, self.control_points.len() - 1);

        // If some control points are missing (which occurs if the value from
        // the source function is greater than the largest input value or less
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
            alpha = 1.0 - alpha;
            std::mem::swap(&mut input0, &mut input1);
        }

        // Squaring the alpha produces the terrace effect.
        alpha *= alpha;

        // Now perform the cubic interpolation and return.
        interpolate::linear(input0, input1, alpha)
    }
}

fn clamp_index(index: isize, min: usize, max: usize) -> usize {
    index.clamp(min as isize, max as isize) as usize
}

impl<T, const DIM: usize> Seedable for Terrace<T, DIM>
where
    T: NoiseFn<DIM> + Seedable,
{
    fn new(seed: u32) -> Self {
        Self {
            source: T::new(seed),
            invert_terraces: false,
            control_points: Vec::with_capacity(2),
        }
    }

    fn set_seed(self, seed: u32) -> Self {
        Self {
            source: self.source.set_seed(seed),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.source.seed()
    }
}

impl<T, const DIM: usize> MultiFractal for Terrace<T, DIM>
where
    T: NoiseFn<DIM> + MultiFractal,
{
    fn set_octaves(self, octaves: usize) -> Self {
        Self {
            source: self.source.set_octaves(octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Self {
            source: self.source.set_frequency(frequency),
            ..self
        }
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self {
            source: self.source.set_lacunarity(lacunarity),
            ..self
        }
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Self {
            source: self.source.set_persistence(persistence),
            ..self
        }
    }
}
