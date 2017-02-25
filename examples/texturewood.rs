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
    // Base wood texture. Uses concentric cylinders aligned on the z-axis, like a log.
    let base_wood = Cylinders::new().set_frequency(16.0);

    // Basic Multifractal noise to use for the wood grain.
    let wood_grain_noise = BasicMulti::new()
        .set_seed(0)
        .set_frequency(48.0)
        .set_persistence(0.5)
        .set_lacunarity(2.20703125)
        .set_octaves(3);

    // Stretch the perlin noise in the same direction as the center of the log. Should
    // produce a nice wood-grain texture.
    let scaled_base_wood_grain = ScalePoint::new(wood_grain_noise)
        .set_y_scale(0.25);

    // Scale the wood-grain values so that they can be added to the base wood texture.
    let wood_grain = ScaleBias::new(scaled_base_wood_grain)
        .set_scale(0.25)
        .set_bias(0.125);

    // Add the wood grain texture to the base wood texture.
    let combined_wood = Add::new(base_wood, wood_grain);

    // Slightly perturb the wood to create a more realistic texture.
    let perturbed_wood = Turbulence::new(combined_wood)
        .set_seed(1)
        .set_frequency(4.0)
        .set_power(1.0/256.0)
        .set_roughness(4);

    // Cut the wood texture a small distance from the center of the log.
    let translated_wood = TranslatePoint::new(perturbed_wood)
        .set_y_translation(1.48);

    // Set the cut on a angle to produce a more interesting texture.
    let rotated_wood = RotatePoint::new(translated_wood)
        .set_angles(84.0, 0.0, 0.0, 0.0);

    // Finally, perturb the wood texture again to produce the final texture.
    let final_wood = Turbulence::new(rotated_wood)
        .set_seed(2)
        .set_frequency(2.0)
        .set_power(1.0/64.0)
        .set_roughness(4);

    debug::render_noise_module("texturewood.png", final_wood, 1024, 1024, 100);
}
