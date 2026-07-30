use rand::RngExt;
use rusttype::Font;

pub struct FontManager<'a> {
    fonts: Vec<Font<'a>>,
}

impl<'a> Default for FontManager<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> FontManager<'a> {
    /// Creates a new FontManager and loads the default bundled fonts.
    pub fn new() -> Self {
        let mut manager = Self { fonts: Vec::new() };
        manager.load_defaults();
        manager
    }

    fn load_defaults(&mut self) {
        let font_data: &[&[u8]] = &[
            include_bytes!("../fonts/AdventProRegular.ttf"),
            include_bytes!("../fonts/BalooBhaina.ttf"),
            include_bytes!("../fonts/BalooChettanRegular.ttf"),
            include_bytes!("../fonts/BorghsNormal.ttf"),
            include_bytes!("../fonts/CabinSketchRegular.ttf"),
            include_bytes!("../fonts/Chewy.ttf"),
            include_bytes!("../fonts/DINk.ttf"),
            include_bytes!("../fonts/LoveYaLikeASisterRegular.ttf"),
            include_bytes!("../fonts/ShadowsIntoLight.ttf"),
            include_bytes!("../fonts/SourceCodePro.ttf"),
        ];

        for &data in font_data {
            if let Some(font) = Font::try_from_bytes(data) {
                self.fonts.push(font);
            }
        }
    }

    /// Returns a random font from the loaded fonts pool.
    pub fn get_random_font(&self) -> Option<&Font<'a>> {
        if self.fonts.is_empty() {
            return None;
        }
        let mut rng = rand::rng();
        let idx = rng.random_range(0..self.fonts.len());
        Some(&self.fonts[idx])
    }
}
