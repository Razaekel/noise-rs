use crate::{gradient, math::vectors::*, permutationtable::NoiseHasher};

const TO_REAL_CONSTANT_2D: f64 = -0.211_324_865_405_187; // (1 / sqrt(2 + 1) - 1) / 2
const TO_SIMPLEX_CONSTANT_2D: f64 = 0.366_025_403_784_439; // (sqrt(2 + 1) - 1) / 2
const TO_SIMPLEX_CONSTANT_3D: f64 = -2.0 / 3.0;

// Determined using the Mathematica code listed in the super_simplex example and find_maximum_super_simplex.nb
const NORM_CONSTANT_2D: f64 = 1.0 / 0.054_282_952_886_616_23;
const NORM_CONSTANT_3D: f64 = 1.0 / 0.086_766_400_165_536_9;

// Points taken into account for 2D:
//             (0, -1)
//                |    \
//                |      \
//                |        \
// (-1, 0) --- ( 0,  0) --- ( 1,  0)
//        \       |    \       |    \
//          \     |      \     |      \
//            \   |        \   |        \
//             ( 0,  1) --- ( 1,  1) --- ( 2,  1)
//                     \       |
//                       \     |
//                         \   |
//                          ( 1,  2)
#[rustfmt::skip]
const LATTICE_LOOKUP_2D: [([i8; 2], [f64; 2]); 4 * 8] =
    [([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([-1, 0], [0.788_675_134_594_813_f64, -0.211_324_865_405_187_f64]),
     ([0, -1], [-0.211_324_865_405_187_f64, 0.788_675_134_594_813_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([0, 1], [0.211_324_865_405_187_f64, -0.788_675_134_594_813_f64]),
     ([1, 0], [-0.788_675_134_594_813_f64, 0.211_324_865_405_187_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([1, 0], [-0.788_675_134_594_813_f64, 0.211_324_865_405_187_f64]),
     ([0, -1], [-0.211_324_865_405_187_f64, 0.788_675_134_594_813_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([2, 1], [-1.366_025_403_784_439_f64, -0.366_025_403_784_439_04_f64]),
     ([1, 0], [-0.788_675_134_594_813_f64, 0.211_324_865_405_187_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([-1, 0], [0.788_675_134_594_813_f64, -0.211_324_865_405_187_f64]),
     ([0, 1], [0.211_324_865_405_187_f64, -0.788_675_134_594_813_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([0, 1], [0.211_324_865_405_187_f64, -0.788_675_134_594_813_f64]),
     ([1, 2], [-0.366_025_403_784_439_04_f64, -1.366_025_403_784_439_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([1, 0], [-0.788_675_134_594_813_f64, 0.211_324_865_405_187_f64]),
     ([0, 1], [0.211_324_865_405_187_f64, -0.788_675_134_594_813_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([2, 1], [-1.366_025_403_784_439_f64, -0.366_025_403_784_439_04_f64]),
     ([1, 2], [-0.366_025_403_784_439_04_f64, -1.366_025_403_784_439_f64])];

#[rustfmt::skip]
const LATTICE_LOOKUP_3D: [[i8; 3]; 4 * 16] =
    [[0, 0, 0],[1, 0, 0],[0, 1, 0],[0, 0, 1],
     [1, 1, 1],[1, 0, 0],[0, 1, 0],[0, 0, 1],
     [0, 0, 0],[0, 1, 1],[0, 1, 0],[0, 0, 1],
     [1, 1, 1],[0, 1, 1],[0, 1, 0],[0, 0, 1],
     [0, 0, 0],[1, 0, 0],[1, 0, 1],[0, 0, 1],
     [1, 1, 1],[1, 0, 0],[1, 0, 1],[0, 0, 1],
     [0, 0, 0],[0, 1, 1],[1, 0, 1],[0, 0, 1],
     [1, 1, 1],[0, 1, 1],[1, 0, 1],[0, 0, 1],
     [0, 0, 0],[1, 0, 0],[0, 1, 0],[1, 1, 0],
     [1, 1, 1],[1, 0, 0],[0, 1, 0],[1, 1, 0],
     [0, 0, 0],[0, 1, 1],[0, 1, 0],[1, 1, 0],
     [1, 1, 1],[0, 1, 1],[0, 1, 0],[1, 1, 0],
     [0, 0, 0],[1, 0, 0],[1, 0, 1],[1, 1, 0],
     [1, 1, 1],[1, 0, 0],[1, 0, 1],[1, 1, 0],
     [0, 0, 0],[0, 1, 1],[1, 0, 1],[1, 1, 0],
     [1, 1, 1],[0, 1, 1],[1, 0, 1],[1, 1, 0]];

pub fn super_simplex_2d<NH>(point: [f64; 2], hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    let point = Vector2::from(point);

    // Transform point from real space to simplex space
    let to_simplex_offset = point.sum() * TO_SIMPLEX_CONSTANT_2D;
    let simplex_point = point.map(|v| v + to_simplex_offset);

    // Get base point of simplex and barycentric coordinates in simplex space
    let simplex_base_point = simplex_point.floor();
    let simplex_base_point_i = simplex_base_point.numcast().unwrap();
    let simplex_rel_coords = simplex_point - simplex_base_point;

    // Create index to lookup table from barycentric coordinates
    let region_sum = simplex_rel_coords.sum().floor();
    let index = ((region_sum >= 1.0) as usize) << 2
        | ((simplex_rel_coords.x - simplex_rel_coords.y * 0.5 + 1.0 - region_sum * 0.5 >= 1.0)
            as usize)
            << 3
        | ((simplex_rel_coords.y - simplex_rel_coords.x * 0.5 + 1.0 - region_sum * 0.5 >= 1.0)
            as usize)
            << 4;

    // Transform barycentric coordinates to real space
    let to_real_offset = simplex_rel_coords.sum() * TO_REAL_CONSTANT_2D;
    let real_rel_coords = simplex_rel_coords.map(|v| v + to_real_offset);

    let mut value = 0.0;

    for lattice_lookup in &LATTICE_LOOKUP_2D[index..index + 4] {
        let dpos = real_rel_coords + Vector2::from(lattice_lookup.1).numcast().unwrap();
        let attn = (2.0 / 3.0) - dpos.magnitude_squared();
        if attn > 0.0 {
            let lattice_point =
                simplex_base_point_i + Vector2::from(lattice_lookup.0).numcast().unwrap();
            let gradient = Vector2::from(gradient::grad2(hasher.hash(&lattice_point.into_array())));
            value += attn.powi(4) * gradient.dot(dpos);
        }
    }

    value * NORM_CONSTANT_2D
}

pub fn super_simplex_3d<NH>(point: [f64; 3], hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    let point = Vector3::from(point);

    // Transform point from real space to simplex space
    let to_simplex_offset = point.sum() * TO_SIMPLEX_CONSTANT_3D;
    let simplex_point = point.map(|v| -(v + to_simplex_offset));
    let second_simplex_point = simplex_point.map(|v| v + 512.5);

    // Get base point of simplex and barycentric coordinates in simplex space
    let simplex_base_point = simplex_point.floor();
    let simplex_base_point_i = simplex_base_point.numcast().unwrap();
    let simplex_rel_coords = simplex_point - simplex_base_point;
    let second_simplex_base_point = second_simplex_point.floor();
    let second_simplex_base_point_i = second_simplex_base_point.numcast().unwrap();
    let second_simplex_rel_coords = second_simplex_point - second_simplex_base_point;

    // Create indices to lookup table from barycentric coordinates
    let index = ((simplex_rel_coords.x + simplex_rel_coords.y + simplex_rel_coords.z >= 1.5)
        as usize)
        << 2
        | ((-simplex_rel_coords.x + simplex_rel_coords.y + simplex_rel_coords.z >= 0.5) as usize)
            << 3
        | ((simplex_rel_coords.x - simplex_rel_coords.y + simplex_rel_coords.z >= 0.5) as usize)
            << 4
        | ((simplex_rel_coords.x + simplex_rel_coords.y - simplex_rel_coords.z >= 0.5) as usize)
            << 5;
    let second_index = ((second_simplex_rel_coords.x
        + second_simplex_rel_coords.y
        + second_simplex_rel_coords.z
        >= 1.5) as usize)
        << 2
        | ((-second_simplex_rel_coords.x
            + second_simplex_rel_coords.y
            + second_simplex_rel_coords.z
            >= 0.5) as usize)
            << 3
        | ((second_simplex_rel_coords.x - second_simplex_rel_coords.y + second_simplex_rel_coords.z
            >= 0.5) as usize)
            << 4
        | ((second_simplex_rel_coords.x + second_simplex_rel_coords.y - second_simplex_rel_coords.z
            >= 0.5) as usize)
            << 5;

    let mut value = 0.0;

    // Sum contributions from first lattice
    for &lattice_lookup in &LATTICE_LOOKUP_3D[index..index + 4] {
        let dpos = simplex_rel_coords - Vector3::from(lattice_lookup).numcast().unwrap();
        let attn = 0.75 - dpos.magnitude_squared();
        if attn > 0.0 {
            let lattice_point =
                simplex_base_point_i + Vector3::from(lattice_lookup).numcast().unwrap();
            let gradient = Vector3::from(gradient::grad3(hasher.hash(&lattice_point.into_array())));
            value += attn.powi(4) * gradient.dot(dpos);
        }
    }

    // Sum contributions from second lattice
    for &lattice_lookup in &LATTICE_LOOKUP_3D[second_index..second_index + 4] {
        let dpos = second_simplex_rel_coords - Vector3::from(lattice_lookup).numcast().unwrap();
        let attn = 0.75 - dpos.magnitude_squared();
        if attn > 0.0 {
            let lattice_point =
                second_simplex_base_point_i + Vector3::from(lattice_lookup).numcast().unwrap();
            let gradient = Vector3::from(gradient::grad3(hasher.hash(&lattice_point.into_array())));
            value += attn.powi(4) * gradient.dot(dpos);
        }
    }

    value * NORM_CONSTANT_3D
}
