use anyhow::Result;
use image::DynamicImage;
use std::fs;
use std::path::Path;

use crate::cli::Cli;

/// Optimizes a WebP image with configurable quality and lossless options
///
/// # Errors
/// Returns an error if WebP encoding fails or file I/O operations fail
pub fn optimize_webp(
    input_path: &Path,
    output_path: &Path,
    args: &Cli,
    resized_img: Option<DynamicImage>,
) -> Result<()> {
    let rgb_img = if let Some(img) = resized_img {
        img.to_rgb8()
    } else {
        image::open(input_path)?.to_rgb8()
    };

    let encoder = if args.lossless {
        webp::Encoder::from_rgb(&rgb_img, rgb_img.width(), rgb_img.height()).encode_lossless()
    } else {
        webp::Encoder::from_rgb(&rgb_img, rgb_img.width(), rgb_img.height())
            .encode(f32::from(args.quality))
    };

    fs::write(output_path, &*encoder)?;

    Ok(())
}
