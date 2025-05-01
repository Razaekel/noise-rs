use crate::{utils::NoiseMap, NoiseFn};

#[cfg(feature = "libm")]
use num_traits::Float;

use super::NoiseMapBuilder;

pub struct SphereMapBuilder<SourceModule>
where
    SourceModule: NoiseFn<f64, 3>,
{
    latitude_bounds: (f64, f64),
    longitude_bounds: (f64, f64),
    size: (usize, usize),
    source_module: SourceModule,
}

impl<SourceModule> SphereMapBuilder<SourceModule>
where
    SourceModule: NoiseFn<f64, 3>,
{
    pub fn new(source_module: SourceModule) -> Self {
        SphereMapBuilder {
            latitude_bounds: (-1.0, 1.0),
            longitude_bounds: (-1.0, 1.0),
            size: (100, 100),
            source_module,
        }
    }

    pub fn set_latitude_bounds(self, min_lat_bound: f64, max_lat_bound: f64) -> Self {
        SphereMapBuilder {
            latitude_bounds: (min_lat_bound, max_lat_bound),
            ..self
        }
    }

    pub fn set_longitude_bounds(self, min_lon_bound: f64, max_lon_bound: f64) -> Self {
        SphereMapBuilder {
            longitude_bounds: (min_lon_bound, max_lon_bound),
            ..self
        }
    }

    pub fn set_bounds(
        self,
        min_lat_bound: f64,
        max_lat_bound: f64,
        min_lon_bound: f64,
        max_lon_bound: f64,
    ) -> Self {
        SphereMapBuilder {
            latitude_bounds: (min_lat_bound, max_lat_bound),
            longitude_bounds: (min_lon_bound, max_lon_bound),
            ..self
        }
    }

    pub fn latitude_bounds(&self) -> (f64, f64) {
        self.latitude_bounds
    }

    pub fn longitude_bounds(&self) -> (f64, f64) {
        self.longitude_bounds
    }
}

impl<SourceModule> NoiseMapBuilder<SourceModule> for SphereMapBuilder<SourceModule>
where
    SourceModule: NoiseFn<f64, 3>,
{
    fn set_size(self, width: usize, height: usize) -> Self {
        SphereMapBuilder {
            size: (width, height),
            ..self
        }
    }

    fn set_source_module(self, source_module: SourceModule) -> Self {
        SphereMapBuilder {
            source_module,
            ..self
        }
    }

    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn build(&self) -> NoiseMap {
        let (width, height) = self.size;

        let mut result_map = NoiseMap::new(width, height);

        let lon_extent = self.longitude_bounds.1 - self.longitude_bounds.0;
        let lat_extent = self.latitude_bounds.1 - self.latitude_bounds.0;

        let x_step = lon_extent / width as f64;
        let y_step = lat_extent / height as f64;

        for y in 0..height {
            let current_lat = self.latitude_bounds.0 + y_step * y as f64;

            for x in 0..width {
                let current_lon = self.longitude_bounds.0 + x_step * x as f64;

                let point = lat_lon_to_xyz(current_lat, current_lon);

                result_map[(x, y)] = self.source_module.get(point);
            }
        }

        result_map
    }
}

fn lat_lon_to_xyz(lat: f64, lon: f64) -> [f64; 3] {
    let r = lat.to_radians().cos();
    let x = r * lon.to_radians().cos();
    let y = lat.to_radians().sin();
    let z = r * lon.to_radians().sin();

    [x, y, z]
}
