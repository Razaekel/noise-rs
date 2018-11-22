use math::{Point2, Point3, Point4};
use noise_fns::NoiseFn;

/// Noise function that scales the coordinates of the input value before
/// returning the output value from the source function.
///
/// The get() method multiplies the coordinates of the input value with a
/// scaling factor before returning the output value from the source function.
pub struct ScalePoint<Source> {
    /// Source function that outputs a value
    pub source: Source,

    /// Scaling factor applied to the _x_ coordinate of the input value. The
    /// default scaling factor is set to 1.0.
    pub x_scale: f64,

    /// Scaling factor applied to the _y_ coordinate of the input value. The
    /// default scaling factor is set to 1.0.
    pub y_scale: f64,

    /// Scaling factor applied to the _z_ coordinate of the input value. The
    /// default scaling factor is set to 1.0.
    pub z_scale: f64,

    /// Scaling factor applied to the _u_ coordinate of the input value. The
    /// default scaling factor is set to 1.0.
    pub u_scale: f64,
}

impl<Source> ScalePoint<Source> {
    pub fn new(source: Source) -> Self {
        ScalePoint {
            source,
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
            u_scale: 1.0,
        }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_x_scale(self, x_scale: f64) -> Self {
        ScalePoint { x_scale, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_y_scale(self, y_scale: f64) -> Self {
        ScalePoint { y_scale, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_z_scale(self, z_scale: f64) -> Self {
        ScalePoint { z_scale, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_u_scale(self, u_scale: f64) -> Self {
        ScalePoint { u_scale, ..self }
    }

    /// Sets the scaling factor to apply to all coordinates of the input value.
    pub fn set_scale(self, scale: f64) -> Self {
        ScalePoint {
            x_scale: scale,
            y_scale: scale,
            z_scale: scale,
            u_scale: scale,
            ..self
        }
    }

    /// Sets the individual scaling factors to apply to each coordinate of the
    /// input value.
    pub fn set_all_scales(self, x_scale: f64, y_scale: f64, z_scale: f64, u_scale: f64) -> Self {
        ScalePoint {
            x_scale,
            y_scale,
            z_scale,
            u_scale,
            ..self
        }
    }
}

impl<Source> NoiseFn<Point2<f64>> for ScalePoint<Source>
where
    Source: NoiseFn<Point2<f64>>,
{
    fn get(&self, point: Point2<f64>) -> f64 {
        self.source
            .get([point[0] * self.x_scale, point[1] * self.y_scale])
    }
}

impl<Source> NoiseFn<Point3<f64>> for ScalePoint<Source>
where
    Source: NoiseFn<Point3<f64>>,
{
    fn get(&self, point: Point3<f64>) -> f64 {
        self.source.get([
            point[0] * self.x_scale,
            point[1] * self.y_scale,
            point[2] * self.z_scale,
        ])
    }
}

impl<Source> NoiseFn<Point4<f64>> for ScalePoint<Source>
where
    Source: NoiseFn<Point4<f64>>,
{
    fn get(&self, point: Point4<f64>) -> f64 {
        self.source.get([
            point[0] * self.x_scale,
            point[1] * self.y_scale,
            point[2] * self.z_scale,
            point[3] * self.u_scale,
        ])
    }
}
