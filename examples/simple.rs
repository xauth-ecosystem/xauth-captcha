use xauth_captcha::generate_captcha;

fn main() {
    println!("Generating captcha...");
    if let Err(e) = generate_captcha() {
        eprintln!("Error generating captcha: {}", e);
    } else {
        println!("Captcha successfully generated as captcha_output.png");
    }
}
