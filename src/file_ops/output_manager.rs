use anyhow::Result;
use std::path::{Path, PathBuf};

/// Ensures the output directory structure exists and returns the output file path.
///
/// This function creates the necessary directory structure in the output directory
/// to mirror the input directory structure, then returns the full path where the
/// optimized file should be written.
///
/// # Arguments
///
/// * `output_path` - Base output directory path
/// * `input_path` - Base input directory path (used to calculate relative paths)
/// * `file_path` - Path to the specific file being processed
///
/// # Returns
///
/// Returns the full `PathBuf` where the optimized file should be written.
///
/// # Errors
///
/// Returns an error if:
/// - Path stripping fails (`file_path` is not under `input_path`)
/// - Directory creation fails due to permissions or I/O errors
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use image_optimizer::file_ops::ensure_output_dir;
///
/// # fn example() -> anyhow::Result<()> {
/// let output_dir = Path::new("./optimized");
/// let input_dir = Path::new("./photos");
/// let file_path = Path::new("./photos/subfolder/image.jpg");
///
/// let output_file = ensure_output_dir(output_dir, input_dir, file_path)?;
/// // Returns: "./optimized/subfolder/image.jpg"
/// # Ok(())
/// # }
/// ```
pub fn ensure_output_dir(
    output_path: &Path,
    input_path: &Path,
    file_path: &Path,
) -> Result<PathBuf> {
    let relative_path = file_path.strip_prefix(input_path)?;
    let output_file_path = output_path.join(relative_path);

    if let Some(parent) = output_file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    Ok(output_file_path)
}
