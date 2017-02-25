// Copyright 2016 The Noise-rs Developers.
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

extern crate noise;

use noise::modules::*;

mod debug;

fn main() {
    // Primary granite texture. This generates the "roughness" of the texture
    // when lit by a light source.
    let primary_granite = Billow::new()
        .set_seed(0)
        .set_frequency(8.0)
        .set_persistence(0.625)
        .set_lacunarity(2.18359375)
        .set_octaves(6);

    // Use Worley polygons to produce the small grains for the granite texture.
    let base_grains = Worley::new()
        .set_seed(1)
        .set_frequency(16.0)
        .enable_range(true);

    // Scale the small grain values so that they can be added to the base
    // granite texture. Worley polygons normally generate pits, so apply a
    // negative scaling factor to produce bumps instead.
    let scaled_grains = ScaleBias::new(base_grains)
        .set_scale(-0.5)
        .set_bias(0.0);

    // Combine the primary granite texture with the small grain texture.
    let combined_granite = Add::new(primary_granite, scaled_grains);

    // Finally, perturb the granite texture to add realism.
    let final_granite = Turbulence::new(combined_granite)
        .set_seed(2)
        .set_frequency(4.0)
        .set_power(1.0 / 8.0)
        .set_roughness(6);

    debug::render_noise_module("texturegranite.png", final_granite, 1024, 1024, 500);
}
