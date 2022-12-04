extern crate noise;

use noise::{utils::*, *};

fn main() {
    // Primary jade texture. The ridges from the ridged-multifractal function
    // produces the veins.
    let primary_jade = RidgedMulti::<Perlin>::new(0)
        .set_frequency(2.0)
        .set_lacunarity(2.20703125)
        .set_octaves(6);

    // Base of the secondary jade texture. The base texture uses concentric
    // cylinders aligned on the z axis, which will eventually be perturbed.
    let base_secondary_jade = Cylinders::new().set_frequency(2.0);

    // Rotate the base secondary jade texture so that the cylinders are not
    // aligned with any axis. This produces more variation in the secondary
    // jade texture since the texture is parallel to the y-axis.
    let rotated_base_secondary_jade =
        RotatePoint::new(base_secondary_jade).set_angles(90.0, 25.0, 5.0, 0.0);

    // Slightly perturb the secondary jade texture for more realism.
    let perturbed_base_secondary_jade = Turbulence::<_, Perlin>::new(rotated_base_secondary_jade)
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
    let final_jade = Turbulence::<_, Perlin>::new(combined_jade)
        .set_seed(2)
        .set_frequency(4.0)
        .set_power(1.0 / 16.0)
        .set_roughness(2);

    let planar_texture = PlaneMapBuilder::<_, 2>::new(&final_jade)
        .set_size(1024, 1024)
        .build();

    let seamless_texture = PlaneMapBuilder::<_, 2>::new(final_jade)
        .set_size(1024, 1024)
        .set_is_seamless(true)
        .build();

    // Create a jade palette.
    let jade_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.000, [24, 146, 102, 255])
        .add_gradient_point(0.000, [78, 154, 115, 255])
        .add_gradient_point(0.250, [128, 204, 165, 255])
        .add_gradient_point(0.375, [78, 154, 115, 255])
        .add_gradient_point(1.000, [29, 135, 102, 255]);

    let mut renderer = ImageRenderer::new().set_gradient(jade_gradient);

    renderer
        .render(&planar_texture)
        .write_to_file("texture_jade_planar.png");

    renderer
        .render(&seamless_texture)
        .write_to_file("texture_jade_seamless.png");
}
