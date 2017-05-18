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
    // Primary jade texture. The ridges from the ridged-multifractal module
    // produces the veins.
    let primary_jade = RidgedMulti::new()
        .set_seed(0)
        .set_frequency(2.0)
        .set_lacunarity(2.20703125)
        .set_octaves(6);

    // Base of the secondary jade texture. The base texture uses concentric
    // cylinders aligned on the z axis, which will eventually be perturbed.
    let base_secondary_jade = Cylinders::new().set_frequency(2.0);

    // Rotate the base secondary jade texture so that the cylinders are not
    // aligned with any axis. This produces more variation in the secondary
    // jade texture since the texture is parallel to the y-axis.
    let rotated_base_secondary_jade = RotatePoint::new(base_secondary_jade)
        .set_angles(90.0, 25.0, 5.0, 0.0);

    // Slightly perturb the secondary jade texture for more realism.
    let perturbed_base_secondary_jade = Turbulence::new(rotated_base_secondary_jade)
        .set_seed(1)
        .set_frequency(4.0)
        .set_power(1.0 / 4.0)
        .set_roughness(4);

    // Scale the secondary jade texture so it makes a small contribution to the
    // final jade texture.
    let secondary_jade = ScaleBias::new(perturbed_base_secondary_jade)
        .set_scale(0.25)
        .set_bias(0.0);

    // Add the two jade textures together. These two textures were produced
    // using different combinations of coherent noise, so the final texture
    // will have a lot of variation.
    let combined_jade = Add::new(primary_jade, secondary_jade);

    // Finally, perturb the combined jade texture to produce the final jade
    // texture. A low roughness produces nice veins.
    let final_jade = Turbulence::new(combined_jade)
        .set_seed(2)
        .set_frequency(4.0)
        .set_power(1.0 / 16.0)
        .set_roughness(2);

    debug::render_noise_module3("texturejade.png", final_jade, 1024, 1024, 500);
}
