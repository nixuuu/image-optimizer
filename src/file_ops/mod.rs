//! File system operations and utilities.
//!
//! This module provides file system utilities for the image optimizer including:
//!
//! - **Image scanning**: Discovering image files in directories with extension filtering
//! - **Backup management**: Creating backup copies of original files
//! - **Output management**: Ensuring output directory structure exists
//! - **Size calculations**: Computing resize dimensions while preserving aspect ratio
//! - **Byte formatting**: Converting byte counts to human-readable format

pub mod backup_manager;
pub mod byte_formatter;
pub mod image_scanner;
pub mod output_manager;
pub mod size_calculator;

pub use backup_manager::create_backup;
pub use byte_formatter::format_bytes;
pub use image_scanner::scan_images;
pub use output_manager::ensure_output_dir;
pub use size_calculator::calculate_resize_dimensions;
