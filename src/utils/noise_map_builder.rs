use crate::noise_fns::NoiseFn;
use crate::utils::noise_map::NoiseMap;

pub trait NoiseMapBuilder<'a> {
    fn set_size(self, width: usize, height: usize) -> Self;

    fn set_source_module(self, source_module: &'a dyn NoiseFn<[f64; 3]>) -> Self;

    fn size(&self) -> (usize, usize);

    fn build(&self) -> NoiseMap;
}

pub struct CylinderMapBuilder<'a> {
    angle_bounds: (f64, f64),
    height_bounds: (f64, f64),
    size: (usize, usize),
    source_module: &'a dyn NoiseFn<[f64; 3]>,
}

impl<'a> CylinderMapBuilder<'a> {
    pub fn new(source_module: &'a dyn NoiseFn<[f64; 3]>) -> Self {
        CylinderMapBuilder {
            angle_bounds: (-90.0, 90.0),
            height_bounds: (-1.0, 1.0),
            size: (256, 256),
            source_module,
        }
    }

    pub fn set_angle_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        let angle_bounds = if lower_bound >= upper_bound {
            eprintln!(
                "lower bound {:?} is larger than upper bound {:?}, switching order",
                lower_bound, upper_bound
            );
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
            eprintln!(
                "lower bound {:?} is larger than upper bound {:?}, switching order",
                lower_bound, upper_bound
            );
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

impl<'a> NoiseMapBuilder<'a> for CylinderMapBuilder<'a> {
    fn set_size(self, width: usize, height: usize) -> Self {
        CylinderMapBuilder {
            size: (width, height),
            ..self
        }
    }

    fn set_source_module(self, source_module: &'a dyn NoiseFn<[f64; 3]>) -> Self {
        CylinderMapBuilder {
            source_module,
            ..self
        }
    }

    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn build(&self) -> NoiseMap {
        let (width, height) = self.size;

        let angle_extent = self.angle_bounds.1 - self.angle_bounds.0;
        let height_extent = self.height_bounds.1 - self.height_bounds.0;

        let x_step = angle_extent / width as f64;
        let y_step = height_extent / height as f64;

        let mut points = vec![[0f64; 3]; height * width];

        for y in 0..height {
            let current_height = self.height_bounds.0 + y_step * y as f64;

            for x in 0..width {
                let current_angle = self.angle_bounds.0 + x_step * x as f64;

                let point_x = current_angle.to_radians().cos();
                let point_z = current_angle.to_radians().sin();

                points[(height * y) + x] = [point_x, current_height, point_z];
            }
        }

        let mut result_map = NoiseMap::new(width, height);

        result_map.set_values(self.source_module.generate(&points[..]));

        result_map
    }
}

pub struct PlaneMapBuilder<'a> {
    is_seamless: bool,
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
    size: (usize, usize),
    source_module: &'a dyn NoiseFn<[f64; 3]>,
}

impl<'a> PlaneMapBuilder<'a> {
    pub fn new(source_module: &'a dyn NoiseFn<[f64; 3]>) -> Self {
        PlaneMapBuilder {
            is_seamless: false,
            x_bounds: (-1.0, 1.0),
            y_bounds: (-1.0, 1.0),
            size: (256, 256),
            source_module,
        }
    }

    pub fn set_is_seamless(self, is_seamless: bool) -> Self {
        PlaneMapBuilder {
            is_seamless,
            ..self
        }
    }

    pub fn set_x_bounds(self, lower_x_bound: f64, upper_x_bound: f64) -> Self {
        PlaneMapBuilder {
            x_bounds: (lower_x_bound, upper_x_bound),
            ..self
        }
    }

    pub fn set_y_bounds(self, lower_y_bound: f64, upper_y_bound: f64) -> Self {
        PlaneMapBuilder {
            y_bounds: (lower_y_bound, upper_y_bound),
            ..self
        }
    }

    pub fn x_bounds(&self) -> (f64, f64) {
        self.x_bounds
    }

    pub fn y_bounds(&self) -> (f64, f64) {
        self.y_bounds
    }
}

impl<'a> NoiseMapBuilder<'a> for PlaneMapBuilder<'a> {
    fn set_size(self, width: usize, height: usize) -> Self {
        PlaneMapBuilder {
            size: (width, height),
            ..self
        }
    }

    fn set_source_module(self, source_module: &'a dyn NoiseFn<[f64; 3]>) -> Self {
        PlaneMapBuilder {
            source_module,
            ..self
        }
    }

    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn build(&self) -> NoiseMap {
        let (width, height) = self.size;

        let x_extent = self.x_bounds.1 - self.x_bounds.0;
        let y_extent = self.y_bounds.1 - self.y_bounds.0;

        let x_step = x_extent / width as f64;
        let y_step = y_extent / height as f64;

        let mut points = vec![[0f64; 3]; height * width];

        for y in 0..height {
            let current_y = self.y_bounds.0 + y_step * y as f64;

            for x in 0..width {
                let current_x = self.x_bounds.0 + x_step * x as f64;

                points[(height * y) + x] = [current_x, current_y, 0.5];
            }
        }

        let mut result_map = NoiseMap::new(width, height);

        result_map.set_values(self.source_module.generate(&points[..]));

        result_map
    }
}

pub struct SphereMapBuilder<'a> {
    latitude_bounds: (f64, f64),
    longitude_bounds: (f64, f64),
    size: (usize, usize),
    source_module: &'a dyn NoiseFn<[f64; 3]>,
}

impl<'a> SphereMapBuilder<'a> {
    pub fn new(source_module: &'a dyn NoiseFn<[f64; 3]>) -> Self {
        SphereMapBuilder {
            latitude_bounds: (-1.0, 1.0),
            longitude_bounds: (-1.0, 1.0),
            size: (256, 256),
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

impl<'a> NoiseMapBuilder<'a> for SphereMapBuilder<'a> {
    fn set_size(self, width: usize, height: usize) -> Self {
        SphereMapBuilder {
            size: (width, height),
            ..self
        }
    }

    fn set_source_module(self, source_module: &'a dyn NoiseFn<[f64; 3]>) -> Self {
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

        let lon_extent = self.longitude_bounds.1 - self.longitude_bounds.0;
        let lat_extent = self.latitude_bounds.1 - self.latitude_bounds.0;

        let x_step = lon_extent / width as f64;
        let y_step = lat_extent / height as f64;

        let mut points = vec![[0f64; 3]; height * width];

        for y in 0..height {
            let current_lat = self.latitude_bounds.0 + y_step * y as f64;

            for x in 0..width {
                let current_lon = self.longitude_bounds.0 + x_step * x as f64;

                points[(height * y) + x] = lat_lon_to_xyz(current_lat, current_lon);
            }
        }

        let mut result_map = NoiseMap::new(width, height);

        result_map.set_values(self.source_module.generate(&points[..]));

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
