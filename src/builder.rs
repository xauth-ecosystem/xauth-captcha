use crate::font::FontManager;
use crate::generator::CaptchaGenerator;
use crate::filters::{geometry::GeometryFilter, noise::NoiseFilter, CaptchaFilter};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::RngExt;
use rusttype::Scale;

pub struct CaptchaBuilder<'a> {
    width: u32,
    height: u32,
    length: usize,
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
            font_manager: None,
            filters: vec![Box::new(NoiseFilter::default()), Box::new(GeometryFilter::default())],
        }
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
        let text = CaptchaGenerator::generate(self.length, None);
        let mut image: RgbImage = ImageBuffer::from_pixel(self.width, self.height, Rgb([255, 255, 255]));

        for filter in &self.filters {
            filter.apply(&mut image);
        }

        let fm = self.font_manager.unwrap_or_default();
        let scale = Scale::uniform(30.0);
        let mut rng = rand::rng();

        let grid = Self::generate_dynamic_grid(self.length, self.width, self.height);

        for (i, c) in text.chars().enumerate() {
            let font = fm.get_random_font().expect("No fonts available");

            let cell = &grid[i];
            
            // Random offset for x and y
            let safe_offset = 15;
            let x_offset = rng.random_range(-safe_offset..=safe_offset);
            let y_offset = rng.random_range(-safe_offset..=safe_offset);

            let glyph = font
                .glyph(c)
                .scaled(scale)
                .positioned(rusttype::point(
                    (cell.x + x_offset) as f32,
                    (cell.y + y_offset) as f32,
                ));

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

                    if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 && v > 0.5 {
                        image.put_pixel(px as u32, py as u32, color);
                    }
                });
            }
        }

        (text, image)
    }

    fn generate_dynamic_grid(char_count: usize, width: u32, height: u32) -> Vec<GridCell> {
        let mut grid = Vec::with_capacity(char_count);
        let margin = 20;
        
        let usable_width = (width as i32).saturating_sub(2 * margin).max(1) as f32;
        
        let mut rng = rand::rng();

        if char_count <= 3 {
            let step_x = usable_width / (char_count + 1) as f32;
            for i in 0..char_count {
                grid.push(GridCell {
                    x: margin + (step_x * (i + 1) as f32) as i32,
                    y: (height / 2) as i32,
                });
            }
        } else {
            let top_row_count = (char_count as f32 / 2.0).ceil() as usize;
            let bottom_row_count = char_count - top_row_count;

            let step_x_top = usable_width / (top_row_count + 1) as f32;
            let step_x_bottom = usable_width / (bottom_row_count + 1) as f32;

            for i in 0..top_row_count {
                grid.push(GridCell {
                    x: margin + (step_x_top * (i + 1) as f32) as i32 + rng.random_range(-5..=5),
                    y: (height as f32 * 0.35) as i32 + rng.random_range(-5..=5),
                });
            }

            for i in 0..bottom_row_count {
                grid.push(GridCell {
                    x: margin + (step_x_bottom * (i + 1) as f32) as i32 + rng.random_range(-5..=5),
                    y: (height as f32 * 0.75) as i32 + rng.random_range(-5..=5),
                });
            }
        }

        grid
    }
}

struct GridCell {
    x: i32,
    y: i32,
}
