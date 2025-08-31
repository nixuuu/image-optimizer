use anyhow::Result;
use std::path::PathBuf;

/// Gets the path to the current executable file.
///
/// This function retrieves the filesystem path to the currently running executable,
/// which is needed for the self-update process to know which file to replace.
/// It wraps Rust's `std::env::current_exe()` with proper error handling.
///
/// # Returns
///
/// Returns the `PathBuf` to the current executable file.
///
/// # Errors
///
/// Returns an error if the current executable path cannot be determined, which may
/// happen in some restricted execution environments or if the executable has been
/// deleted after startup.
///
/// # Examples
///
/// ```rust
/// use image_optimizer::updater::get_current_executable;
///
/// # fn example() -> anyhow::Result<()> {
/// let exe_path = get_current_executable()?;
/// println!("Current executable: {}", exe_path.display());
/// # Ok(())
/// # }
/// ```
pub fn get_current_executable() -> Result<PathBuf> {
    std::env::current_exe()
        .map_err(|e| anyhow::anyhow!("Failed to get current executable path: {}", e))
}
