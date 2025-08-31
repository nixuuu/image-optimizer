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
#[command(about = "CLI tool for optimizing images (JPEG, PNG, WebP)")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_defaults() {
        let cli = Cli::parse_from(&["image-optimizer"]);
        assert_eq!(cli.input, None);
        assert_eq!(cli.output, None);
        assert!(!cli.backup);
        assert!(!cli.lossless);
        assert_eq!(cli.quality, 85);
        assert!(!cli.recursive);
        assert_eq!(cli.max_size, None);
        assert!(!cli.update);
    }

    #[test]
    fn test_cli_with_input() {
        let cli = Cli::parse_from(&["image-optimizer", "-i", "/path/to/images"]);
        assert_eq!(cli.input, Some(PathBuf::from("/path/to/images")));
    }

    #[test]
    fn test_cli_with_all_flags() {
        let cli = Cli::parse_from(&[
            "image-optimizer",
            "-i",
            "/input",
            "-o",
            "/output",
            "--backup",
            "--lossless",
            "-q",
            "90",
            "--recursive",
            "--max-size",
            "1024",
            "--update",
        ]);

        assert_eq!(cli.input, Some(PathBuf::from("/input")));
        assert_eq!(cli.output, Some(PathBuf::from("/output")));
        assert!(cli.backup);
        assert!(cli.lossless);
        assert_eq!(cli.quality, 90);
        assert!(cli.recursive);
        assert_eq!(cli.max_size, Some(1024));
        assert!(cli.update);
    }

    #[test]
    fn test_cli_quality_bounds() {
        let cli = Cli::parse_from(&["image-optimizer", "-q", "1"]);
        assert_eq!(cli.quality, 1);

        let cli = Cli::parse_from(&["image-optimizer", "-q", "100"]);
        assert_eq!(cli.quality, 100);
    }

    #[test]
    fn test_cli_help_generation() {
        let mut cmd = Cli::command();
        let help = cmd.render_help();
        assert!(help.to_string().contains("CLI tool for optimizing images"));
    }
}
