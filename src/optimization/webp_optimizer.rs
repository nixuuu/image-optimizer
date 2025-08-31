use anyhow::Result;
use image::DynamicImage;
use std::fs;
use std::path::Path;

use crate::cli::Cli;

/// Optimizes a WebP image with configurable quality and lossless options.
///
/// This function uses Google's WebP encoder to create optimized WebP images.
/// It supports both lossy compression with quality control and lossless compression
/// mode for maximum quality preservation.
///
/// # Arguments
///
/// * `input_path` - Path to the source WebP file
/// * `output_path` - Path where the optimized WebP will be written
/// * `args` - CLI configuration containing quality settings and lossless flag
/// * `resized_img` - Optional pre-resized image data; if None, reads from input_path
///
/// # Returns
///
/// Returns `Ok(())` on successful optimization.
///
/// # Errors
///
/// Returns an error if:
/// - WebP encoding fails
/// - File I/O operations fail (reading input or writing output)
/// - Image format conversion to RGB8 fails
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
