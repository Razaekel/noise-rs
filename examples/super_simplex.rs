// Copyright 2015 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of using Super Simplex noise

extern crate noise;

use noise::{SuperSimplex, Seedable};

mod debug;

fn main() {
    let mut lookup_2d: [([i8; 2], [f64; 2]); 8 * 4] = [([0; 2], [0.0; 2]); 8 * 4];
    let mut lookup_3d: [[i8; 3]; 16 * 4] = [[0; 3]; 16 * 4];

    let skew_constant = -0.211324865405187;
    for i in 0..8 {
	    let (i1, j1, i2, j2);
	    if i & 1 == 0 {
		    if i & 2 == 0 { i1 = -1; j1 = 0; } else { i1 = 1; j1 = 0; }
		    if i & 4 == 0 { i2 = 0; j2 = -1; } else { i2 = 0; j2 = 1; }
	    } else {
		    if i & 2 != 0 { i1 = 2; j1 = 1; } else { i1 = 0; j1 = 1; }
		    if i & 4 != 0 { i2 = 1; j2 = 2; } else { i2 = 1; j2 = 0; }
	    }
	    lookup_2d[i * 4 + 0] = ([0, 0], [0.0, 0.0]);
        let skew_factor = -1.0 - 2.0 * skew_constant;
	    lookup_2d[i * 4 + 1] = ([1, 1], [skew_factor, skew_factor]);
        let skew_factor = (i1 as f64 + j1 as f64) * skew_constant;
	    lookup_2d[i * 4 + 2] = ([i1, j1], [-i1 as f64 - skew_factor, -j1 as f64 - skew_factor]);
        let skew_factor = (i2 as f64 + j2 as f64) * skew_constant;
	    lookup_2d[i * 4 + 3] = ([i2, j2], [-i2 as f64 - skew_factor, -j2 as f64 - skew_factor]);
    }

    print!("lookup_2d = [");
    for x in &lookup_2d {
        print!("([{}, {}], [{}f64, {}f64]),", x.0[0], x.0[1], x.1[0], x.1[1]);
    }
    println!("\x08]");

    for i in 0..16 {
	    let (i1, j1, k1, i2, j2, k2, i3, j3, k3, i4, j4, k4);
	    if i & 1 != 0 { i1 = 1; j1 = 1; k1 = 1; } else { i1 = 0; j1 = 0; k1 = 0; }
	    if i & 2 != 0 { i2 = 0; j2 = 1; k2 = 1; } else { i2 = 1; j2 = 0; k2 = 0; }
	    if i & 4 != 0 { j3 = 0; i3 = 1; k3 = 1; } else { j3 = 1; i3 = 0; k3 = 0; }
	    if i & 8 != 0 { k4 = 0; i4 = 1; j4 = 1; } else { k4 = 1; i4 = 0; j4 = 0; }
	    lookup_3d[i * 4 + 0] = [i1, j1, k1];
	    lookup_3d[i * 4 + 1] = [i2, j2, k2];
	    lookup_3d[i * 4 + 2] = [i3, j3, k3];
	    lookup_3d[i * 4 + 3] = [i4, j4, k4];
    }

     print!("lookup_3d = [");
    for x in lookup_3d.iter() {
        print!("[{}, {}, {}],", x[0], x[1], x[2]);
    }
    println!("\x08]");

    debug::render_noise_module("super_simplex.png", SuperSimplex::new(), 1024, 1024, 50);
    debug::render_noise_module("super_simplex_seeded.png", SuperSimplex::new().set_seed(1), 1024, 1024, 50);
}
