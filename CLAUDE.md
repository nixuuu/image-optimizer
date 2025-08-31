# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A CLI image optimizer written in Rust that supports JPEG, PNG, and WebP formats. The tool can optimize images in-place or to a separate output directory, with options for resizing, quality adjustment, and creating backups.

## Architecture

The codebase is organized into three main modules:

- `src/main.rs` - CLI argument parsing using clap, parallel processing coordination with rayon, and progress tracking with indicatif
- `src/optimizer.rs` - Core optimization logic with format-specific functions for JPEG (mozjpeg), PNG (oxipng), and WebP 
- `src/utils.rs` - Utility functions for file scanning, directory management, backup creation, and size calculations

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