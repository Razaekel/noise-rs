pub type Color = [u8; 4];

#[derive(Clone, Copy, Debug, Default)]
struct GradientPoint {
    pos: f64,
    color: Color,
}

#[derive(Clone, Debug, Default)]
pub struct ColorGradient {
    gradient_points: Vec<GradientPoint>,
}

impl ColorGradient {
    pub fn new() -> Self {
        let gradient = Self {
            gradient_points: Vec::new(),
        };

        gradient.build_grayscale_gradient()
    }

    pub fn add_gradient_point(mut self, pos: f64, color: Color) -> Self {
        // check to see if the vector already contains the input point.
        if !self
            .gradient_points
            .iter()
            .any(|&x| (x.pos - pos).abs() < std::f64::EPSILON)
        {
            // it doesn't, so find the correct position to insert the new
            // control point.
            let insertion_point = self.find_insertion_point(pos);

            // add the new control point at the correct position.
            self.gradient_points
                .insert(insertion_point, GradientPoint { pos, color });
        }

        self
    }

    fn find_insertion_point(&self, pos: f64) -> usize {
        self.gradient_points
            .iter()
            .position(|x| x.pos >= pos)
            .unwrap_or_else(|| self.gradient_points.len())
    }

    pub fn clear_gradient(mut self) -> Self {
        self.gradient_points.clear();

        self
    }

    pub fn build_grayscale_gradient(self) -> Self {
        self.clear_gradient()
            .add_gradient_point(-1.0, [0, 0, 0, 255])
            .add_gradient_point(1.0, [255, 255, 255, 255])
    }

    #[rustfmt::skip]
    pub fn build_terrain_gradient(self) -> Self {
        self.clear_gradient()
            .add_gradient_point(-1.00,              [  0,   0,   0, 255])
            .add_gradient_point(-256.0 / 16384.0,   [  6,  58, 127, 255])
            .add_gradient_point(-1.0 / 16384.0,     [ 14, 112, 192, 255])
            .add_gradient_point(0.0,                [ 70, 120,  60, 255])
            .add_gradient_point(1024.0 / 16384.0,   [110, 140,  75, 255])
            .add_gradient_point(2048.0 / 16384.0,   [160, 140, 111, 255])
            .add_gradient_point(3072.0 / 16384.0,   [184, 163, 141, 255])
            .add_gradient_point(4096.0 / 16384.0,   [128, 128, 128, 255])
            .add_gradient_point(5632.0 / 16384.0,   [128, 128, 128, 255])
            .add_gradient_point(6144.0 / 16384.0,   [250, 250, 250, 255])
            .add_gradient_point(1.0,                [255, 255, 255, 255])
    }

    pub fn build_rainbow_gradient(self) -> Self {
        self.clear_gradient()
            .add_gradient_point(-1.0, [255, 0, 0, 255])
            .add_gradient_point(-0.7, [255, 255, 0, 255])
            .add_gradient_point(-0.4, [0, 255, 0, 255])
            .add_gradient_point(0.0, [0, 255, 255, 255])
            .add_gradient_point(0.3, [0, 0, 255, 255])
            .add_gradient_point(0.6, [255, 0, 255, 255])
            .add_gradient_point(1.0, [255, 0, 0, 255])
    }

    pub fn get_color(&self, pos: f64) -> Color {
        // confirm that there's at least 2 control points in the vector.
        assert!(self.gradient_points.len() >= 2);

        // we need to clamp the value to the range of pos in the gradient.
        let clamped_pos = pos.clamp(
            self.gradient_points[0].pos,
            self.gradient_points[self.gradient_points.len() - 1].pos,
        );

        // Find the first element in the control point array that has a input
        // value larger than the output value from the source module
        let index = self
            .gradient_points
            .iter()
            .position(|&x| (x.pos > clamped_pos))
            .unwrap_or_else(|| self.gradient_points.len());

        if index < 1 {
            println!(
                "index_pos in curve was less than 1! source value was {}",
                pos
            );
        }

        // Find the two nearest control points so that we can perform linear
        // interpolation.
        let index1 = (index - 1).clamp(0, self.gradient_points.len() - 1);
        let index2 = index.clamp(0, self.gradient_points.len() - 1);

        // If some control points are missing (which occurs if the value from
        // the source module is greater than the largest input value or less
        // than the smallest input value of the control point array), get the
        // corresponding output value of the nearest control point and exit.
        if index1 == index2 {
            return self.gradient_points[index1].color;
        }

        // Compute the alpha value used for linear interpolation
        let input0 = self.gradient_points[index1].pos;
        let input1 = self.gradient_points[index2].pos;
        let alpha = (pos - input0) / (input1 - input0);

        // Now perform the linear interpolation and return.
        linerp_color(
            self.gradient_points[index1].color,
            self.gradient_points[index2].color,
            alpha,
        )
    }
}

fn blend_channels(channel0: u8, channel1: u8, alpha: f64) -> u8 {
    let c0 = (f64::from(channel0)) / 255.0;
    let c1 = (f64::from(channel1)) / 255.0;

    (((c1 * alpha) + (c0 * (1.0 - alpha))) * 255.0) as u8
}

fn linerp_color(color0: Color, color1: Color, alpha: f64) -> Color {
    let r = blend_channels(color0[0], color1[0], alpha);
    let g = blend_channels(color0[1], color1[1], alpha);
    let b = blend_channels(color0[2], color1[2], alpha);
    let a = blend_channels(color0[3], color1[3], alpha);

    [r, g, b, a]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_channels_min() {
        let result = blend_channels(0, 255, 0.0);
        assert_eq!(
            0, result,
            "blend_channels should've created 0, produced `{}` instead",
            result
        );
    }

    #[test]
    fn blend_channels_mid() {
        let result = blend_channels(0, 255, 0.5);
        assert_eq!(
            127, result,
            "blend_channels should've created 127, produced `{}` instead",
            result
        );
    }

    #[test]
    fn blend_channels_max() {
        let result = blend_channels(0, 255, 1.0);
        assert_eq!(
            255, result,
            "blend_channels should've created 255, produced `{}` instead",
            result
        );
    }

    #[test]
    fn linerp_color_1() {
        assert_eq!(
            [0, 127, 255, 0],
            linerp_color([0, 0, 255, 0], [0, 255, 255, 0], 0.5)
        );
    }

    #[test]
    fn color_gradient_1() {
        let gradient = ColorGradient::new();

        let gradient = gradient
            .clear_gradient()
            .add_gradient_point(0.0, [0, 0, 0, 0])
            .add_gradient_point(1.0, [255, 255, 255, 255]);

        assert_eq!([127, 127, 127, 127], gradient.get_color(0.5));
    }
}
