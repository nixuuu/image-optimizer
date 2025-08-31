use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat};
use std::fs;
use std::path::Path;

use crate::cli::Cli;

/// Optimizes a PNG image using oxipng with zopfli compression.
///
/// This function uses oxipng with zopfli compression for maximum PNG optimization.
/// It enables alpha optimization and safe chunk stripping for the best balance
/// between file size reduction and compatibility.
///
/// # Arguments
///
/// * `input_path` - Path to the source PNG file
/// * `output_path` - Path where the optimized PNG will be written
/// * `_args` - CLI configuration (currently unused for PNG optimization)
/// * `resized_img` - Optional pre-resized image data; if None, copies from `input_path`
///
/// # Returns
///
/// Returns `Ok(())` on successful optimization.
///
/// # Errors
///
/// Returns an error if:
/// - PNG optimization fails
/// - File I/O operations fail (copying or saving)
/// - Image format conversion fails
pub fn optimize_png(
    input_path: &Path,
    output_path: &Path,
    _args: &Cli,
    resized_img: Option<DynamicImage>,
) -> Result<()> {
    if let Some(img) = resized_img {
        img.save_with_format(output_path, ImageFormat::Png)?;
    } else {
        fs::copy(input_path, output_path)?;
    }

    let options = oxipng::Options {
        optimize_alpha: true,
        strip: oxipng::StripChunks::Safe,
        ..Default::default()
    };

    let input_file = oxipng::InFile::Path(output_path.to_path_buf());
    let output_file = oxipng::OutFile::Path {
        path: Some(output_path.to_path_buf()),
        preserve_attrs: true,
    };

    oxipng::optimize(&input_file, &output_file, &options).context("Failed to optimize PNG")?;

    Ok(())
}
