use anyhow::Result;
use std::path::PathBuf;

pub fn get_current_executable() -> Result<PathBuf> {
    std::env::current_exe()
        .map_err(|e| anyhow::anyhow!("Failed to get current executable path: {}", e))
}