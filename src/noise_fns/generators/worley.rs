use crate::{
    core::worley::*,
    math::vectors::*,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::PermutationTable,
};
use alloc::rc::Rc;

/// Noise function that outputs Worley noise.
#[derive(Clone)]
pub struct Worley {
    /// Specifies the distance function to use when calculating the boundaries of
    /// the cell.
    pub distance_function: Rc<DistanceFunction>,

    /// Signifies whether the distance from the borders of the cell should be returned, or the
    /// value for the cell.
    pub return_type: ReturnType,

    /// Frequency of the seed points.
    pub frequency: f64,

    seed: u32,
    perm_table: PermutationTable,
}

type DistanceFunction = dyn Fn(&[f64], &[f64]) -> f64;

impl Worley {
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;

    pub fn new(seed: u32) -> Self {
        Self {
            perm_table: PermutationTable::new(seed),
            seed,
            distance_function: Rc::new(distance_functions::euclidean),
            return_type: ReturnType::Value,
            frequency: Self::DEFAULT_FREQUENCY,
        }
    }

    /// Sets the distance function used by the Worley cells.
    pub fn set_distance_function<F>(self, function: F) -> Self
    where
        F: Fn(&[f64], &[f64]) -> f64 + 'static,
    {
        Self {
            distance_function: Rc::new(function),
            ..self
        }
    }

    /// Enables or disables applying the distance from the nearest seed point
    /// to the output value.
    pub fn set_return_type(self, return_type: ReturnType) -> Self {
        Self {
            return_type,
            ..self
        }
    }

    /// Sets the frequency of the seed points.
    pub fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }
}

impl Default for Worley {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Seedable for Worley {
    /// Sets the seed value used by the Worley cells.
    fn set_seed(self, seed: u32) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Self {
            perm_table: PermutationTable::new(seed),
            seed,
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

impl NoiseFn<f64, 2> for Worley {
    fn get(&self, point: [f64; 2]) -> f64 {
        worley_2d(
            &self.perm_table,
            &*self.distance_function,
            self.return_type,
            (Vector2::from(point) * self.frequency).into_array(),
        )
    }
}

impl NoiseFn<f64, 3> for Worley {
    fn get(&self, point: [f64; 3]) -> f64 {
        worley_3d(
            &self.perm_table,
            &*self.distance_function,
            self.return_type,
            (Vector3::from(point) * self.frequency).into_array(),
        )
    }
}

#[allow(clippy::cognitive_complexity)]
impl NoiseFn<f64, 4> for Worley {
    fn get(&self, point: [f64; 4]) -> f64 {
        worley_4d(
            &self.perm_table,
            &*self.distance_function,
            self.return_type,
            (Vector4::from(point) * self.frequency).into_array(),
        )
    }
}
