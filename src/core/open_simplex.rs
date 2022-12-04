use crate::{
    gradient,
    math::vectors::{Vector, Vector2, Vector3, Vector4, VectorMap},
    permutationtable::NoiseHasher,
};

pub fn open_simplex_2d<NH>(point: [f64; 2], hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    const STRETCH_CONSTANT: f64 = -0.211_324_865_405_187; //(1/sqrt(2+1)-1)/2;
    const SQUISH_CONSTANT: f64 = 0.366_025_403_784_439; //(sqrt(2+1)-1)/2;
    const NORM_CONSTANT: f64 = 1.0 / 14.0;

    fn surflet(index: usize, point: Vector2<f64>) -> f64 {
        let t = 2.0 - point.magnitude_squared();

        if t > 0.0 {
            let gradient = Vector2::from(gradient::grad2(index));
            t.powi(4) * point.dot(gradient)
        } else {
            0.0
        }
    }

    let point = Vector2::from(point);

    // Place input coordinates onto grid.
    let stretch_offset = point.sum() * STRETCH_CONSTANT;
    let stretched = point.map(|v| v + stretch_offset);

    // Floor to get grid coordinates of rhombus (stretched square) cell origin.
    let stretched_floor = stretched.floor();

    // Skew out to get actual coordinates of rhombus origin. We'll need these later.
    let squish_offset = stretched_floor.sum() * SQUISH_CONSTANT;
    let origin = stretched_floor.map(|v| v + squish_offset);

    // Compute grid coordinates relative to rhombus origin.
    let rel_coords = stretched - stretched_floor;

    // Sum those together to get a value that determines which region we're in.
    let region_sum = rel_coords.sum();

    // Positions relative to origin point (0, 0).
    let rel_pos = point - origin;

    macro_rules! contribute (
        ($x:expr, $y:expr) => {
            {
                let offset = Vector2::new($x, $y);
                let vertex = stretched_floor + offset;
                let index = hasher.hash(&vertex.numcast().unwrap().into_array());
                let dpos = rel_pos - (Vector2::broadcast(SQUISH_CONSTANT) * offset.sum()) - offset;

                surflet(index, dpos)
            }
        }
    );

    let mut value = 0.0;

    // (0, 0) --- (1, 0)
    // |   A     /     |
    // |       /       |
    // |     /     B   |
    // (0, 1) --- (1, 1)

    // Contribution (1, 0)
    value += contribute!(1.0, 0.0);

    // Contribution (0, 1)
    value += contribute!(0.0, 1.0);

    // See the graph for an intuitive explanation; the sum of `x` and `y` is
    // only greater than `1` if we're on Region B.
    if region_sum > 1.0 {
        // Contribution (1, 1)
        value += contribute!(1.0, 1.0);
    } else {
        // Contribution (1, 1)
        value += contribute!(0.0, 0.0);
    }

    value * NORM_CONSTANT
}

