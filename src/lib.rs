pub mod generator;

use image::{ImageBuffer, Rgb, RgbImage};
use rand::RngExt;
use rusttype::{Font, Scale};
use std::error::Error;

pub fn generate_captcha() -> Result<(), Box<dyn Error>> {
    let width = 128;
    let height = 128;
    
    // Create a white background image
    let mut image: RgbImage = ImageBuffer::from_pixel(width, height, Rgb([255, 255, 255]));

    // Load a bundled font
    let font_data = include_bytes!("../fonts/SourceCodePro.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).ok_or("Failed to load font")?;

    let scale = Scale::uniform(30.0);
    let mut rng = rand::rng();
    
    // Text to render
    let text = generator::CaptchaGenerator::generate(5, None);
    
    let x_base = 20;
    let y_base = 60;

    for (i, c) in text.chars().enumerate() {
        // Random offset for x and y
        let x_offset = rng.random_range(-5..=5);
        let y_offset = rng.random_range(-5..=5);
        
        let glyph = font.glyph(c)
            .scaled(scale)
            .positioned(rusttype::point(
                (x_base + (i * 18) as i32 + x_offset) as f32,
                (y_base + y_offset) as f32,
            ));

        let color = Rgb([
            rng.random_range(0..150),
            rng.random_range(0..150),
            rng.random_range(0..150),
        ]);

        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let px = x as i32 + bounding_box.min.x;
                let py = y as i32 + bounding_box.min.y;
                
                // Only draw if inside image bounds and alpha > 0.5
                if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 && v > 0.5 {
                    image.put_pixel(px as u32, py as u32, color);
                }
            });
        }
    }

    image.save("captcha_output.png")?;
    Ok(())
}
