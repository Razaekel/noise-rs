use crate::noise_fns::NoiseFn;
use rayon::prelude::*;

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
    Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
{
    pub fn new(
        source: Source,
        x_displace: XDisplace,
        y_displace: YDisplace,
        z_displace: ZDisplace,
        u_displace: UDisplace,
    ) -> Self {
        Self {
            source,
            x_displace,
            y_displace,
            z_displace,
            u_displace,
        }
    }
}

impl<Source, XDisplace, YDisplace, ZDisplace, UDisplace> NoiseFn<[f64; 2]>
    for Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
where
    Source: NoiseFn<[f64; 2]>,
    XDisplace: NoiseFn<[f64; 2]>,
    YDisplace: NoiseFn<[f64; 2]>,
{
    fn generate(&self, points: &[[f64; 2]]) -> Vec<f64> {
        // Get the output values from the displacement functions and add them to
        // the corresponding coordinate in the input value. Since this is a 2d
        // function, we only need the x_displace and y_displace functions.
        let x_points = self.x_displace.generate(points);
        let y_points = self.y_displace.generate(points);

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.generate(
            &points
                .par_iter()
                .zip(x_points)
                .zip(y_points)
                .map(|((point, x_value), y_value)| [point[0] + x_value, point[1] + y_value])
                .collect::<Vec<_>>(),
        )
    }
}

impl<Source, XDisplace, YDisplace, ZDisplace, UDisplace> NoiseFn<[f64; 3]>
    for Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
where
    Source: NoiseFn<[f64; 3]>,
    XDisplace: NoiseFn<[f64; 3]>,
    YDisplace: NoiseFn<[f64; 3]>,
    ZDisplace: NoiseFn<[f64; 3]>,
{
    fn generate(&self, points: &[[f64; 3]]) -> Vec<f64> {
        // Get the output values from the displacement functions and add them to
        // the corresponding coordinate in the input value. Since this is a 2d
        // function, we only need the x_displace and y_displace functions.
        let x_points = self.x_displace.generate(points);
        let y_points = self.y_displace.generate(points);
        let z_points = self.z_displace.generate(points);

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.generate(
            &points
                .par_iter()
                .zip(x_points)
                .zip(y_points)
                .zip(z_points)
                .map(|(((point, x_value), y_value), z_value)| {
                    [point[0] + x_value, point[1] + y_value, point[2] + z_value]
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl<Source, XDisplace, YDisplace, ZDisplace, UDisplace> NoiseFn<[f64; 4]>
    for Displace<Source, XDisplace, YDisplace, ZDisplace, UDisplace>
where
    Source: NoiseFn<[f64; 4]>,
    XDisplace: NoiseFn<[f64; 4]>,
    YDisplace: NoiseFn<[f64; 4]>,
    ZDisplace: NoiseFn<[f64; 4]>,
    UDisplace: NoiseFn<[f64; 4]>,
{
    fn generate(&self, points: &[[f64; 4]]) -> Vec<f64> {
        // Get the output values from the displacement functions and add them to
        // the corresponding coordinate in the input value. Since this is a 2d
        // function, we only need the x_displace and y_displace functions.
        let x_points = self.x_displace.generate(points);
        let y_points = self.y_displace.generate(points);
        let z_points = self.z_displace.generate(points);
        let u_points = self.u_displace.generate(points);

        // get the output value using the offset input value instead of the
        // original input value.
        self.source.generate(
            &points
                .par_iter()
                .zip(x_points)
                .zip(y_points)
                .zip(z_points)
                .zip(u_points)
                .map(|((((point, x_value), y_value), z_value), u_value)| {
                    [
                        point[0] + x_value,
                        point[1] + y_value,
                        point[2] + z_value,
                        point[3] + u_value,
                    ]
                })
                .collect::<Vec<_>>(),
        )
    }
}
