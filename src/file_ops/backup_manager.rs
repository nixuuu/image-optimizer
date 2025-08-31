use anyhow::Result;
use std::ffi::OsStr;
use std::path::Path;

/// Creates a backup file by copying the original with a .bak extension.
///
/// This function creates a safety backup of the original file before optimization
/// by copying it to a new file with the same name but with `.bak` appended to the extension.
/// For example, `image.jpg` becomes `image.jpg.bak`.
///
/// # Arguments
///
/// * `file_path` - Path to the file to backup
///
/// # Returns
///
/// Returns `Ok(())` on successful backup creation.
///
/// # Errors
///
/// Returns an error if the file copy operation fails due to:
/// - Insufficient disk space
/// - Permission issues
/// - I/O errors during file copying
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use image_optimizer::file_ops::create_backup;
///
/// # fn example() -> anyhow::Result<()> {
/// let file_path = Path::new("image.jpg");
/// create_backup(file_path)?; // Creates image.jpg.bak
/// # Ok(())
/// # }
/// ```
pub fn create_backup(file_path: &Path) -> Result<()> {
    let backup_path = file_path.with_extension(format!(
        "{}.bak",
        file_path.extension().and_then(OsStr::to_str).unwrap_or("")
    ));
    std::fs::copy(file_path, backup_path)?;
    Ok(())
}
