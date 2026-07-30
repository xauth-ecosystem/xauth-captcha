use xauth_captcha::CaptchaBuilder;

fn main() {
    println!("Generating captcha...");
    let (text, image) = CaptchaBuilder::new().length(6).build();

    if let Err(e) = image.save("captcha_output.png") {
        eprintln!("Error saving captcha: {}", e);
    } else {
        println!(
            "Captcha '{}' successfully generated as captcha_output.png",
            text
        );
    }
}
