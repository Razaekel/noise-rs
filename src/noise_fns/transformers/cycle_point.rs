use crate::noise_fns::NoiseFn;

fn lerp(t: f64, start: f64, end: f64) -> f64 {
    t.mul_add(end, (-t).mul_add(start, start))
}

/// Noise function that loops the input value over a the modulus of a domain
/// before returning the output value from the source function.
///
/// The looped Noise will be continuous across the start and the end of the domain,
/// and to accomplish this, the source function will be sampled twice and blended.
///
/// The domain of each dimension always starts at 0. If a different start is required,
/// use TranslatePoint.
///
/// CyclePoint may be useful for repeated tiled noise, or noise around a polar domain.
pub struct CyclePoint<Source> {
    /// Outputs a value.
    pub source: Source,

    pub x_period: f64,
    pub y_period: f64,
    pub z_period: f64,
    pub u_period: f64,
}

impl<Source> CyclePoint<Source> {
    pub fn new(source: Source) -> Self {
        Self {
            source,
            x_period: 1.,
            y_period: 1.,
            z_period: 1.,
            u_period: 1.,
        }
    }

    pub fn set_x_period(self, x_period: f64) -> Self {
        Self { x_period, ..self }
    }

    pub fn set_y_period(self, y_period: f64) -> Self {
        Self { y_period, ..self }
    }

    pub fn set_z_period(self, z_period: f64) -> Self {
        Self { z_period, ..self }
    }

    pub fn set_u_period(self, u_period: f64) -> Self {
        Self { u_period, ..self }
    }
}

impl<Source> NoiseFn<f64, 1> for CyclePoint<Source>
where
    Source: NoiseFn<f64, 1>,
{
    fn get(&self, point: [f64; 1]) -> f64 {
        let x0 = if point[0] >= 0. {
            point[0] % self.x_period
        } else {
            (point[0] % self.x_period) + self.x_period
        };
        let x1 = self.x_period - x0;
        let xt = x0 / self.x_period;
        lerp(xt, self.source.get([x0]), self.source.get([x1]))
    }
}

impl<Source> NoiseFn<f64, 2> for CyclePoint<Source>
where
    Source: NoiseFn<f64, 2>,
{
    fn get(&self, point: [f64; 2]) -> f64 {
        let x0 = if point[0] >= 0. {
            point[0] % self.x_period
        } else {
            (point[0] % self.x_period) + self.x_period
        };
        let x1 = self.x_period - x0;
        let xt = x0 / self.x_period;

        let y0 = if point[1] >= 0. {
            point[1] % self.y_period
        } else {
            (point[1] % self.y_period) + self.y_period
        };
        let y1 = self.y_period - y0;
        let yt = y0 / self.y_period;

        lerp(
            yt,
            lerp(xt, self.source.get([x0, y0]), self.source.get([x1, y0])),
            lerp(xt, self.source.get([x0, y1]), self.source.get([x1, y1])),
        )
    }
}

impl<Source> NoiseFn<f64, 3> for CyclePoint<Source>
where
    Source: NoiseFn<f64, 3>,
{
    fn get(&self, point: [f64; 3]) -> f64 {
        let x0 = if point[0] >= 0. {
            point[0] % self.x_period
        } else {
            (point[0] % self.x_period) + self.x_period
        };
        let x1 = self.x_period - x0;
        let xt = x0 / self.x_period;

        let y0 = if point[1] >= 0. {
            point[1] % self.y_period
        } else {
            (point[1] % self.y_period) + self.y_period
        };
        let y1 = self.y_period - y0;
        let yt = y0 / self.y_period;

        let z0 = if point[2] >= 0. {
            point[2] % self.z_period
        } else {
            (point[2] % self.z_period) + self.z_period
        };
        let z1 = self.z_period - z0;
        let zt = z0 / self.z_period;

        lerp(
            zt,
            lerp(
                yt,
                lerp(
                    xt,
                    self.source.get([x0, y0, z0]),
                    self.source.get([x1, y0, z0]),
                ),
                lerp(
                    xt,
                    self.source.get([x0, y1, z0]),
                    self.source.get([x1, y1, z0]),
                ),
            ),
            lerp(
                yt,
                lerp(
                    xt,
                    self.source.get([x0, y0, z1]),
                    self.source.get([x1, y0, z1]),
                ),
                lerp(
                    xt,
                    self.source.get([x0, y1, z1]),
                    self.source.get([x1, y1, z1]),
                ),
            ),
        )
    }
}

impl<Source> NoiseFn<f64, 4> for CyclePoint<Source>
where
    Source: NoiseFn<f64, 4>,
{
    fn get(&self, point: [f64; 4]) -> f64 {
        let x0 = if point[0] >= 0. {
            point[0] % self.x_period
        } else {
            (point[0] % self.x_period) + self.x_period
        };
        let x1 = self.x_period - x0;
        let xt = x0 / self.x_period;

        let y0 = if point[1] >= 0. {
            point[1] % self.y_period
        } else {
            (point[1] % self.y_period) + self.y_period
        };
        let y1 = self.y_period - y0;
        let yt = y0 / self.y_period;

        let z0 = if point[2] >= 0. {
            point[2] % self.z_period
        } else {
            (point[2] % self.z_period) + self.z_period
        };
        let z1 = self.z_period - z0;
        let zt = z0 / self.z_period;

        let u0 = if point[3] >= 0. {
            point[3] % self.u_period
        } else {
            (point[3] % self.u_period) + self.u_period
        };
        let u1 = self.u_period - u0;
        let ut = u0 / self.u_period;

        lerp(
            ut,
            lerp(
                zt,
                lerp(
                    yt,
                    lerp(
                        xt,
                        self.source.get([x0, y0, z0, u0]),
                        self.source.get([x1, y0, z0, u0]),
                    ),
                    lerp(
                        xt,
                        self.source.get([x0, y1, z0, u0]),
                        self.source.get([x1, y1, z0, u0]),
                    ),
                ),
                lerp(
                    yt,
                    lerp(
                        xt,
                        self.source.get([x0, y0, z1, u0]),
                        self.source.get([x1, y0, z1, u0]),
                    ),
                    lerp(
                        xt,
                        self.source.get([x0, y1, z1, u0]),
                        self.source.get([x1, y1, z1, u0]),
                    ),
                ),
            ),
            lerp(
                zt,
                lerp(
                    yt,
                    lerp(
                        xt,
                        self.source.get([x0, y0, z0, u1]),
                        self.source.get([x1, y0, z0, u1]),
                    ),
                    lerp(
                        xt,
                        self.source.get([x0, y1, z0, u1]),
                        self.source.get([x1, y1, z0, u1]),
                    ),
                ),
                lerp(
                    yt,
                    lerp(
                        xt,
                        self.source.get([x0, y0, z1, u1]),
                        self.source.get([x1, y0, z1, u1]),
                    ),
                    lerp(
                        xt,
                        self.source.get([x0, y1, z1, u1]),
                        self.source.get([x1, y1, z1, u1]),
                    ),
                ),
            ),
        )
    }
}