pub fn open_simplex_3d<NH>(point: [f64; 3], hasher: &NH) -> f64
where
    NH: NoiseHasher,
{
    const STRETCH_CONSTANT: f64 = -1.0 / 6.0; //(1/Math.sqrt(3+1)-1)/3;
    const SQUISH_CONSTANT: f64 = 1.0 / 3.0; //(Math.sqrt(3+1)-1)/3;
    const NORM_CONSTANT: f64 = 1.0 / 14.0;

    fn surflet(index: usize, point: Vector3<f64>) -> f64 {
        let t = 2.0 - point.magnitude_squared();

        if t > 0.0 {
            let gradient = Vector3::from(gradient::grad3(index));
            t.powi(4) * point.dot(gradient)
        } else {
            0.0
        }
    }

    let point = Vector3::from(point);

    // Place input coordinates on simplectic honeycomb.
    let stretch_offset = point.sum() * STRETCH_CONSTANT;
    let stretched = point.map(|v| v + stretch_offset);

    // Floor to get simplectic honeycomb coordinates of rhombohedron
    // (stretched cube) super-cell origin.
    let stretched_floor = stretched.floor();

    // Skew out to get actual coordinates of rhombohedron origin. We'll need
    // these later.
    let squish_offset = stretched_floor.sum() * SQUISH_CONSTANT;
    let origin = stretched_floor.map(|v| v + squish_offset);

    // Compute simplectic honeycomb coordinates relative to rhombohedral origin.
    let rel_coords = stretched - stretched_floor;

    // Sum those together to get a value that determines which region we're in.
    let region_sum = rel_coords.sum();

    // Positions relative to origin point.
    let rel_pos = point - origin;

    macro_rules! contribute (
        ($x:expr, $y:expr, $z:expr) => {
            {
                let offset = Vector3::new($x, $y, $z);
                let vertex = stretched_floor + offset;
                let index = hasher.hash(&vertex.numcast().unwrap().into_array());
                let dpos = rel_pos - (Vector3::broadcast(SQUISH_CONSTANT) * offset.sum()) - offset;

                surflet(index, dpos)
            }
        }
    );

    let mut value = 0.0;

    if region_sum <= 1.0 {
        // We're inside the tetrahedron (3-Simplex) at (0, 0, 0)

        // Contribution at (0, 0, 0)
        value += contribute!(0.0, 0.0, 0.0);

        // Contribution at (1, 0, 0)
        value += contribute!(1.0, 0.0, 0.0);

        // Contribution at (0, 1, 0)
        value += contribute!(0.0, 1.0, 0.0);

        // Contribution at (0, 0, 1)
        value += contribute!(0.0, 0.0, 1.0);
    } else if region_sum >= 2.0 {
        // We're inside the tetrahedron (3-Simplex) at (1, 1, 1)

        // Contribution at (1, 1, 0)
        value += contribute!(1.0, 1.0, 0.0);

        // Contribution at (1, 0, 1)
        value += contribute!(1.0, 0.0, 1.0);

        // Contribution at (0, 1, 1)
        value += contribute!(0.0, 1.0, 1.0);

        // Contribution at (1, 1, 1)
        value += contribute!(1.0, 1.0, 1.0);
    } else {
        // We're inside the octahedron (Rectified 3-Simplex) inbetween.

        // Contribution at (1, 0, 0)
        value += contribute!(1.0, 0.0, 0.0);

        // Contribution at (0, 1, 0)
        value += contribute!(0.0, 1.0, 0.0);

        // Contribution at (0, 0, 1)
        value += contribute!(0.0, 0.0, 1.0);

        // Contribution at (1, 1, 0)
        value += contribute!(1.0, 1.0, 0.0);

        // Contribution at (1, 0, 1)
        value += contribute!(1.0, 0.0, 1.0);

        // Contribution at (0, 1, 1)
        value += contribute!(0.0, 1.0, 1.0);
    }

    value * NORM_CONSTANT
}

