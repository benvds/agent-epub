//! Generate synthetic, non-publishing cover fixtures for evaluations and benchmarks.
use image::{DynamicImage, ImageFormat, RgbImage};
use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .ok_or("supply an output directory")?,
    );
    fs::create_dir_all(&output)?;
    let pixels = RgbImage::from_fn(900, 1350, |x, y| {
        if (120..780).contains(&x) && (250..350).contains(&y) {
            image::Rgb([240, 226, 190])
        } else {
            image::Rgb([24 + (x / 20) as u8, 50 + (y / 30) as u8, 100])
        }
    });
    let image = DynamicImage::ImageRgb8(pixels);
    image.save_with_format(output.join("cover.png"), ImageFormat::Png)?;
    image.save_with_format(output.join("cover.jpg"), ImageFormat::Jpeg)?;
    if std::env::args().any(|argument| argument == "--benchmark") {
        let mut state = 0x8a5c_14d2_70e6_392bu64;
        let noise = RgbImage::from_fn(1800, 1800, |_, _| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            image::Rgb([state as u8, (state >> 8) as u8, (state >> 16) as u8])
        });
        noise.save_with_format(output.join("benchmark-cover.png"), ImageFormat::Png)?;
    }
    Ok(())
}
