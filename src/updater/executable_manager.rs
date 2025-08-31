use anyhow::Result;
use std::path::PathBuf;

/// Gets the path to the current executable
///
/// # Errors
/// Returns an error if the current executable path cannot be determined
pub fn get_current_executable() -> Result<PathBuf> {
    std::env::current_exe()
        .map_err(|e| anyhow::anyhow!("Failed to get current executable path: {}", e))
}
