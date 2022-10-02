//! An example of using Super Simplex noise

extern crate noise;

use noise::{utils::*, Seedable, SuperSimplex};

fn main() {
    let mut lookup_2d: [([i8; 2], [f64; 2]); 8 * 4] = [([0; 2], [0.0; 2]); 8 * 4];
    let mut lookup_3d: [[i8; 3]; 16 * 4] = [[0; 3]; 16 * 4];

    let skew_constant = -0.211324865405187;
    for i in 0..8 {
        let (i1, j1, i2, j2);
        if i & 1 == 0 {
            if i & 2 == 0 {
                i1 = -1;
                j1 = 0;
            } else {
                i1 = 1;
                j1 = 0;
            }
            if i & 4 == 0 {
                i2 = 0;
                j2 = -1;
            } else {
                i2 = 0;
                j2 = 1;
            }
        } else {
            if i & 2 != 0 {
                i1 = 2;
                j1 = 1;
            } else {
                i1 = 0;
                j1 = 1;
            }
            if i & 4 != 0 {
                i2 = 1;
                j2 = 2;
            } else {
                i2 = 1;
                j2 = 0;
            }
        }
        lookup_2d[i * 4] = ([0, 0], [0.0, 0.0]);
        let skew_factor = -1.0 - 2.0 * skew_constant;
        lookup_2d[i * 4 + 1] = ([1, 1], [skew_factor, skew_factor]);
        let skew_factor = (i1 as f64 + j1 as f64) * skew_constant;
        lookup_2d[i * 4 + 2] = (
            [i1, j1],
            [-i1 as f64 - skew_factor, -j1 as f64 - skew_factor],
        );
        let skew_factor = (i2 as f64 + j2 as f64) * skew_constant;
        lookup_2d[i * 4 + 3] = (
            [i2, j2],
            [-i2 as f64 - skew_factor, -j2 as f64 - skew_factor],
        );
    }

    print!("lookup_2d = [");
    for x in &lookup_2d {
        print!(
            "([{}, {}], [{}f64, {}f64]),",
            x.0[0], x.0[1], x.1[0], x.1[1]
        );
    }
    println!("\x08]");

    for i in 0..16 {
        let (i1, j1, k1, i2, j2, k2, i3, j3, k3, i4, j4, k4);
        if i & 1 != 0 {
            i1 = 1;
            j1 = 1;
            k1 = 1;
        } else {
            i1 = 0;
            j1 = 0;
            k1 = 0;
        }
        if i & 2 != 0 {
            i2 = 0;
            j2 = 1;
            k2 = 1;
        } else {
            i2 = 1;
            j2 = 0;
            k2 = 0;
        }
        if i & 4 != 0 {
            j3 = 0;
            i3 = 1;
            k3 = 1;
        } else {
            j3 = 1;
            i3 = 0;
            k3 = 0;
        }
        if i & 8 != 0 {
            k4 = 0;
            i4 = 1;
            j4 = 1;
        } else {
            k4 = 1;
            i4 = 0;
            j4 = 0;
        }
        lookup_3d[i * 4] = [i1, j1, k1];
        lookup_3d[i * 4 + 1] = [i2, j2, k2];
        lookup_3d[i * 4 + 2] = [i3, j3, k3];
        lookup_3d[i * 4 + 3] = [i4, j4, k4];
    }

    print!("lookup_3d = [");
    for x in lookup_3d.iter() {
        print!("[{}, {}, {}],", x[0], x[1], x[2]);
    }
    println!("\x08]");

    // Calculation of maximum value:
    // x => real_rel_coords[0], y => real_rel_coords[1]
    // a-h, components of gradient vectors for 4 closest points
    // One contribution: (a*x + b*y) * (2/3 - x^2 - y^2)^4
    // Limit per contribution: 0 <= x^2 + y^2 < 2/3
    // skew = ((1 / sqrt(2 + 1) - 1) / 2)
    // (a*x + b*y) * (2/3 - x^2 - y^2)^4 + (c*(x - 1 - 2 * skew) + d*(y - 1 - 2 * skew)) * (2/3 - (x - 1 - 2 * skew)^2 - (y - 1 - 2 * skew)^2)^4 + (e*(x - skew) + f*(y - 1 - skew)) * (2/3 - (x - skew)^2 - (y - 1 - skew)^2)^4 + (g*(x - 1 - skew) + h*(y - skew)) * (2/3 - (x - 1 - skew)^2 - (y - skew)^2)^4
    // 0 <= x^2 + y^2 < 2/3 && 0 <= (x - 1 - 2 * skew)^2 + (y - 1 - 2 * skew)^2 < 2/3 && 0 <= (x - skew)^2 + (y - 1 - skew)^2 < 2/3 && 0 <= (x - 1 - skew)^2 + (y - skew)^2 < 2/3
    // a^2 + b^2 == 1 && c^2 + d^2 == 1 && e^2 + f^2 == 1 && g^2 + h^2 == 1

    // Note: Maximum value is dependent on gradients. In the example below the gradients were [0,1] at [0,0], [-1,0] at [1,1], and [1/sqrt(2),-1/sqrt(2)] at [0,1] (on the simplex grid)
    // The maximum possible value is achieved when the dot product of the delta position to the gradient is 1.0. As such, the gradients used below were picked because they produced the maximum possible dot product when sampled at the centroid of the simplex.
    // Mathematica code for finding maximum of 2D Super Simplex noise:
    // Clear["Global`*"];
    // skew = (1/Sqrt[2 + 1] - 1)/2
    // eq[a_, b_, x_, y_] = (a*x + b*y)*(2/3 - x^2 - y^2)^4
    // eq5[x_, y_] = eq[0, 1, x, y] + eq[-1, 0, x - 1 - 2*skew, y - 1 - 2*skew] + eq[1/Sqrt[2], -1/Sqrt[2], x - skew, y - 1 - skew]
    // F[{x_, y_}] = eq5[x, y];
    // Fx[x_, y_] = D[eq5[x, y], x];
    // Fy[x_, y_] = D[eq5[x, y], y];
    // Fxx[x_, y_] = D[D[eq5[x, y], x], x];
    // Fyy[x_, y_] = D[D[eq5[x, y], y], y];
    // Fxy[x_, y_] = D[D[eq5[x, y], x], y];
    // X0 = {1/3 + skew, 2/3 + skew};
    // P0 = N[X0];
    // gradF[{x_, y_}] = {Fx[x, y], Fy[x, y]};
    // H[{x_, y_}] = {{Fxx[x, y], Fxy[x, y]}, {Fxy[x, y], Fyy[x, y]}};
    // Print["f[", PaddedForm[P0, {13, 12}], "]=",
    //       PaddedForm[F[P0], {13, 12}]]
    //     For[i = 1, i <= 10, i++, P0 = P0 - gradF[P0].Inverse[H[P0]];
    //         Print["f[", PaddedForm[P0, {21, 20}], "]=",
    //               PaddedForm[F[P0], {21, 20}]]]
    //     P0;
    // {xout, yout} = P0;
    // eq5[xout, yout]

    // The computation for the maximum in 3D is shown below.
    // As it turns out, the maximum is at [1/4,1/4,1/4], so iteration is unnecessary in this case.
    // The most likely cause for this is that the gradient vectors lined up much better in 3D than they did in 2D, and so the "center" of one of the simplices also turned out to be the maxima.
    // All gradient vectors were chosen pointing towards the center of the simplex cube.
    // Clear["Global`*"];
    // skew3d = 2/3;
    // norm = 1/Sqrt[2];
    // norm2 = 1/Sqrt[3];
    // eq3d[a_, b_, c_, x_, y_,
    //    z_] = (a*x + b*y + c*z)*(3/4 - x^2 - y^2 - z^2)^4;
    // (*first lattice: [0,0,0], [1,0,0], [0,1,0], [0,0,1]*)
    // (*second lattice: [1,1,1], [0,1,1], [1,0,1], [1,1,0]*)
    // eq3dsp[x_, y_, z_] =
    //   eq3d[norm2, norm2, norm2, x, y, z] +
    //    eq3d[-norm2, norm2, norm2, x - 1, y, z] +
    //    eq3d[norm2, -norm2, norm2, x, y - 1, z] +
    //    eq3d[norm2, norm2, -norm2, x, y, z - 1] +
    //    eq3d[-norm2, -norm2, -norm2, x + 1/2 - 1, y + 1/2 - 1, z + 1/2 - 1] +
    //    eq3d[norm2, -norm2, -norm2, x + 1/2, y + 1/2 - 1, z + 1/2 - 1] +
    //    eq3d[-norm2, norm2, -norm2, x + 1/2 - 1, y + 1/2, z + 1/2 - 1] +
    //    eq3d[-norm2, -norm2, norm2, x + 1/2 - 1, y + 1/2 - 1, z + 1/2];
    // F[{x_, y_, z_}] = eq3dsp[x, y, z];
    // Fx[x_, y_, z_] = D[eq3dsp[x, y, z], x];
    // Fy[x_, y_, z_] = D[eq3dsp[x, y, z], y];
    // Fz[x_, y_, z_] = D[eq3dsp[x, y, z], z];
    // Fxx[x_, y_, z_] = D[D[eq3dsp[x, y, z], x], x];
    // Fyy[x_, y_, z_] = D[D[eq3dsp[x, y, z], y], y];
    // Fzz[x_, y_, z_] = D[D[eq3dsp[x, y, z], z], z];
    // Fxy[x_, y_, z_] = D[D[eq3dsp[x, y, z], x], y];
    // Fxz[x_, y_, z_] = D[D[eq3dsp[x, y, z], x], z];
    // Fyz[x_, y_, z_] = D[D[eq3dsp[x, y, z], y], z];
    // X0 = {1/4, 1/4, 1/4};
    // P0 = N[X0];
    // gradF[{x_, y_, z_}] = {Fx[x, y, z], Fy[x, y, z], Fz[x, y, z]};
    // H[{x_, y_, z_}] = {{Fxx[x, y, z], Fxy[x, y, z],
    //     Fxz[x, y, z]}, {Fxy[x, y, z], Fyy[x, y, z],
    //     Fyz[x, y, z]}, {Fxz[x, y, z], Fyz[x, y, z], Fzz[x, y, z]}};
    // Print["f[", PaddedForm[P0, {13, 12}], "]=",
    //  PaddedForm[F[P0], {13, 12}]]
    // For[i = 1, i <= 10, i++, P0 = P0 - gradF[P0].Inverse[H[P0]];
    //  Print["f[", PaddedForm[P0, {21, 20}], "]=",
    //   PaddedForm[F[P0], {21, 20}]]]
    // P0;
    // {xout, yout, zout} = P0;
    // eq3dsp[xout, yout, zout]

    let super_simplex = SuperSimplex::default();

    PlaneMapBuilder::<_, 2>::new(super_simplex)
        .build()
        .write_to_file("super_simplex.png");

    let super_simplex = super_simplex.set_seed(1);

    PlaneMapBuilder::<_, 2>::new(super_simplex)
        .build()
        .write_to_file("super_simplex_seed=1.png");
}
