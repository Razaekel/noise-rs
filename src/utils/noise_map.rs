#[cfg(feature = "image")]
use crate::math;
#[cfg(feature = "image")]
use image;
#[cfg(feature = "image")]
use std::{self, path::Path};

const RASTER_MAX_WIDTH: u16 = 32_767;
const RASTER_MAX_HEIGHT: u16 = 32_767;

pub struct NoiseMap {
    size: (usize, usize),
    border_value: f64,
    map: Vec<f64>,
}

impl NoiseMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self::initialize().set_size(width, height)
    }

    pub fn set_size(self, width: usize, height: usize) -> Self {
        // Check for invalid width or height.
        assert!(width < RASTER_MAX_WIDTH as usize);
        assert!(height < RASTER_MAX_HEIGHT as usize);

        if width == 0 || height == 0 {
            // An empty noise map was specified. Return a new blank, empty map.
            Self::initialize()
        } else {
            // New noise map size specified. Allocate a new Vec unless the current Vec is large
            // enough.
            let map_size = width * height;
            if self.map.capacity() < map_size {
                // New size is too big for the current Vec. Create a new Vec with a large enough
                // capacity now so we're not reallocating when filling the map.
                Self {
                    map: vec![0.0; map_size],
                    size: (width, height),
                    ..self
                }
            } else {
                // Vec capacity is already big enough, so leave it alone and just change the set size.
                Self {
                    size: (width, height),
                    ..self
                }
            }
        }
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn set_border_value(self, border_value: f64) -> Self {
        Self {
            border_value,
            ..self
        }
    }

    pub fn border_value(&self) -> f64 {
        self.border_value
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: f64) {
        let (width, height) = self.size;

        if x < width && y < height {
            self.map[x + y * width] = value;
        } else {
            eprintln!("input point out of bounds")
        }
    }

    pub fn get_value(&self, x: usize, y: usize) -> f64 {
        let (width, height) = self.size;

        if x < width && y < height {
            self.map[x + y * width]
        } else {
            self.border_value
        }
    }

    #[cfg(feature = "image")]
    pub fn write_to_file(&self, filename: &str) {
        // Create the output directory for the images, if it doesn't already exist
        let target_dir = Path::new("example_images/");

        if !target_dir.exists() {
            std::fs::create_dir(target_dir).expect("failed to create example_images directory");
        }

        //concatenate the directory to the filename string
        let directory: String = "example_images/".to_owned();
        let file_path = directory + filename;

        // collect the values from f64 into u8 in a separate vec
        let (width, height) = self.size;
        let mut pixels: Vec<u8> = Vec::with_capacity(width * height);

        for i in &self.map {
            pixels.push((math::clamp(i * 0.5 + 0.5, 0.0, 1.0) * 255.0) as u8);
        }

        let _ = image::save_buffer(
            &Path::new(&file_path),
            &*pixels,
            self.size.0 as u32,
            self.size.1 as u32,
            image::ColorType::Gray(8),
        );

        println!("\nFinished generating {}", filename);
    }

    fn initialize() -> Self {
        Self {
            size: (0, 0),
            border_value: 0.0,
            map: Vec::new(),
        }
    }
}

impl Default for NoiseMap {
    fn default() -> Self {
        Self::initialize()
    }
}
