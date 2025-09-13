use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat};
use std::fs;
use std::path::Path;

use crate::cli::Cli;

/// Optimizes a PNG image using oxipng with configurable optimization levels.
///
/// This function uses `oxipng` with configurable optimization levels (0-6 or `"max"`).
/// Higher levels use zopfli compression for better compression at the cost of speed.
/// It enables alpha optimization and safe chunk stripping for the best balance
/// between file size reduction and compatibility.
///
/// # Arguments
///
/// * `input_path` - Path to the source PNG file
/// * `output_path` - Path where the optimized PNG will be written
/// * `args` - CLI configuration containing oxipng optimization level
/// * `resized_img` - Optional pre-resized image data; if None, copies from `input_path`
///
/// # Returns
///
/// Returns `Ok(())` on successful optimization.
///
/// # Errors
///
/// Returns an error if:
/// - Invalid optimization level is provided (not 0-6 or `"max"`)
/// - PNG optimization fails
/// - File I/O operations fail (copying or saving)
/// - Image format conversion fails
pub fn optimize_png(
    input_path: &Path,
    output_path: &Path,
    args: &Cli,
    resized_img: Option<DynamicImage>,
) -> Result<()> {
    if let Some(img) = resized_img {
        img.save_with_format(output_path, ImageFormat::Png)?;
    } else {
        fs::copy(input_path, output_path)?;
    }

    let optimization_level = if args.png_optimization_level == "max" {
        6
    } else {
        match args.png_optimization_level.parse::<u8>() {
            Ok(level) if level <= 6 => level,
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid oxipng optimization level: {}. Valid values are 0-6 or 'max'",
                    args.png_optimization_level
                ));
            }
        }
    };

    let mut options = oxipng::Options::from_preset(optimization_level);
    options.optimize_alpha = true;
    options.fast_evaluation = true;
    options.strip = oxipng::StripChunks::Safe;

    if args.no_zopfli {
        options.deflate = oxipng::Deflaters::Libdeflater { compression: 12 };
    } else {
        options.deflate = oxipng::Deflaters::Zopfli {
            iterations: args.zopfli_iterations,
        };
    }

    let input_file = oxipng::InFile::Path(output_path.to_path_buf());
    let output_file = oxipng::OutFile::Path {
        path: Some(output_path.to_path_buf()),
        preserve_attrs: true,
    };

    oxipng::optimize(&input_file, &output_file, &options).context("Failed to optimize PNG")?;

    Ok(())
}
