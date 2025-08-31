use anyhow::{Context, Result};
use image::{ImageFormat, DynamicImage};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::{utils, Cli};

pub fn optimize_image(input_path: &Path, args: &Cli, input_dir: &Path) -> Result<u64> {
    let original_size = fs::metadata(input_path)?.len();
    
    let is_in_place = args.output.is_none();
    let output_path = if let Some(ref output_dir) = args.output {
        utils::ensure_output_dir(output_dir, input_dir, input_path)?
    } else {
        let temp_path = input_path.with_extension(format!("tmp.{}", 
            input_path.extension().and_then(OsStr::to_str).unwrap_or("jpg")));
        temp_path
    };

    if args.backup && is_in_place {
        utils::create_backup(input_path)?;
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
            let (new_width, new_height) = utils::calculate_resize_dimensions(width, height, max_size);
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
        "jpg" | "jpeg" => optimize_jpeg(input_path, &output_path, args, img)?,
        "png" => optimize_png(input_path, &output_path, args, img)?,
        "webp" => optimize_webp(input_path, &output_path, args, img)?,
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

fn optimize_jpeg(input_path: &Path, output_path: &Path, args: &Cli, resized_img: Option<DynamicImage>) -> Result<()> {
    let quality = if args.lossless { 100 } else { args.quality };
    
    let (width, height, rgb_data) = if let Some(img) = resized_img {
        let rgb_img = img.to_rgb8();
        (rgb_img.width(), rgb_img.height(), rgb_img.into_raw())
    } else {
        let input_data = fs::read(input_path)?;
        let decompress = mozjpeg::Decompress::new_mem(&input_data)?;
        let width = decompress.width() as u32;
        let height = decompress.height() as u32;
        let mut decompress_started = decompress.rgb()?;
        let rgb_data: Vec<u8> = decompress_started.read_scanlines()?;
        (width, height, rgb_data)
    };
    
    let mut compress = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    compress.set_quality(quality as f32);
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

fn optimize_png(input_path: &Path, output_path: &Path, args: &Cli, resized_img: Option<DynamicImage>) -> Result<()> {
    if let Some(img) = resized_img {
        img.save_with_format(output_path, ImageFormat::Png)?;
    } else {
        fs::copy(input_path, output_path)?;
    }
    
    let mut options = oxipng::Options::default();
    
    if args.lossless {
        options.optimize_alpha = true;
        options.strip = oxipng::StripChunks::Safe;
    } else {
        options.optimize_alpha = true;
        options.strip = oxipng::StripChunks::Safe;
    }
    
    let input_file = oxipng::InFile::Path(output_path.to_path_buf());
    let output_file = oxipng::OutFile::Path { 
        path: Some(output_path.to_path_buf()),
        preserve_attrs: true,
    };
    
    oxipng::optimize(&input_file, &output_file, &options)
        .context("Failed to optimize PNG")?;
    
    Ok(())
}

fn optimize_webp(input_path: &Path, output_path: &Path, args: &Cli, resized_img: Option<DynamicImage>) -> Result<()> {
    let rgb_img = if let Some(img) = resized_img {
        img.to_rgb8()
    } else {
        image::open(input_path)?.to_rgb8()
    };
    
    let encoder = if args.lossless {
        webp::Encoder::from_rgb(&rgb_img, rgb_img.width(), rgb_img.height())
            .encode_lossless()
    } else {
        webp::Encoder::from_rgb(&rgb_img, rgb_img.width(), rgb_img.height())
            .encode(args.quality as f32)
    };
    
    fs::write(output_path, &*encoder)?;
    
    Ok(())
}