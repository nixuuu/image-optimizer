use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat};
use std::fs;
use std::path::Path;

use crate::cli::Cli;

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