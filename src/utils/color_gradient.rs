pub type Color = [u8; 4];

#[derive(Clone, Copy, Debug, Default)]
struct GradientPoint {
    pos: f64,
    color: Color,
}

#[derive(Clone, Copy, Debug, Default)]
struct GradientDomain {
    min: f64,
    max: f64,
}

impl GradientDomain {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn set_min(&mut self, min: f64) {
        self.min = min;
    }

    pub fn set_max(&mut self, max: f64) {
        self.max = max;
    }
}

#[derive(Clone, Debug, Default)]
pub struct ColorGradient {
    gradient_points: Vec<GradientPoint>,
    domain: GradientDomain,
}

impl ColorGradient {
    pub fn new() -> Self {
        let gradient = Self {
            gradient_points: Vec::new(),
            domain: GradientDomain::new(0.0, 1.0),
        };

        gradient.build_grayscale_gradient()
    }

    pub fn add_gradient_point(mut self, pos: f64, color: Color) -> Self {
        let new_point = GradientPoint { pos, color };

        // first check to see if the position is within the domain of the gradient. if the position
        // is not within the domain, expand the domain and add the GradientPoint
        if self.domain.min > pos {
            self.domain.set_min(pos);
            // since the new position is the smallest value, insert it at the beginning of the
            // gradient
            self.gradient_points.insert(0, new_point);
        } else if self.domain.max < pos {
            self.domain.set_max(pos);

            // since the new position is at the largest value, insert it at the end of the gradient
            self.gradient_points.push(new_point)
        } else if !self // new point must be somewhere inside the existing domain. Check to see if
            // it doesn't exist already
            .gradient_points
            .iter()
            .any(|&x| (x.pos - pos).abs() < f64::EPSILON)
        {
            // it doesn't, so find the correct position to insert the new
            // control point.
            let insertion_point = self.find_insertion_point(pos);

            // add the new control point at the correct position.
            self.gradient_points.insert(insertion_point, new_point);
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
        let mut color = [0; 4];

        if pos < self.domain.min {
            color = self.gradient_points.first().unwrap().color
        } else if pos > self.domain.max {
            color = self.gradient_points.last().unwrap().color
        } else {
            for points in self.gradient_points.windows(2) {
                if (points[0].pos <= pos) && (points[1].pos > pos) {
                    // Compute the alpha value used for linear interpolation
                    let alpha = (pos - points[0].pos) / (points[1].pos - points[0].pos);

                    // Now perform the linear interpolation and return.
                    color = linerp_color(points[0].color, points[1].color, alpha)
                }
            }
        };

        color
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
