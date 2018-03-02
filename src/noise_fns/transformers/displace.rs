use math::{Point2, Point3, Point4};
use noise_fns::NoiseFn;

/// Noise function that uses multiple source functions to displace each coordinate
/// of the input value before returning the output value from the `source` function.
pub struct Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace> {
    /// Source function that outputs a value
    pub source: Source,

    /// Displacement function that displaces the _x_ coordinate of the input
    /// value.
    pub x_displace: XDisplace,

    /// Displacement function that displaces the _y_ coordinate of the input
    /// value.
    pub y_displace: YDisplace,

    /// Displacement function that displaces the _z_ coordinate of the input
    /// value. Only needed for 3d or higher noise.
    pub z_displace: ZDisplace,

    /// Displacement function that displaces the _u_ coordinate of the input
    /// value. Only needed for 4d or higher noise.
    pub u_displace: UDisplace,
}

impl<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
    Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace> {
    pub fn new(
        source: Source,
        x_displace: XDisplace,
        y_displace: YDisplace,
        z_displace: ZDisplace,
        u_displace: UDisplace,
    ) -> Self {
        Displace {
            source,
            x_displace,
            y_displace,
            z_displace,
            u_displace,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<Source, XDisplace, YDisplace, ZDisplace, UDisplace> NoiseFn<Point2<f64>>
    for Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
    where Source: NoiseFn<Point2<f64>>,
          XDisplace: NoiseFn<Point2<f64>>,
          YDisplace: NoiseFn<Point2<f64>>,
{
    fn get(&self, point: Point2<f64>) -> f64 {
        // Get the output values from the displacement functions and add them to
        // the corresponding coordinate in the input value. Since this is a 2d
        // function, we only need the x_displace and y_displace functions.
        let x = point[0] + self.x_displace.get(point);
        let y = point[1] + self.y_displace.get(point);

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.get([x, y])
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<Source, XDisplace, YDisplace, ZDisplace, UDisplace> NoiseFn<Point3<f64>>
    for Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
    where Source: NoiseFn<Point3<f64>>,
          XDisplace: NoiseFn<Point3<f64>>,
          YDisplace: NoiseFn<Point3<f64>>,
          ZDisplace: NoiseFn<Point3<f64>>,
{
    fn get(&self, point: Point3<f64>) -> f64 {
        // Get the output values from the displacement functions and add them to
        // the corresponding coordinate in the input value. Since this is a 3d
        // function, we only need the x_displace, y_displace, and z_displace
        // functions. Also, panic if there is no z_displace function defined.
        let x = point[0] + self.x_displace.get(point);
        let y = point[1] + self.y_displace.get(point);
        let z = point[2] + self.z_displace.get(point);

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.get([x, y, z])
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<Source, XDisplace, YDisplace, ZDisplace, UDisplace> NoiseFn<Point4<f64>>
    for Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
    where Source: NoiseFn<Point4<f64>>,
          XDisplace: NoiseFn<Point4<f64>>,
          YDisplace: NoiseFn<Point4<f64>>,
          ZDisplace: NoiseFn<Point4<f64>>,
          UDisplace: NoiseFn<Point4<f64>>,
{
    fn get(&self, point: Point4<f64>) -> f64 {
        // Get the output values from the displacement functions and add them to
        // the corresponding coordinate in the input value. Since this is a 4d
        // function, we need all of the displace functions. Panic if there is no z-
        // or u-displace function defined.
        let x = point[0] + self.x_displace.get(point);
        let y = point[1] + self.y_displace.get(point);
        let z = point[2] + self.z_displace.get(point);
        let u = point[3] + self.u_displace.get(point);

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.get([x, y, z, u])
    }
}
