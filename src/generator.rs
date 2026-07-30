use rand::RngExt;

pub const DEFAULT_CHARSET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

pub struct CaptchaGenerator;

impl CaptchaGenerator {
    /// Generates a random captcha code of the specified length.
    /// If no charset is provided, uses the safe default charset.
    pub fn generate(length: usize, charset: Option<&str>) -> String {
        let chars = charset.unwrap_or(DEFAULT_CHARSET);
        let chars_vec: Vec<char> = chars.chars().collect();
        let mut rng = rand::rng();
        
        let mut result = String::with_capacity(length);
        for _ in 0..length {
            let idx = rng.random_range(0..chars_vec.len());
            result.push(chars_vec[idx]);
        }
        
        result
    }
}
