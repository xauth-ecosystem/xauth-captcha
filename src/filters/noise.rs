use super::CaptchaFilter;
use image::{Rgb, RgbImage};
use rand::RngExt;

pub struct NoiseFilter {
    pub dots: usize,
}

impl Default for NoiseFilter {
    fn default() -> Self {
        Self { dots: 65 }
    }
}

impl NoiseFilter {
    pub fn new(dots: usize) -> Self {
        Self { dots }
    }
}

impl CaptchaFilter for NoiseFilter {
    fn apply(&self, image: &mut RgbImage) {
        let mut rng = rand::rng();
        let width = image.width() as i32;
        let height = image.height() as i32;

        let dot_count = rng.random_range((self.dots.saturating_sub(15))..=(self.dots + 15));
        for _ in 0..dot_count {
            let color = Rgb([
                rng.random_range(0..=255),
                rng.random_range(0..=255),
                rng.random_range(0..=255),
            ]);
            let px = rng.random_range(0..width);
            let py = rng.random_range(0..height);
            if px >= 0 && px < width && py >= 0 && py < height {
                image.put_pixel(px as u32, py as u32, color);
            }
        }
    }
}
