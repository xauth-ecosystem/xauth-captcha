use super::CaptchaFilter;
use image::{Rgb, RgbImage};
use rand::RngExt;

pub struct GeometryFilter {
    pub lines: usize,
}

impl Default for GeometryFilter {
    fn default() -> Self {
        Self { lines: 30 }
    }
}

impl GeometryFilter {
    pub fn new(lines: usize) -> Self {
        Self { lines }
    }

    fn draw_line(image: &mut RgbImage, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Rgb<u8>) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && x0 < image.width() as i32 && y0 >= 0 && y0 < image.height() as i32 {
                image.put_pixel(x0 as u32, y0 as u32, color);
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

impl CaptchaFilter for GeometryFilter {
    fn apply(&self, image: &mut RgbImage) {
        let mut rng = rand::rng();
        let width = image.width() as i32;
        let height = image.height() as i32;

        let line_count = rng.random_range((self.lines.saturating_sub(5))..=(self.lines + 5));
        for _ in 0..line_count {
            let color = Rgb([
                rng.random_range(0..220),
                rng.random_range(0..220),
                rng.random_range(0..220),
            ]);
            Self::draw_line(
                image,
                rng.random_range(0..width),
                rng.random_range(0..height),
                rng.random_range(0..width),
                rng.random_range(0..height),
                color,
            );
        }
    }
}
