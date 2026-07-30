use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CaptchaConfig {
    #[serde(default = "default_width")]
    pub width: u32,
    #[serde(default = "default_height")]
    pub height: u32,
    #[serde(default = "default_length")]
    pub length: usize,
    #[serde(default)]
    pub noise_dots: Option<usize>,
    #[serde(default)]
    pub noise_lines: Option<usize>,
    #[serde(default)]
    pub enable_wave: bool,
}

impl Default for CaptchaConfig {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            length: default_length(),
            noise_dots: None,
            noise_lines: None,
            enable_wave: false,
        }
    }
}

fn default_width() -> u32 {
    128
}
fn default_height() -> u32 {
    128
}
fn default_length() -> usize {
    5
}
