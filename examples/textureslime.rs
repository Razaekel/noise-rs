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

use noise::*;

mod debug;

fn main() {
    // Large slime bubble texture.
    let large_slime = Billow::new()
        .set_seed(0)
        .set_frequency(4.0)
        .set_lacunarity(2.12109375)
        .set_octaves(1);

    // Base of the small slime bubble texture. This texture will eventually
    // appear inside cracks in the large slime bubble texture.
    let small_slime_base = Billow::new()
        .set_seed(1)
        .set_frequency(24.0)
        .set_lacunarity(2.14453125)
        .set_octaves(1);

    // Scale and lower the small slime bubble values.
    let small_slime = ScaleBias::new(&small_slime_base)
        .set_scale(0.5)
        .set_bias(-0.5);

    // Create a map that specifies where the large and small slime bubble
    // textures will appear in the final texture map.
    let slime_map = RidgedMulti::new()
        .set_seed(2)
        .set_frequency(2.0)
        .set_lacunarity(2.20703125)
        .set_octaves(3);

    // Choose between the large or small slime bubble textures depending on
    // the corresponding value from the slime map. Choose the small slime
    // bubble texture if the slime map value is within a narrow range of
    // values, otherwise choose the large slime bubble texture. The edge
    // falloff is non-zero so that there is a smooth transition between the
    // two textures.
    let slime_chooser = Select::new(&large_slime, &small_slime, &slime_map)
        .set_bounds(-0.375, 0.375)
        .set_edge_falloff(0.5);

    // Finally, perturb the slime texture to add realism.
    let final_slime = Turbulence::new(slime_chooser)
        .set_seed(3)
        .set_frequency(8.0)
        .set_power(1.0 / 32.0)
        .set_roughness(2);

    debug::render_noise_module("textureslime.png", &final_slime, 1024, 1024, 500);
}
