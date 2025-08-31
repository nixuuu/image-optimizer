use anyhow::Result;
use std::ffi::OsStr;
use std::path::Path;

pub fn create_backup(file_path: &Path) -> Result<()> {
    let backup_path = file_path.with_extension(format!(
        "{}.bak",
        file_path.extension().and_then(OsStr::to_str).unwrap_or("")
    ));
    std::fs::copy(file_path, backup_path)?;
    Ok(())
}
