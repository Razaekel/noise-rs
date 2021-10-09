extern crate noise;

use noise::{core::worley::ReturnType, utils::*, *};

fn main() {
    // Primary granite texture. This generates the "roughness" of the texture
    // when lit by a light source.
    let primary_granite = Billow::<Perlin>::new(0)
        .set_frequency(8.0)
        .set_persistence(0.625)
        .set_lacunarity(2.18359375)
        .set_octaves(6);

    // Use Worley polygons to produce the small grains for the granite texture.
    let base_grains = Worley::new(1)
        .set_frequency(16.0)
        .set_return_type(ReturnType::Distance);

    // Scale the small grain values so that they can be added to the base
    // granite texture. Worley polygons normally generate pits, so apply a
    // negative scaling factor to produce bumps instead.
    let scaled_grains = ScaleBias::new(base_grains).set_scale(-0.5).set_bias(0.0);

    // Combine the primary granite texture with the small grain texture.
    let combined_granite = Add::new(primary_granite, scaled_grains);

    // Finally, perturb the granite texture to add realism.
    let final_granite = Turbulence::<_, Perlin>::new(combined_granite)
        .set_seed(2)
        .set_frequency(4.0)
        .set_power(1.0 / 8.0)
        .set_roughness(6);

    let planar_texture = PlaneMapBuilder::new(&final_granite)
        .set_size(1024, 1024)
        .build();

    let seamless_texture = PlaneMapBuilder::new(&final_granite)
        .set_size(1024, 1024)
        .set_is_seamless(true)
        .build();

    let sphere_texture = SphereMapBuilder::new(final_granite)
        .set_size(1024, 512)
        .set_bounds(-90.0, 90.0, -180.0, 180.0)
        .build();

    // Create a gray granite palette. Black and pink appear at either ends of the palette; these
    // colors provide the characteristic flecks in granite.
    let granite_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.0000, [0, 0, 0, 255])
        .add_gradient_point(-0.9375, [0, 0, 0, 255])
        .add_gradient_point(-0.8750, [216, 216, 242, 255])
        .add_gradient_point(0.0000, [191, 191, 191, 255])
        .add_gradient_point(0.5000, [210, 116, 125, 255])
        .add_gradient_point(0.7500, [210, 113, 98, 255])
        .add_gradient_point(1.0000, [255, 176, 192, 255]);

    let mut renderer = ImageRenderer::new().set_gradient(granite_gradient);

    renderer
        .render(&planar_texture)
        .write_to_file("texture_granite_planar.png");

    renderer
        .render(&seamless_texture)
        .write_to_file("texture_granite_seamless.png");

    renderer
        .render(&sphere_texture)
        .write_to_file("texture_granite_sphere.png");
}
