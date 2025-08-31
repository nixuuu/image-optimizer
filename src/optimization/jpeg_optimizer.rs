use anyhow::{Context, Result};
use image::DynamicImage;
use std::fs;
use std::path::Path;

use crate::cli::Cli;

/// Optimizes a JPEG image using mozjpeg compression
///
/// # Errors
/// Returns an error if JPEG compression fails or file I/O operations fail
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