pub fn open_simplex_4d<NH>(point: [f64; 4], hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    const STRETCH_CONSTANT: f64 = -0.138_196_601_125_011; //(Math.sqrt(4+1)-1)/4;
    const SQUISH_CONSTANT: f64 = 0.309_016_994_374_947; //(Math.sqrt(4+1)-1)/4;

    const NORM_CONSTANT: f64 = 1.0 / 6.869_909_007_095_662_5;

    fn surflet(index: usize, point: Vector4<f64>) -> f64 {
        let t = 2.0 - point.magnitude_squared();

        if t > 0.0 {
            let gradient = Vector4::from(gradient::grad4(index));
            t.powi(4) * point.dot(gradient)
        } else {
            0.0
        }
    }

    let point = Vector4::from(point);

    // Place input coordinates on simplectic honeycomb.
    let stretch_offset = point.sum() * STRETCH_CONSTANT;
    let stretched = point.map(|v| v + stretch_offset);

    // Floor to get simplectic honeycomb coordinates of rhombo-hypercube
    // super-cell origin.
    let stretched_floor = stretched.floor();

    // Skew out to get actual coordinates of stretched rhombo-hypercube origin.
    // We'll need these later.
    let squish_offset = stretched_floor.sum() * SQUISH_CONSTANT;
    let origin = stretched_floor.map(|v| v + squish_offset);

    // Compute simplectic honeycomb coordinates relative to rhombo-hypercube
    // origin.
    let rel_coords = stretched - stretched_floor;

    // Sum those together to get a value that determines which region
    // we're in.
    let region_sum = rel_coords.sum();

    // Position relative to origin point.
    let rel_pos = point - origin;

    macro_rules! contribute (
        ($x:expr, $y:expr, $z:expr, $w:expr) => {
            {
                let offset = Vector4::new($x, $y, $z, $w);
                let vertex = stretched_floor + offset;
                let index = hasher.hash(&vertex.numcast().unwrap().into_array());
                let dpos = rel_pos - (Vector4::broadcast(SQUISH_CONSTANT) * offset.sum()) - offset;

                surflet(index, dpos)
            }
        }
    );

    let mut value = 0.0;

    if region_sum <= 1.0 {
        // We're inside the pentachoron (4-Simplex) at (0, 0, 0, 0)

        // Contribution at (0, 0, 0, 0)
        value += contribute!(0.0, 0.0, 0.0, 0.0);

        // Contribution at (1, 0, 0, 0)
        value += contribute!(1.0, 0.0, 0.0, 0.0);

        // Contribution at (0, 1, 0, 0)
        value += contribute!(0.0, 1.0, 0.0, 0.0);

        // Contribution at (0, 0, 1, 0)
        value += contribute!(0.0, 0.0, 1.0, 0.0);

        // Contribution at (0, 0, 0, 1)
        value += contribute!(0.0, 0.0, 0.0, 1.0);
    } else if region_sum >= 3.0 {
        // We're inside the pentachoron (4-Simplex) at (1, 1, 1, 1)

        // Contribution at (1, 1, 1, 0)
        value += contribute!(1.0, 1.0, 1.0, 0.0);

        // Contribution at (1, 1, 0, 1)
        value += contribute!(1.0, 1.0, 0.0, 1.0);

        // Contribution at (1, 0, 1, 1)
        value += contribute!(1.0, 0.0, 1.0, 1.0);

        // Contribution at (0, 1, 1, 1)
        value += contribute!(0.0, 1.0, 1.0, 1.0);

        // Contribution at (1, 1, 1, 1)
        value += contribute!(1.0, 1.0, 1.0, 1.0);
    } else if region_sum <= 2.0 {
        // We're inside the first dispentachoron (Rectified 4-Simplex)

        // Contribution at (1, 0, 0, 0)
        value += contribute!(1.0, 0.0, 0.0, 0.0);

        // Contribution at (0, 1, 0, 0)
        value += contribute!(0.0, 1.0, 0.0, 0.0);

        // Contribution at (0, 0, 1, 0)
        value += contribute!(0.0, 0.0, 1.0, 0.0);

        // Contribution at (0, 0, 0, 1)
        value += contribute!(0.0, 0.0, 0.0, 1.0);

        // Contribution at (1, 1, 0, 0)
        value += contribute!(1.0, 1.0, 0.0, 0.0);

        // Contribution at (1, 0, 1, 0)
        value += contribute!(1.0, 0.0, 1.0, 0.0);

        // Contribution at (1, 0, 0, 1)
        value += contribute!(1.0, 0.0, 0.0, 1.0);

        // Contribution at (0, 1, 1, 0)
        value += contribute!(0.0, 1.0, 1.0, 0.0);

        // Contribution at (0, 1, 0, 1)
        value += contribute!(0.0, 1.0, 0.0, 1.0);

        // Contribution at (0, 0, 1, 1)
        value += contribute!(0.0, 0.0, 1.0, 1.0);
    } else {
        // We're inside the second dispentachoron (Rectified 4-Simplex)

        // Contribution at (1, 1, 1, 0)
        value += contribute!(1.0, 1.0, 1.0, 0.0);

        // Contribution at (1, 1, 0, 1)
        value += contribute!(1.0, 1.0, 0.0, 1.0);

        // Contribution at (1, 0, 1, 1)
        value += contribute!(1.0, 0.0, 1.0, 1.0);

        // Contribution at (0, 1, 1, 1)
        value += contribute!(0.0, 1.0, 1.0, 1.0);

        // Contribution at (1, 1, 0, 0)
        value += contribute!(1.0, 1.0, 0.0, 0.0);

        // Contribution at (1, 0, 1, 0)
        value += contribute!(1.0, 0.0, 1.0, 0.0);

        // Contribution at (1, 0, 0, 1)
        value += contribute!(1.0, 0.0, 0.0, 1.0);

        // Contribution at (0, 1, 1, 0)
        value += contribute!(0.0, 1.0, 1.0, 0.0);

        // Contribution at (0, 1, 0, 1)
        value += contribute!(0.0, 1.0, 0.0, 1.0);

        // Contribution at (0, 0, 1, 1)
        value += contribute!(0.0, 0.0, 1.0, 1.0);
    }

    value * NORM_CONSTANT
}
