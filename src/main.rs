use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod optimizer;
mod utils;

#[derive(Parser)]
#[command(name = "image-optimizer")]
#[command(about = "CLI tool for optimizing images (JPEG, PNG, WebP)")]
#[command(version = "0.1.0")]
struct Cli {
    /// Input directory to scan for images
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory (if not specified, optimizes in place)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Create backup files (.bak)
    #[arg(long)]
    backup: bool,

    /// Use lossless compression
    #[arg(long)]
    lossless: bool,

    /// JPEG quality (1-100), ignored if lossless is set
    #[arg(short, long, default_value = "85")]
    quality: u8,

    /// Recursively scan subdirectories
    #[arg(short, long)]
    recursive: bool,

    /// Maximum size for the longer edge (resizes if larger)
    #[arg(long)]
    max_size: Option<u32>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.quality > 100 {
        return Err(anyhow::anyhow!("Quality must be between 1 and 100"));
    }

    if !args.input.exists() {
        return Err(anyhow::anyhow!("Input directory does not exist"));
    }

    if !args.input.is_dir() {
        return Err(anyhow::anyhow!("Input path must be a directory"));
    }

    let image_files = utils::scan_images(&args.input, args.recursive)?;
    
    if image_files.is_empty() {
        println!("No image files found in the specified directory");
        return Ok(());
    }

    println!("Found {} image files", image_files.len());

    let pb = ProgressBar::new(image_files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")?
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    let total_saved = Arc::new(Mutex::new(0u64));
    let processed = Arc::new(Mutex::new(0usize));
    let skipped = Arc::new(Mutex::new(0usize));

    image_files.into_par_iter().for_each(|image_path| {
        pb.set_message(format!("Processing: {}", image_path.file_name().unwrap_or_default().to_string_lossy()));
        
        match optimizer::optimize_image(&image_path, &args) {
            Ok(saved_bytes) => {
                if saved_bytes > 0 {
                    *total_saved.lock().unwrap() += saved_bytes;
                    *processed.lock().unwrap() += 1;
                } else {
                    *skipped.lock().unwrap() += 1;
                }
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", image_path.display(), e);
            }
        }
        
        pb.inc(1);
    });

    let total_saved = *total_saved.lock().unwrap();
    let processed = *processed.lock().unwrap();
    let skipped = *skipped.lock().unwrap();

    pb.finish_with_message("Optimization complete");

    println!("\nProcessed {} files", processed);
    if skipped > 0 {
        println!("Skipped {} files (optimization would increase size)", skipped);
    }
    if total_saved > 0 {
        println!("Total space saved: {}", utils::format_bytes(total_saved));
    }

    Ok(())
}
