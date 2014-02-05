pub use self::perlin::Perlin;

pub mod perlin;

/// A source of noise values.
pub trait Source {
    /// For the given coordinate, return the value
    /// The value is between -1 and 1
    fn get<F:Float>(&self, x: F, y: F, z: F) -> F;
}
