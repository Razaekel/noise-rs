extern crate noise;

use noise::*;

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
    let scaled_grains = ScaleBias::new(&base_grains).set_scale(-0.5).set_bias(0.0);

    // Combine the primary granite texture with the small grain texture.
    let combined_granite = Add::new(&primary_granite, &scaled_grains);

    // Finally, perturb the granite texture to add realism.
    let final_granite = Turbulence::new(combined_granite)
        .set_seed(2)
        .set_frequency(4.0)
        .set_power(1.0 / 8.0)
        .set_roughness(6);

    debug::render_noise_module3("texturegranite.png", &final_granite, 1024, 1024, 500);
}
