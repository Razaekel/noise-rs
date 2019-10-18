use crate::noise_fns::NoiseFn;
use rayon::prelude::*;

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
        Self {
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
        Self { x_scale, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_y_scale(self, y_scale: f64) -> Self {
        Self { y_scale, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_z_scale(self, z_scale: f64) -> Self {
        Self { z_scale, ..self }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_u_scale(self, u_scale: f64) -> Self {
        Self { u_scale, ..self }
    }

    /// Sets the scaling factor to apply to all coordinates of the input value.
    pub fn set_scale(self, scale: f64) -> Self {
        Self {
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
        Self {
            x_scale,
            y_scale,
            z_scale,
            u_scale,
            ..self
        }
    }
}

impl<Source> NoiseFn<[f64; 2]> for ScalePoint<Source>
where
    Source: NoiseFn<[f64; 2]>,
{
    fn generate(&self, points: &[[f64; 2]]) -> Vec<f64> {
        let x_scale = self.x_scale;
        let y_scale = self.y_scale;

        self.source.generate(
            &points
                .par_iter()
                .map(|point| [point[0] * x_scale, point[1] * y_scale])
                .collect::<Vec<_>>(),
        )
    }
}

impl<Source> NoiseFn<[f64; 3]> for ScalePoint<Source>
where
    Source: NoiseFn<[f64; 3]>,
{
    fn generate(&self, points: &[[f64; 3]]) -> Vec<f64> {
        let x_scale = self.x_scale;
        let y_scale = self.y_scale;
        let z_scale = self.z_scale;

        self.source.generate(
            &points
                .par_iter()
                .map(|point| [point[0] * x_scale, point[1] * y_scale, point[2] * z_scale])
                .collect::<Vec<_>>(),
        )
    }
}

impl<Source> NoiseFn<[f64; 4]> for ScalePoint<Source>
where
    Source: NoiseFn<[f64; 4]>,
{
    fn generate(&self, points: &[[f64; 4]]) -> Vec<f64> {
        let x_scale = self.x_scale;
        let y_scale = self.y_scale;
        let z_scale = self.z_scale;
        let u_scale = self.u_scale;

        self.source.generate(
            &points
                .par_iter()
                .map(|point| {
                    [
                        point[0] * x_scale,
                        point[1] * y_scale,
                        point[2] * z_scale,
                        point[3] * u_scale,
                    ]
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Perlin;
    use super::*;

    #[test]
    fn test_pass_by_ref() {
        let source = Perlin::new();
        let transformed_by_ref = ScalePoint::new(&source)
            .set_x_scale(0.8)
            .set_y_scale(0.1)
            .set_z_scale(0.4)
            .set_u_scale(0.2);
        let mut zero_count = 0;

        for x in 0..10 {
            for y in 0..10 {
                for z in 0..10 {
                    for u in 0..10 {
                        let point: [f64; 4] = [
                            (x as f64) / 10.0,
                            (y as f64) / 10.0,
                            (z as f64) / 10.0,
                            (u as f64) / 10.0,
                        ];
                        let source_value = source.generate(&[point]);
                        let transform_value = transformed_by_ref.generate(&[point]);

                        for i in 0..source_value.len() {
                            if source_value[i] != 0.0 {
                                assert_ne!(source_value[i], transform_value[i]);
                            } else {
                                zero_count += 1;
                            }
                        }
                    }
                }
            }
        }

        assert!(zero_count < 10 * 10 * 10 * 10);
    }
}
