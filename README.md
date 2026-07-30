# xauth-captcha

A lightweight, robust, and dependency-minimal CAPTCHA generation library for Rust. Originally designed for the `xauth-ecosystem`, it provides an easy-to-use Builder API, dynamic character grids, and built-in obfuscation (noise and dots).

## Features

- **Dynamic Grid Layout**: Automatically calculates whether to place text on a single line (<= 3 characters) or intelligently split it into two rows for improved bot resistance.
- **Built-in Fonts**: Bundles 10 unique TrueType fonts. Each character is rendered using a randomly selected font from the pool.
- **Lightweight Noise Generation**: Employs a custom, zero-dependency Bresenham's line algorithm to render obfuscation lines and random dots without pulling in heavy external image processing crates.
- **Random Code Generation**: Secure text generation using a strictly safe character set (`ABCDEFGHJKLMNPQRSTUVWXYZ23456789`) to prevent ambiguous characters like `0` vs `O` or `1` vs `I`.
- **Fluent API**: Highly configurable `CaptchaBuilder` for seamless integration.

## Usage

```rust
use xauth_captcha::CaptchaBuilder;

fn main() {
    // Generate a secure 6-character captcha
    let (text, image) = CaptchaBuilder::new()
        .width(128)
        .height(128)
        .length(6)
        .build();
    
    // Save to file or encode to base64 for web transit
    image.save("captcha_output.png").unwrap();
    println!("Generated CAPTCHA: {}", text);
}
```

## Running the Example

You can run the included example to generate a sample CAPTCHA image directly:

```bash
cargo run --example simple
```

Check the root directory for the resulting `captcha_output.png` file!

## License

This library is licensed under the CSSM Unlimited License v2.0 (CSSM-ULv2). Please note that this is a custom license. See the [LICENSE](LICENSE) file for details.
