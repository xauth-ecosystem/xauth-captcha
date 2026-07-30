//! Captcha generation engine and image builder.
//!
//! The core generation and dynamic grid algorithms are based on
//! the original PHP implementation by `tarunk04/Captcha_Generator`.

use crate::filters::{CaptchaFilter, geometry::GeometryFilter, noise::NoiseFilter};
use crate::font::FontManager;
use crate::generator::CaptchaGenerator;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::RngExt;
use rusttype::Scale;

pub struct CaptchaBuilder<'a> {
    width: u32,
    height: u32,
    length: usize,
    charset: Option<String>,
    font_manager: Option<FontManager<'a>>,
    filters: Vec<Box<dyn CaptchaFilter>>,
}

impl<'a> Default for CaptchaBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CaptchaBuilder<'a> {
    pub fn new() -> Self {
        Self {
            width: 128,
            height: 128,
            length: 5,
            charset: None,
            font_manager: None,
            filters: vec![
                Box::new(NoiseFilter::default()),
                Box::new(GeometryFilter::default()),
            ],
        }
    }

    pub fn from_config(config: &crate::config::CaptchaConfig) -> Self {
        let mut builder = Self::new()
            .width(config.width)
            .height(config.height)
            .length(config.length)
            .charset(config.charset.clone())
            .clear_filters();

        let dots = config.noise_dots.unwrap_or(65);
        builder = builder.add_filter(NoiseFilter::new(dots));

        let lines = config.noise_lines.unwrap_or(30);
        builder = builder.add_filter(GeometryFilter::new(lines));

        if config.enable_wave {
            builder = builder.add_filter(crate::filters::wave::WaveFilter::default());
        }

        builder
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    pub fn charset(mut self, charset: Option<String>) -> Self {
        self.charset = charset;
        self
    }

    pub fn font_manager(mut self, manager: FontManager<'a>) -> Self {
        self.font_manager = Some(manager);
        self
    }

    pub fn add_filter<F: CaptchaFilter + 'static>(mut self, filter: F) -> Self {
        self.filters.push(Box::new(filter));
        self
    }

    pub fn clear_filters(mut self) -> Self {
        self.filters.clear();
        self
    }

    pub fn build(self) -> (String, RgbImage) {
        let text = CaptchaGenerator::generate(self.length, self.charset.as_deref());
        let mut image: RgbImage =
            ImageBuffer::from_pixel(self.width, self.height, Rgb([255, 255, 255]));

        let fm = self.font_manager.unwrap_or_default();
        let mut rng = rand::rng();

        let (grid, scale) = Self::generate_dynamic_grid(self.length, self.width, self.height);

        for (i, c) in text.chars().enumerate() {
            let font = fm.get_random_font().expect("No fonts available");

            let cell = &grid[i];

            let v_metrics = font.v_metrics(scale);
            let h_metrics = font.glyph(c).scaled(scale).h_metrics();

            // Align center of text to center of cell
            let baseline_y = cell.y as f32 + (v_metrics.ascent + v_metrics.descent) / 2.0;
            let start_x = cell.x as f32 - (h_metrics.advance_width / 2.0);

            let glyph = font
                .glyph(c)
                .scaled(scale)
                .positioned(rusttype::point(start_x, baseline_y));

            let color = Rgb([
                rng.random_range(0..150),
                rng.random_range(0..150),
                rng.random_range(0..150),
            ]);

            let angle: f32 = rng.random_range(-25.0..=25.0);
            let rad = angle.to_radians();
            let cos_a = rad.cos();
            let sin_a = rad.sin();

            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                let width = (bounding_box.max.x - bounding_box.min.x) as f32;
                let height = (bounding_box.max.y - bounding_box.min.y) as f32;
                let cx = width / 2.0;
                let cy = height / 2.0;

                glyph.draw(|x, y, v| {
                    let rx = x as f32 - cx;
                    let ry = y as f32 - cy;

                    let rot_x = rx * cos_a - ry * sin_a;
                    let rot_y = rx * sin_a + ry * cos_a;

                    let px = (rot_x + cx + bounding_box.min.x as f32) as i32;
                    let py = (rot_y + cy + bounding_box.min.y as f32) as i32;

                    if px >= 0
                        && px < self.width as i32
                        && py >= 0
                        && py < self.height as i32
                        && v > 0.5
                    {
                        image.put_pixel(px as u32, py as u32, color);
                    }
                });
            }
        }

        // Apply filters at the very end so they distort the text too!
        for filter in &self.filters {
            filter.apply(&mut image);
        }

        (text, image)
    }

    fn generate_dynamic_grid(char_count: usize, width: u32, height: u32) -> (Vec<GridCell>, Scale) {
        let mut grid = Vec::with_capacity(char_count);
        let margin = (width.min(height) / 10).clamp(5, 20) as i32;

        let usable_width = (width as i32).saturating_sub(2 * margin).max(1) as f32;
        let usable_height = (height as i32).saturating_sub(2 * margin).max(1) as f32;

        let mut best_cols = 1;
        let mut max_cell_size = 0.0_f32;

        for c in 1..=char_count {
            let r = (char_count as f32 / c as f32).ceil() as usize;
            let cell_w = usable_width / c as f32;
            let cell_h = usable_height / r as f32;
            let size = cell_w.min(cell_h);
            if size > max_cell_size {
                max_cell_size = size;
                best_cols = c;
            }
        }

        let cols = best_cols;
        let rows = (char_count as f32 / cols as f32).ceil() as usize;

        let font_size = max_cell_size * 0.8;
        let scale = Scale::uniform(font_size.clamp(10.0, usable_height * 0.8));

        let cell_height = usable_height / rows as f32;
        let mut rng = rand::rng();
        let mut current_char = 0;

        for row in 0..rows {
            let chars_in_this_row = if row == rows - 1 {
                char_count - current_char
            } else {
                cols
            };

            let cell_width = usable_width / chars_in_this_row as f32;
            let y_base = margin as f32 + (row as f32 * cell_height) + (cell_height / 2.0);

            for col in 0..chars_in_this_row {
                let x_base = margin as f32 + (col as f32 * cell_width) + (cell_width / 2.0);
                grid.push(GridCell {
                    x: x_base as i32 + rng.random_range(-5..=5),
                    y: y_base as i32 + rng.random_range(-5..=5),
                });
                current_char += 1;
            }
        }

        (grid, scale)
    }
}

struct GridCell {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_defaults() {
        let (text, image) = CaptchaBuilder::new()
            .length(4)
            .width(100)
            .height(50)
            .build();
        assert_eq!(text.len(), 4);
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 50);
    }

    #[test]
    fn test_builder_from_config() {
        let config = crate::config::CaptchaConfig {
            width: 200,
            height: 100,
            length: 8,
            noise_dots: None,
            noise_lines: None,
            charset: None,
            enable_wave: false,
        };
        let (text, image) = CaptchaBuilder::from_config(&config).build();
        assert_eq!(text.len(), 8);
        assert_eq!(image.width(), 200);
        assert_eq!(image.height(), 100);
    }
}
