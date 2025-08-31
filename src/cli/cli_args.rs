use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "image-optimizer-rs")]
#[command(about = "CLI tool for optimizing images (JPEG, PNG, WebP)")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    /// Input directory to scan for images
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Output directory (if not specified, optimizes in place)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Create backup files (.bak)
    #[arg(long)]
    pub backup: bool,

    /// Use lossless compression
    #[arg(long)]
    pub lossless: bool,

    /// JPEG quality (1-100), ignored if lossless is set
    #[arg(short, long, default_value = "85")]
    pub quality: u8,

    /// Recursively scan subdirectories
    #[arg(short, long)]
    pub recursive: bool,

    /// Maximum size for the longer edge (resizes if larger)
    #[arg(long)]
    pub max_size: Option<u32>,

    /// Update to the latest version
    #[arg(long)]
    pub update: bool,
}
