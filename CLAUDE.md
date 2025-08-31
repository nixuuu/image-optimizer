# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A CLI image optimizer written in Rust that supports JPEG, PNG, and WebP formats. The tool can optimize images in-place or to a separate output directory, with options for resizing, quality adjustment, and creating backups.

## Architecture

**Code Organization Rule: One function/struct/trait per file**

The codebase follows a strict modular architecture where each function, struct, or trait is defined in its own file. Modules are grouped by functionality:

- `src/main.rs` - Main application entry point with parallel processing coordination and progress tracking
- `src/cli/` - Command-line interface components
  - `cli_args.rs` - Cli struct definition
- `src/optimization/` - Image optimization functionality
  - `image_optimizer.rs` - Main optimization orchestration function
  - `jpeg_optimizer.rs` - JPEG-specific optimization using mozjpeg
  - `png_optimizer.rs` - PNG optimization using oxipng with zopfli
  - `webp_optimizer.rs` - WebP optimization functionality
- `src/file_ops/` - File system operations and utilities
  - `image_scanner.rs` - Directory scanning for image files
  - `output_manager.rs` - Output directory management
  - `backup_manager.rs` - Backup file creation
  - `size_calculator.rs` - Image resize dimension calculations
  - `byte_formatter.rs` - Human-readable byte size formatting
- `src/updater/` - Self-update functionality
  - `self_updater.rs` - Main update orchestration function
  - `github_release.rs` - GitHub release data structures
  - `platform_detector.rs` - Platform target detection
  - `version_comparator.rs` - Version comparison logic
  - `executable_manager.rs` - Current executable path management

The application uses parallel processing via rayon to optimize multiple images concurrently, with a progress bar showing real-time status.

## Common Commands

### Build and Development
```bash
cargo build          # Build the project
cargo build --release  # Build optimized release version
cargo check          # Check compilation without building
cargo clippy         # Run linting
cargo test           # Run tests
```

### Running the Application
```bash
cargo run -- --help                    # Show usage help
cargo run -- -i ./test/images -r       # Optimize all images recursively in test/images
cargo run -- -i input_dir -o output_dir --quality 90  # Optimize with custom quality to output dir
cargo run -- -i images --backup --lossless  # Create backups and use lossless compression
```

## Key Features

- Supports JPEG (mozjpeg), PNG (oxipng with zopfli), and WebP optimization
- Optional image resizing with `--max-size` parameter
- In-place optimization or separate output directory
- Backup creation with `--backup` flag
- Quality control (1-100) and lossless mode
- Parallel processing for batch operations
- Progress tracking with file-by-file status

## Test Data

The `test/images/` directory contains hundreds of sample JPEG files for testing the optimizer with real image data.

## Code Quality Requirements

After making any code changes, always run these commands in sequence to ensure code quality:

1. **Format Check**: Run `cargo fmt --check` to verify formatting
   - If it fails, run `cargo fmt` to apply automatic formatting
2. **Linting**: Run `cargo lint` to check for code issues
   - If it fails, run `cargo fix-lint` to apply automatic fixes, then run `cargo lint` again
3. **Final Verification**: Ensure all formatting and linting passes without errors or warnings

This workflow ensures consistent code style and catches potential issues before committing changes.