//! # Image Optimizer
//!
//! A high-performance CLI tool for optimizing images written in Rust.
//!
//! This crate provides image optimization capabilities for JPEG, PNG, WebP, and SVG formats
//! with support for parallel processing, quality control, resizing, and backup creation.
//!
//! ## Features
//!
//! - **Multi-format support**: JPEG (mozjpeg), PNG (oxipng with zopfli), WebP, SVG (regex-based)
//! - **Parallel processing**: Concurrent optimization using rayon
//! - **Flexible output**: In-place optimization or separate output directory
//! - **Quality control**: Adjustable quality settings and lossless mode (raster formats only)
//! - **Image resizing**: Optional resizing with `--max-size` parameter (raster formats only)
//! - **Backup support**: Create backup files before optimization
//! - **Progress tracking**: Real-time progress bar with file-by-file status
//! - **Self-updating**: Built-in update mechanism via GitHub releases
//!
//! ## Architecture
//!
//! The crate is organized into distinct modules following a one-function-per-file pattern:
//!
//! - [`cli`] - Command-line interface components
//! - [`file_ops`] - File system operations and utilities
//! - [`optimization`] - Image optimization functionality
//! - [`updater`] - Self-update functionality
//!
//! ## Usage
//!
//! ```bash
//! # Optimize all images in a directory recursively
//! image-optimizer -i ./images -r
//!
//! # Optimize with custom quality and backup
//! image-optimizer -i input_dir -o output_dir --quality 90 --backup
//!
//! # Resize and optimize with lossless compression
//! image-optimizer -i images --max-size 1024 --lossless
//! ```

pub mod cli;
pub mod file_ops;
pub mod optimization;
pub mod updater;
