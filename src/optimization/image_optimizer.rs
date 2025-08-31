use anyhow::Result;
use image;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use super::{jpeg_optimizer, png_optimizer, webp_optimizer};
use crate::cli::Cli;
use crate::file_ops::{calculate_resize_dimensions, create_backup, ensure_output_dir};

/// Optimizes an image file using the appropriate format-specific optimizer
///
/// # Errors
/// Returns an error if file I/O operations fail, image processing fails, or unsupported format
pub fn optimize_image(input_path: &Path, args: &Cli, input_dir: &Path) -> Result<u64> {
    let original_size = fs::metadata(input_path)?.len();

    let is_in_place = args.output.is_none();
    let output_path = if let Some(ref output_dir) = args.output {
        ensure_output_dir(output_dir, input_dir, input_path)?
    } else {
        input_path.with_extension(format!(
            "tmp.{}",
            input_path
                .extension()
                .and_then(OsStr::to_str)
                .unwrap_or("jpg")
        ))
    };

    if args.backup && is_in_place {
        create_backup(input_path)?;
    }

    let extension = input_path
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
        .to_lowercase();

    let img = if args.max_size.is_some() {
        let img = image::open(input_path)?;
        let (width, height) = (img.width(), img.height());

        if let Some(max_size) = args.max_size {
            let (new_width, new_height) = calculate_resize_dimensions(width, height, max_size);
            if new_width != width || new_height != height {
                Some(img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3))
            } else {
                Some(img)
            }
        } else {
            Some(img)
        }
    } else {
        None
    };

    match extension.as_str() {
        "jpg" | "jpeg" => jpeg_optimizer::optimize_jpeg(input_path, &output_path, args, img)?,
        "png" => png_optimizer::optimize_png(input_path, &output_path, args, img)?,
        "webp" => webp_optimizer::optimize_webp(input_path, &output_path, args, img)?,
        _ => return Err(anyhow::anyhow!("Unsupported file format: {}", extension)),
    }

    let optimized_size = fs::metadata(&output_path)?.len();

    if optimized_size < original_size {
        if is_in_place {
            fs::rename(&output_path, input_path)?;
        }
        Ok(original_size - optimized_size)
    } else {
        if is_in_place {
            fs::remove_file(&output_path)?;
        } else {
            fs::copy(input_path, &output_path)?;
        }
        Ok(0)
    }
}
