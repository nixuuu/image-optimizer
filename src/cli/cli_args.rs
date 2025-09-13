use clap::Parser;
use std::path::PathBuf;

/// Command-line interface configuration for the image optimizer tool.
///
/// This struct defines all available command-line arguments and flags for the image optimization
/// tool. It uses the `clap` crate for parsing and validation of command-line arguments.
///
/// ## Examples
///
/// ```rust
/// use image_optimizer::cli::Cli;
/// use clap::Parser;
///
/// // Parse CLI arguments
/// let cli = Cli::parse();
/// ```
#[derive(Parser)]
#[command(name = "image-optimizer")]
#[command(about = "CLI tool for optimizing images (JPEG, PNG, WebP, SVG)")]
#[command(long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    /// Input directory or file to process
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
    pub webp_lossless: bool,

    /// JPEG quality (1-100), ignored if lossless is set (applies to raster formats only)
    #[arg(long, default_value = "85")]
    pub jpeg_quality: u8,

    /// Recursively scan subdirectories
    #[arg(short, long)]
    pub recursive: bool,

    /// Maximum size for the longer edge (resizes if larger, applies to raster formats only)
    #[arg(long)]
    pub max_size: Option<u32>,

    /// Oxipng optimization level (0-6 or max)
    #[arg(long, default_value = "2")]
    pub png_optimization_level: String,

    /// Zopfli iterations for optimization (1-255)
    #[arg(long, default_value = "15")]
    pub zopfli_iterations: std::num::NonZeroU8,

    #[arg(long, default_value = "false")]
    pub no_zopfli: bool,

    #[arg(long, default_value = "false")]
    pub no_parallel: bool,

    /// Update to the latest version
    #[arg(long)]
    pub update: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_defaults() {
        let cli = Cli::parse_from(["image-optimizer"]);
        assert_eq!(cli.input, None);
        assert_eq!(cli.output, None);
        assert!(!cli.backup);
        assert!(!cli.webp_lossless);
        assert_eq!(cli.jpeg_quality, 85);
        assert!(!cli.recursive);
        assert_eq!(cli.max_size, None);
        assert_eq!(cli.png_optimization_level, "2");
        assert_eq!(cli.zopfli_iterations.get(), 15);
        assert!(!cli.update);
    }

    #[test]
    fn test_cli_with_input() {
        let cli = Cli::parse_from(["image-optimizer", "-i", "/path/to/images"]);
        assert_eq!(cli.input, Some(PathBuf::from("/path/to/images")));
    }

    #[test]
    fn test_cli_with_all_flags() {
        let cli = Cli::parse_from([
            "image-optimizer",
            "-i",
            "/input",
            "-o",
            "/output",
            "--backup",
            "--webp-lossless",
            "--jpeg-quality",
            "90",
            "--recursive",
            "--max-size",
            "1024",
            "--png-optimization-level",
            "max",
            "--zopfli-iterations",
            "25",
            "--update",
        ]);

        assert_eq!(cli.input, Some(PathBuf::from("/input")));
        assert_eq!(cli.output, Some(PathBuf::from("/output")));
        assert!(cli.backup);
        assert!(cli.webp_lossless);
        assert_eq!(cli.jpeg_quality, 90);
        assert!(cli.recursive);
        assert_eq!(cli.max_size, Some(1024));
        assert_eq!(cli.png_optimization_level, "max");
        assert_eq!(cli.zopfli_iterations.get(), 25);
        assert!(cli.update);
        assert!(!cli.no_zopfli);
        assert!(!cli.no_parallel);
    }

    #[test]
    fn test_cli_quality_bounds() {
        let cli = Cli::parse_from(["image-optimizer", "--jpeg-quality", "1"]);
        assert_eq!(cli.jpeg_quality, 1);

        let cli = Cli::parse_from(["image-optimizer", "--jpeg-quality", "100"]);
        assert_eq!(cli.jpeg_quality, 100);
    }

    #[test]
    fn test_cli_help_generation() {
        let mut cmd = Cli::command();
        let help = cmd.render_help();
        assert!(help.to_string().contains("CLI tool for optimizing images"));
    }
}
