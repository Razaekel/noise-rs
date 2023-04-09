use crate::{core::checkerboard::*, noise_fns::NoiseFn};

/// Noise function that outputs a checkerboard pattern.
///
/// This noise function can take one input, size, and outputs 2<sup>size</sup>-sized
/// blocks of alternating values. The values of these blocks alternate between
/// -1.0 and 1.0.
///
/// This noise function is not very useful by itself, but it can be used for
/// debugging purposes.
#[derive(Clone, Copy, Debug)]
pub struct Checkerboard {
    // Controls the size of the block in 2^(size).
    size: usize,
}

impl Checkerboard {
    const DEFAULT_SIZE: usize = 0;

    /// Controls the size of the block in 2^(size) units.
    pub fn new(size: usize) -> Self {
        Self { size: 1 << size }
    }

    pub fn set_size(self, size: usize) -> Self {
        Self { size: 1 << size }
    }

    pub fn size(self) -> usize {
        self.size
    }
}

impl Default for Checkerboard {
    fn default() -> Self {
        Self {
            size: 1 << Checkerboard::DEFAULT_SIZE,
        }
    }
}

impl NoiseFn<f64, 2> for Checkerboard {
    fn get(&self, point: [f64; 2]) -> f64 {
        checkerboard_2d(point.into(), self.size as f64)
    }
}

impl NoiseFn<f64, 3> for Checkerboard {
    fn get(&self, point: [f64; 3]) -> f64 {
        checkerboard_3d(point.into(), self.size as f64)
    }
}

impl NoiseFn<f64, 4> for Checkerboard {
    fn get(&self, point: [f64; 4]) -> f64 {
        checkerboard_4d(point.into(), self.size as f64)
    }
}
