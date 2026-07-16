#[cfg(not(feature = "std"))]
use num_traits::Float as _;

use crate::{utils::NoiseMap, NoiseFn};

use super::NoiseMapBuilder;

pub struct CylinderMapBuilder<SourceModule>
where
    SourceModule: NoiseFn<f64, 3>,
{
    angle_bounds: (f64, f64),
    height_bounds: (f64, f64),
    size: (usize, usize),
    source_module: SourceModule,
}

impl<SourceModule> CylinderMapBuilder<SourceModule>
where
    SourceModule: NoiseFn<f64, 3>,
{
    pub fn new(source_module: SourceModule) -> Self {
        CylinderMapBuilder {
            angle_bounds: (-90.0, 90.0),
            height_bounds: (-1.0, 1.0),
            size: (100, 100),
            source_module,
        }
    }

    pub fn set_angle_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        let angle_bounds = if lower_bound >= upper_bound {
            (upper_bound, lower_bound)
        } else {
            (lower_bound, upper_bound)
        };

        CylinderMapBuilder {
            angle_bounds,
            ..self
        }
    }

    pub fn set_height_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        let height_bounds = if lower_bound >= upper_bound {
            (upper_bound, lower_bound)
        } else {
            (lower_bound, upper_bound)
        };

        CylinderMapBuilder {
            height_bounds,
            ..self
        }
    }

    pub fn angle_bounds(&self) -> (f64, f64) {
        self.angle_bounds
    }

    pub fn height_bounds(&self) -> (f64, f64) {
        self.height_bounds
    }
}

impl<SourceModule> NoiseMapBuilder<SourceModule> for CylinderMapBuilder<SourceModule>
where
    SourceModule: NoiseFn<f64, 3>,
{
    fn set_size(self, width: usize, height: usize) -> Self {
        CylinderMapBuilder {
            size: (width, height),
            ..self
        }
    }

    fn set_source_module(self, source_module: SourceModule) -> Self {
        CylinderMapBuilder {
            source_module,
            ..self
        }
    }

    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn build(&self) -> NoiseMap {
        let mut result_map = NoiseMap::new(self.size.0, self.size.1);

        let (width, height) = self.size;

        let angle_extent = self.angle_bounds.1 - self.angle_bounds.0;
        let height_extent = self.height_bounds.1 - self.height_bounds.0;

        let x_step = angle_extent / width as f64;
        let y_step = height_extent / height as f64;

        for y in 0..height {
            let current_height = self.height_bounds.0 + y_step * y as f64;

            for x in 0..width {
                let current_angle = self.angle_bounds.0 + x_step * x as f64;

                let point_x = current_angle.to_radians().cos();
                let point_z = current_angle.to_radians().sin();

                let value = self.source_module.get([point_x, current_height, point_z]);

                result_map[(x, y)] = value;
            }
        }

        result_map
    }
}
