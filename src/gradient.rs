#[inline(always)]
#[rustfmt::skip]
pub(crate) fn grad2(index: usize) -> [f64; 2] {
    // Vectors are combinations of -1, 0, and 1
    // Precompute the normalized element
    const DIAG : f64 = core::f64::consts::FRAC_1_SQRT_2;

    match index % 8 {
        0 => [  1.0,   0.0],
        1 => [ -1.0,   0.0],
        2 => [  0.0,   1.0],
        3 => [  0.0,  -1.0],
        4 => [ DIAG,  DIAG],
        5 => [-DIAG,  DIAG],
        6 => [ DIAG, -DIAG],
        7 => [-DIAG, -DIAG],
        _ => panic!("Attempt to access gradient {} of 8", index % 8),
    }
}

#[inline(always)]
#[rustfmt::skip]
pub(crate) fn grad3(index: usize) -> [f64; 3] {
    // Vectors are combinations of -1, 0, and 1
    // Precompute the normalized elements
    const DIAG : f64 = core::f64::consts::FRAC_1_SQRT_2;
    const DIAG2 : f64 = 0.577_350_269_189_625_8;

    match index % 32 {
        // 12 edges repeated twice then 8 corners
        0  | 12 => [  DIAG,   DIAG,    0.0],
        1  | 13 => [ -DIAG,   DIAG,    0.0],
        2  | 14 => [  DIAG,  -DIAG,    0.0],
        3  | 15 => [ -DIAG,  -DIAG,    0.0],
        4  | 16 => [  DIAG,    0.0,   DIAG],
        5  | 17 => [ -DIAG,    0.0,   DIAG],
        6  | 18 => [  DIAG,    0.0,  -DIAG],
        7  | 19 => [ -DIAG,    0.0,  -DIAG],
        8  | 20 => [   0.0,   DIAG,   DIAG],
        9  | 21 => [   0.0,  -DIAG,   DIAG],
        10 | 22 => [   0.0,   DIAG,  -DIAG],
        11 | 23 => [   0.0,  -DIAG,  -DIAG],
        24      => [ DIAG2,  DIAG2,  DIAG2],
        25      => [-DIAG2,  DIAG2,  DIAG2],
        26      => [ DIAG2, -DIAG2,  DIAG2],
        27      => [-DIAG2, -DIAG2,  DIAG2],
        28      => [ DIAG2,  DIAG2, -DIAG2],
        29      => [-DIAG2,  DIAG2, -DIAG2],
        30      => [ DIAG2, -DIAG2, -DIAG2],
        31      => [-DIAG2, -DIAG2, -DIAG2],
        _       => panic!("Attempt to access gradient {} of 32", index % 32),
    }
}

#[inline(always)]
#[rustfmt::skip]
pub(crate) fn grad4(index: usize) -> [f64; 4] {
    // Vectors are combinations of -1, 0, and 1
    // Precompute the normalized elements
    const DIAG : f64 = 0.577_350_269_189_625_8;
    const DIAG2 : f64 = 0.5;

    match index % 64 {
        // 32 edges then 16 corners repeated twice
        0       => [   0.0,   DIAG,   DIAG,   DIAG],
        1       => [   0.0,   DIAG,   DIAG,  -DIAG],
        2       => [   0.0,   DIAG,  -DIAG,   DIAG],
        3       => [   0.0,   DIAG,  -DIAG,  -DIAG],
        4       => [   0.0,  -DIAG,   DIAG,   DIAG],
        5       => [   0.0,  -DIAG,   DIAG,  -DIAG],
        6       => [   0.0,  -DIAG,  -DIAG,   DIAG],
        7       => [   0.0,  -DIAG,  -DIAG,  -DIAG],
        8       => [  DIAG,    0.0,   DIAG,   DIAG],
        9       => [  DIAG,    0.0,   DIAG,  -DIAG],
        10      => [  DIAG,    0.0,  -DIAG,   DIAG],
        11      => [  DIAG,    0.0,  -DIAG,  -DIAG],
        12      => [ -DIAG,    0.0,   DIAG,   DIAG],
        13      => [ -DIAG,    0.0,   DIAG,  -DIAG],
        14      => [ -DIAG,    0.0,  -DIAG,   DIAG],
        15      => [ -DIAG,    0.0,  -DIAG,  -DIAG],
        16      => [  DIAG,   DIAG,    0.0,   DIAG],
        17      => [  DIAG,   DIAG,    0.0,  -DIAG],
        18      => [  DIAG,  -DIAG,    0.0,   DIAG],
        19      => [  DIAG,  -DIAG,    0.0,  -DIAG],
        20      => [ -DIAG,   DIAG,    0.0,   DIAG],
        21      => [ -DIAG,   DIAG,    0.0,  -DIAG],
        22      => [ -DIAG,  -DIAG,    0.0,   DIAG],
        23      => [ -DIAG,  -DIAG,    0.0,  -DIAG],
        24      => [  DIAG,   DIAG,   DIAG,    0.0],
        25      => [  DIAG,   DIAG,  -DIAG,    0.0],
        26      => [  DIAG,  -DIAG,   DIAG,    0.0],
        27      => [  DIAG,  -DIAG,  -DIAG,    0.0],
        28      => [ -DIAG,   DIAG,   DIAG,    0.0],
        29      => [ -DIAG,   DIAG,  -DIAG,    0.0],
        30      => [ -DIAG,  -DIAG,   DIAG,    0.0],
        31      => [ -DIAG,  -DIAG,  -DIAG,    0.0],
        32 | 48 => [ DIAG2,  DIAG2,  DIAG2,  DIAG2],
        33 | 49 => [-DIAG2,  DIAG2,  DIAG2,  DIAG2],
        34 | 50 => [ DIAG2, -DIAG2,  DIAG2,  DIAG2],
        35 | 51 => [-DIAG2, -DIAG2,  DIAG2,  DIAG2],
        36 | 52 => [ DIAG2,  DIAG2, -DIAG2,  DIAG2],
        37 | 53 => [-DIAG2,  DIAG2, -DIAG2,  DIAG2],
        38 | 54 => [ DIAG2,  DIAG2,  DIAG2, -DIAG2],
        39 | 55 => [-DIAG2,  DIAG2,  DIAG2, -DIAG2],
        40 | 56 => [ DIAG2, -DIAG2, -DIAG2,  DIAG2],
        41 | 57 => [-DIAG2, -DIAG2, -DIAG2,  DIAG2],
        42 | 58 => [ DIAG2, -DIAG2,  DIAG2, -DIAG2],
        43 | 59 => [-DIAG2, -DIAG2,  DIAG2, -DIAG2],
        44 | 60 => [ DIAG2,  DIAG2, -DIAG2, -DIAG2],
        45 | 61 => [-DIAG2,  DIAG2, -DIAG2, -DIAG2],
        46 | 62 => [ DIAG2, -DIAG2, -DIAG2, -DIAG2],
        47 | 63 => [-DIAG2, -DIAG2, -DIAG2, -DIAG2],
        _       => panic!("Attempt to access gradient {} of 64", index % 64),
    }
}
