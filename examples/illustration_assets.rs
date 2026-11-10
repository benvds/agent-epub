//! Generate the tiny synthetic image corpus; not required by the CLI.
use image::{ImageFormat, Rgb, RgbImage};
use std::{fs, path::PathBuf};
fn main() {
    let output = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("examples/assets"));
    fs::create_dir_all(&output).unwrap();
    let diagram = RgbImage::from_fn(1200, 300, |x, y| {
        let color = if (35..330).contains(&x) && (55..245).contains(&y) {
            [48, 120, 99]
        } else if (450..750).contains(&x) && (55..245).contains(&y) {
            [45, 99, 151]
        } else if (870..1165).contains(&x) && (55..245).contains(&y) {
            [194, 115, 45]
        } else if ((330..450).contains(&x) || (750..870).contains(&x)) && (140..160).contains(&y) {
            [55, 65, 75]
        } else {
            [246, 245, 240]
        };
        Rgb(color)
    });
    diagram
        .save_with_format(output.join("export-flow.png"), ImageFormat::Png)
        .unwrap();
    let scene = RgbImage::from_fn(900, 600, |x, y| {
        let color = if y < 320 {
            [
                ((x * 100 / 900) + 75) as u8,
                ((y * 110 / 320) + 110) as u8,
                210,
            ]
        } else {
            [
                40,
                ((x + y) * 90 / 1500 + 65) as u8,
                ((x * 40 / 900) + 65) as u8,
            ]
        };
        Rgb(color)
    });
    scene
        .save_with_format(output.join("landscape.jpg"), ImageFormat::Jpeg)
        .unwrap();
    RgbImage::from_pixel(1000, 12, Rgb([45, 99, 151]))
        .save_with_format(output.join("divider.png"), ImageFormat::Png)
        .unwrap();
}
