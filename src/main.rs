use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod cli;
mod file_ops;
mod optimization;
mod updater;

use cli::Cli;
use file_ops::{format_bytes, scan_images};
use optimization::optimize_image;
use updater::update_self;

/// Main entry point for the image optimizer CLI application.
///
/// This function coordinates the entire optimization process including:
/// - Command-line argument parsing
/// - Input validation
/// - Image file discovery
/// - Parallel optimization with progress tracking
/// - Results summary display
///
/// The application supports self-updating, batch processing with parallel execution,
/// and comprehensive error handling.
///
/// # Returns
///
/// Returns `Ok(())` on successful completion or an error if any critical operation fails.
///
/// # Errors
///
/// Returns an error if:
/// - Required input parameters are missing
/// - Input file or directory doesn't exist
/// - Quality parameter is out of range (1-100)
/// - Progress bar template formatting fails
/// - Any critical file I/O operations fail
fn main() -> Result<()> {
    let args = Cli::parse();

    if args.update {
        return update_self();
    }

    let input = args
        .input
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Input file or directory is required"))?;

    if args.jpeg_quality > 100 {
        return Err(anyhow::anyhow!("Quality must be between 1 and 100"));
    }

    if !input.exists() {
        return Err(anyhow::anyhow!("Input file or directory does not exist"));
    }

    let image_files = scan_images(input, args.recursive);

    if image_files.is_empty() {
        if input.is_file() {
            println!("The specified file is not a supported image format");
        } else {
            println!("No image files found in the specified directory");
        }
        return Ok(());
    }

    println!("Found {} image files", image_files.len());

    let pb = ProgressBar::new(image_files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
            )?
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    let total_saved = Arc::new(Mutex::new(0u64));
    let processed = Arc::new(Mutex::new(0usize));
    let skipped = Arc::new(Mutex::new(0usize));

    let image_processor = |image_path: PathBuf| {
        pb.set_message(format!(
            "Processing: {}",
            image_path.file_name().unwrap_or_default().to_string_lossy()
        ));

        match optimize_image(&image_path, &args, input) {
            Ok(saved_bytes) => {
                if saved_bytes > 0 {
                    if let Ok(mut saved) = total_saved.lock() {
                        *saved += saved_bytes;
                    }
                    if let Ok(mut proc) = processed.lock() {
                        *proc += 1;
                    }
                } else if let Ok(mut skip) = skipped.lock() {
                    *skip += 1;
                }
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", image_path.display(), e);
            }
        }

        pb.inc(1);
    };

    if args.no_parallel {
        image_files.into_iter().for_each(image_processor);
    } else {
        image_files.into_par_iter().for_each(image_processor);
    }

    let total_saved = total_saved.lock().map(|guard| *guard).unwrap_or(0);
    let processed = processed.lock().map(|guard| *guard).unwrap_or(0);
    let skipped = skipped.lock().map(|guard| *guard).unwrap_or(0);

    pb.finish_with_message("Optimization complete");

    println!("\nProcessed {processed} files");
    if skipped > 0 {
        println!("Skipped {skipped} files (optimization would increase size)");
    }
    if total_saved > 0 {
        println!("Total space saved: {}", format_bytes(total_saved));
    }

    Ok(())
}
