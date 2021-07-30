use num_traits::Float;

#[rustfmt::skip]
pub(crate) fn grad2<F>(index: usize) -> [F; 2]
where
    F: Float,
{
    // Vectors are combinations of -1, 0, and 1
    // Precompute the normalized element
    let diag = F::from(core::f64::consts::FRAC_1_SQRT_2).unwrap();

    match index % 8 {
        0 => [  F::one(),   F::zero()],
        1 => [ -F::one(),   F::zero()],
        2 => [  F::zero(),   F::one()],
        3 => [  F::zero(),  -F::one()],
        4 => [ diag,  diag],
        5 => [-diag,  diag],
        6 => [ diag, -diag],
        7 => [-diag, -diag],
        _ => panic!("Attempt to access gradient {} of 8", index % 8),
    }
}

#[rustfmt::skip]
pub(crate) fn grad3<F>(index: usize) -> [F; 3]
where
    F: Float,
{
    // Vectors are combinations of -1, 0, and 1
    // Precompute the normalized elements
    let diag = F::from(core::f64::consts::FRAC_1_SQRT_2).unwrap();
    let diag2 = F::from(0.577_350_269_189_625_8).unwrap();

    match index % 32 {
        // 12 edges repeated twice then 8 corners
        0  | 12 => [      diag,      diag, F::zero()],
        1  | 13 => [     -diag,      diag, F::zero()],
        2  | 14 => [      diag,     -diag, F::zero()],
        3  | 15 => [     -diag,     -diag, F::zero()],
        4  | 16 => [      diag, F::zero(),      diag],
        5  | 17 => [     -diag, F::zero(),      diag],
        6  | 18 => [      diag, F::zero(),     -diag],
        7  | 19 => [     -diag, F::zero(),     -diag],
        8  | 20 => [ F::zero(),      diag,      diag],
        9  | 21 => [ F::zero(),     -diag,      diag],
        10 | 22 => [ F::zero(),      diag,     -diag],
        11 | 23 => [ F::zero(),     -diag,     -diag],
        24      => [ diag2,  diag2,  diag2],
        25      => [-diag2,  diag2,  diag2],
        26      => [ diag2, -diag2,  diag2],
        27      => [-diag2, -diag2,  diag2],
        28      => [ diag2,  diag2, -diag2],
        29      => [-diag2,  diag2, -diag2],
        30      => [ diag2, -diag2, -diag2],
        31      => [-diag2, -diag2, -diag2],
        _       => panic!("Attempt to access gradient {} of 32", index % 32),
    }
}

#[rustfmt::skip]
pub(crate) fn grad4<F>(index: usize) -> [F; 4]
where
    F: Float,
{
    // Vectors are combinations of -1, 0, and 1
    // Precompute the normalized elements
    let diag = F::from(0.577_350_269_189_625_8).unwrap();
    let diag2 = F::from(0.5).unwrap();

    match index % 64 {
        // 32 edges then 16 corners repeated twice
        0       => [   F::zero(),         diag,         diag,         diag],
        1       => [   F::zero(),         diag,         diag,        -diag],
        2       => [   F::zero(),         diag,        -diag,         diag],
        3       => [   F::zero(),         diag,        -diag,        -diag],
        4       => [   F::zero(),        -diag,         diag,         diag],
        5       => [   F::zero(),        -diag,         diag,        -diag],
        6       => [   F::zero(),        -diag,        -diag,         diag],
        7       => [   F::zero(),        -diag,        -diag,        -diag],
        8       => [        diag,    F::zero(),         diag,         diag],
        9       => [        diag,    F::zero(),         diag,        -diag],
        10      => [        diag,    F::zero(),        -diag,         diag],
        11      => [        diag,    F::zero(),        -diag,        -diag],
        12      => [       -diag,    F::zero(),         diag,         diag],
        13      => [       -diag,    F::zero(),         diag,        -diag],
        14      => [       -diag,    F::zero(),        -diag,         diag],
        15      => [       -diag,    F::zero(),        -diag,        -diag],
        16      => [        diag,         diag,    F::zero(),         diag],
        17      => [        diag,         diag,    F::zero(),        -diag],
        18      => [        diag,        -diag,    F::zero(),         diag],
        19      => [        diag,        -diag,    F::zero(),        -diag],
        20      => [       -diag,         diag,    F::zero(),         diag],
        21      => [       -diag,         diag,    F::zero(),        -diag],
        22      => [       -diag,        -diag,    F::zero(),         diag],
        23      => [       -diag,        -diag,    F::zero(),        -diag],
        24      => [        diag,         diag,         diag,    F::zero()],
        25      => [        diag,         diag,        -diag,    F::zero()],
        26      => [        diag,        -diag,         diag,    F::zero()],
        27      => [        diag,        -diag,        -diag,    F::zero()],
        28      => [       -diag,         diag,         diag,    F::zero()],
        29      => [       -diag,         diag,        -diag,    F::zero()],
        30      => [       -diag,        -diag,         diag,    F::zero()],
        31      => [       -diag,        -diag,        -diag,    F::zero()],
        32 | 48 => [ diag2,  diag2,  diag2,  diag2],
        33 | 49 => [-diag2,  diag2,  diag2,  diag2],
        34 | 50 => [ diag2, -diag2,  diag2,  diag2],
        35 | 51 => [-diag2, -diag2,  diag2,  diag2],
        36 | 52 => [ diag2,  diag2, -diag2,  diag2],
        37 | 53 => [-diag2,  diag2, -diag2,  diag2],
        38 | 54 => [ diag2,  diag2,  diag2, -diag2],
        39 | 55 => [-diag2,  diag2,  diag2, -diag2],
        40 | 56 => [ diag2, -diag2, -diag2,  diag2],
        41 | 57 => [-diag2, -diag2, -diag2,  diag2],
        42 | 58 => [ diag2, -diag2,  diag2, -diag2],
        43 | 59 => [-diag2, -diag2,  diag2, -diag2],
        44 | 60 => [ diag2,  diag2, -diag2, -diag2],
        45 | 61 => [-diag2,  diag2, -diag2, -diag2],
        46 | 62 => [ diag2, -diag2, -diag2, -diag2],
        47 | 63 => [-diag2, -diag2, -diag2, -diag2],
        _       => panic!("Attempt to access gradient {} of 64", index % 64),
    }
}
