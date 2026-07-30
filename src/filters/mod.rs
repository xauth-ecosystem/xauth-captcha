pub mod geometry;
pub mod noise;
pub mod wave;

use image::RgbImage;

/// A trait for applying custom image filters or obfuscation to the CAPTCHA background.
pub trait CaptchaFilter {
    fn apply(&self, image: &mut RgbImage);
}
