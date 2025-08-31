//! Image optimization functionality.
//!
//! This module provides format-specific image optimization capabilities for JPEG, PNG, and WebP
//! formats. Each optimizer uses specialized libraries for maximum compression efficiency:
//!
//! - **JPEG**: Uses mozjpeg for superior compression compared to standard libjpeg
//! - **PNG**: Uses oxipng with zopfli for advanced compression algorithms
//! - **WebP**: Uses Google's WebP encoder with both lossy and lossless modes
//!
//! The main entry point [`optimize_image`] automatically selects the appropriate optimizer
//! based on file extension and coordinates the optimization process.

pub mod image_optimizer;
pub mod jpeg_optimizer;
pub mod png_optimizer;
pub mod webp_optimizer;

pub use image_optimizer::optimize_image;
