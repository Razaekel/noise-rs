use crate::noise_fns::NoiseFn;

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

impl<const N: usize> NoiseFn<f64, N> for Checkerboard {
    fn get(&self, point: [f64; N]) -> f64 {
        let result = point
            .iter()
            .map(|&a| a.floor() as isize)
            .reduce(|a, b| (a & self.size as isize) ^ (b & self.size as isize))
            .unwrap();

        if result > 0 {
            -1.0
        } else {
            1.0
        }
    }
}
