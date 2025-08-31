use anyhow::{Context, Result};
use image::DynamicImage;
use std::fs;
use std::path::Path;

use crate::cli::Cli;

/// Optimizes a JPEG image using mozjpeg compression.
///
/// This function uses the mozjpeg library to achieve superior compression compared to
/// standard libjpeg implementations. It supports both quality-based compression and
/// lossless mode, and can work with either the original image data or a pre-resized image.
///
/// # Arguments
///
/// * `input_path` - Path to the source JPEG file
/// * `output_path` - Path where the optimized JPEG will be written
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
/// - JPEG decompression or compression fails
/// - File I/O operations fail (reading input or writing output)
/// - Image dimensions are too large to convert to u32
/// - RGB color space conversion fails
pub fn optimize_jpeg(
    input_path: &Path,
    output_path: &Path,
    args: &Cli,
    resized_img: Option<DynamicImage>,
) -> Result<()> {
    let quality = if args.lossless { 100 } else { args.quality };

    let (width, height, rgb_data) = if let Some(img) = resized_img {
        let rgb_img = img.to_rgb8();
        (rgb_img.width(), rgb_img.height(), rgb_img.into_raw())
    } else {
        let input_data = fs::read(input_path)?;
        let decompress = mozjpeg::Decompress::new_mem(&input_data)?;
        let width = u32::try_from(decompress.width()).context("Width too large")?;
        let height = u32::try_from(decompress.height()).context("Height too large")?;
        let mut decompress_started = decompress.rgb()?;
        let rgb_data: Vec<u8> = decompress_started.read_scanlines()?;
        (width, height, rgb_data)
    };

    let mut compress = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    compress.set_quality(f32::from(quality));
    compress.set_size(width as usize, height as usize);

    let mut output_data = Vec::new();
    let mut compress_started = compress.start_compress(&mut output_data)?;

    let row_stride = (width * 3) as usize;
    for row in rgb_data.chunks(row_stride) {
        compress_started.write_scanlines(row)?;
    }

    compress_started.finish()?;
    fs::write(output_path, output_data)?;

    Ok(())
}
