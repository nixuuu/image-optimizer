use anyhow::Result;
use std::path::{Path, PathBuf};

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