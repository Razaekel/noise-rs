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
    /// Controls the size of the block in 2^(size).
    pub size: usize,

    // Dummy field to prevent struct initialization except through the
    // new() constructor.
    _dummy: (),
}

impl Checkerboard {
    const DEFAULT_SIZE: usize = 0;

    pub fn new() -> Self {
        Self {
            size: 1 << Self::DEFAULT_SIZE,
            _dummy: (),
        }
    }

    pub fn set_size(self, size: usize) -> Self {
        Self {
            size: 1 << size,
            ..self
        }
    }
}

impl Default for Checkerboard {
    fn default() -> Self {
        Self::new()
    }
}

// These impl's should be made generic over Point, but there is no higher Point
// type. Keep the code the same anyway.
impl NoiseFn<[f64; 2]> for Checkerboard {
    fn get(&self, point: [f64; 2]) -> f64 {
        calculate_checkerboard(&point, self.size)
    }
}

impl NoiseFn<[f64; 3]> for Checkerboard {
    fn get(&self, point: [f64; 3]) -> f64 {
        calculate_checkerboard(&point, self.size)
    }
}

impl NoiseFn<[f64; 4]> for Checkerboard {
    fn get(&self, point: [f64; 4]) -> f64 {
        calculate_checkerboard(&point, self.size)
    }
}

fn calculate_checkerboard(point: &[f64], size: usize) -> f64 {
    let result = point
        .iter()
        .map(|&a| a.floor() as usize)
        .fold(0, |a, b| (a & size) ^ (b & size));

    if result > 0 {
        -1.0
    } else {
        1.0
    }
}
