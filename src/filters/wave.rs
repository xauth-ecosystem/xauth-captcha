use super::CaptchaFilter;
use image::{Rgb, RgbImage};
use rand::RngExt;

pub struct WaveFilter {
    pub x_amplitude: f32,
    pub y_amplitude: f32,
    pub x_frequency: f32,
    pub y_frequency: f32,
}

impl Default for WaveFilter {
    fn default() -> Self {
        Self {
            x_amplitude: 3.5,
            y_amplitude: 3.5,
            x_frequency: 18.0,
            y_frequency: 18.0,
        }
    }
}

impl WaveFilter {
    pub fn new(x_amplitude: f32, y_amplitude: f32, x_frequency: f32, y_frequency: f32) -> Self {
        Self {
            x_amplitude,
            y_amplitude,
            x_frequency,
            y_frequency,
        }
    }
}

impl CaptchaFilter for WaveFilter {
    fn apply(&self, image: &mut RgbImage) {
        let mut rng = rand::rng();
        let width = image.width() as i32;
        let height = image.height() as i32;

        let original = image.clone();
        
        let x_phase: f32 = rng.random_range(0.0..std::f32::consts::TAU);
        let y_phase: f32 = rng.random_range(0.0..std::f32::consts::TAU);

        for y in 0..height {
            for x in 0..width {
                let src_x = (x as f32 + self.x_amplitude * ((y as f32 / self.x_frequency) + x_phase).sin()) as i32;
                let src_y = (y as f32 + self.y_amplitude * ((x as f32 / self.y_frequency) + y_phase).sin()) as i32;

                if src_x >= 0 && src_x < width && src_y >= 0 && src_y < height {
                    image.put_pixel(x as u32, y as u32, *original.get_pixel(src_x as u32, src_y as u32));
                } else {
                    image.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
                }
            }
        }
    }
}
