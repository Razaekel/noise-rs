extern crate noise;

use noise::{utils::*, *};

fn main() {
    // Large slime bubble texture.
    let large_slime = Billow::<Perlin>::new(0)
        .set_frequency(4.0)
        .set_lacunarity(2.12109375)
        .set_octaves(1);

    // Base of the small slime bubble texture. This texture will eventually
    // appear inside cracks in the large slime bubble texture.
    let small_slime_base = Billow::<Perlin>::new(1)
        .set_frequency(24.0)
        .set_lacunarity(2.14453125)
        .set_octaves(1);

    // Scale and lower the small slime bubble values.
    let small_slime = ScaleBias::new(small_slime_base)
        .set_scale(0.5)
        .set_bias(-0.5);

    // Create a map that specifies where the large and small slime bubble
    // textures will appear in the final texture map.
    let slime_map = RidgedMulti::<Perlin>::new(2)
        .set_frequency(2.0)
        .set_lacunarity(2.20703125)
        .set_octaves(3);

    // Choose between the large or small slime bubble textures depending on
    // the corresponding value from the slime map. Choose the small slime
    // bubble texture if the slime map value is within a narrow range of
    // values, otherwise choose the large slime bubble texture. The edge
    // falloff is non-zero so that there is a smooth transition between the
    // two textures.
    let slime_chooser = Select::new(large_slime, small_slime, slime_map)
        .set_bounds(-0.375, 0.375)
        .set_falloff(0.5);

    // Finally, perturb the slime texture to add realism.
    let final_slime = Turbulence::<_, Perlin>::new(slime_chooser)
        .set_seed(3)
        .set_frequency(8.0)
        .set_power(1.0 / 32.0)
        .set_roughness(2);

    let planar_texture = PlaneMapBuilder::<_, 2>::new(&final_slime)
        .set_size(1024, 1024)
        .build();

    let seamless_texture = PlaneMapBuilder::<_, 2>::new(final_slime)
        .set_size(1024, 1024)
        .set_is_seamless(true)
        .build();

    // Create a slime palette.
    let slime_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.0, [160, 64, 42, 255])
        .add_gradient_point(0.0, [64, 192, 64, 255])
        .add_gradient_point(1.0, [128, 255, 128, 255]);

    let mut renderer = ImageRenderer::new().set_gradient(slime_gradient);

    renderer
        .render(&planar_texture)
        .write_to_file("texture_slime_planar.png");

    renderer
        .render(&seamless_texture)
        .write_to_file("texture_slime_seamless.png");
}
