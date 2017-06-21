// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use math::{Point2, Point3, Point4};
use modules::NoiseModule;
use num_traits::Float;

/// Noise Module that uses multiple source modules to displace each coordinate
/// of the input value before returning the output value from the `source` module.
pub struct Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace> {
    /// Source Module that outputs a value
    pub source: Source,

    /// Displacement module that displaces the _x_ coordinate of the input
    /// value.
    pub x_displace: XDisplace,

    /// Displacement module that displaces the _y_ coordinate of the input
    /// value.
    pub y_displace: YDisplace,

    /// Displacement module that displaces the _z_ coordinate of the input
    /// value. Only needed for 3d or higher noise.
    pub z_displace: ZDisplace,

    /// Displacement module that displaces the _u_ coordinate of the input
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
    ) -> Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace> {
        Displace {
            source: source,
            x_displace: x_displace,
            y_displace: y_displace,
            z_displace: z_displace,
            u_displace: u_displace,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<Source, XDisplace, YDisplace, ZDisplace, UDisplace, T> NoiseModule<Point2<T>, T>
    for Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
    where Source: NoiseModule<Point2<T>, T>,
          XDisplace: NoiseModule<Point2<T>, T>,
          YDisplace: NoiseModule<Point2<T>, T>,
          T: Float,
{
    fn get(&self, point: Point2<T>) -> T {
        // Get the output values from the displacement modules and add them to
        // the corresponding coordinate in the input value. Since this is a 2d
        // module, we only need the x_displace and y_displace modules.
        let x = point[0] + self.x_displace.get(point);
        let y = point[1] + self.y_displace.get(point);

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.get([x, y])
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<Source, XDisplace, YDisplace, ZDisplace, UDisplace, T> NoiseModule<Point3<T>, T>
    for Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
    where Source: NoiseModule<Point3<T>, T>,
          XDisplace: NoiseModule<Point3<T>, T>,
          YDisplace: NoiseModule<Point3<T>, T>,
          ZDisplace: NoiseModule<Point3<T>, T>,
          T: Float,
{
    fn get(&self, point: Point3<T>) -> T {
        // Get the output values from the displacement modules and add them to
        // the corresponding coordinate in the input value. Since this is a 3d
        // module, we only need the x_displace, y_displace, and z_displace
        // modules. Also, panic if there is no z_displace module defined.
        let x = point[0] + self.x_displace.get(point);
        let y = point[1] + self.y_displace.get(point);
        let z = point[2] + self.z_displace.get(point);

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.get([x, y, z])
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<Source, XDisplace, YDisplace, ZDisplace, UDisplace, T> NoiseModule<Point4<T>, T>
    for Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
    where Source: NoiseModule<Point4<T>, T>,
          XDisplace: NoiseModule<Point4<T>, T>,
          YDisplace: NoiseModule<Point4<T>, T>,
          ZDisplace: NoiseModule<Point4<T>, T>,
          UDisplace: NoiseModule<Point4<T>, T>,
          T: Float,
{
    fn get(&self, point: Point4<T>) -> T {
        // Get the output values from the displacement modules and add them to
        // the corresponding coordinate in the input value. Since this is a 4d
        // module, we need all of the displace modules. Panic if there is no z-
        // or u-displace module defined.
        let x = point[0] + self.x_displace.get(point);
        let y = point[1] + self.y_displace.get(point);
        let z = point[2] + self.z_displace.get(point);
        let u = point[3] + self.u_displace.get(point);

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.get([x, y, z, u])
    }
}
