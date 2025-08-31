use anyhow::Result;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp"];

pub fn scan_images(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut image_files = Vec::new();
    
    let walker = if recursive {
        WalkDir::new(dir)
    } else {
        WalkDir::new(dir).max_depth(1)
    };

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension().and_then(OsStr::to_str) {
                if SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
                    image_files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    Ok(image_files)
}

pub fn ensure_output_dir(output_path: &Path, input_path: &Path, file_path: &Path) -> Result<PathBuf> {
    let relative_path = file_path.strip_prefix(input_path)?;
    let output_file_path = output_path.join(relative_path);
    
    if let Some(parent) = output_file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    Ok(output_file_path)
}

pub fn create_backup(file_path: &Path) -> Result<()> {
    let backup_path = file_path.with_extension(
        format!("{}.bak", 
            file_path.extension()
                .and_then(OsStr::to_str)
                .unwrap_or("")
        )
    );
    std::fs::copy(file_path, backup_path)?;
    Ok(())
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn calculate_resize_dimensions(width: u32, height: u32, max_size: u32) -> (u32, u32) {
    let longer_edge = width.max(height);
    
    if longer_edge <= max_size {
        return (width, height);
    }
    
    let scale_factor = max_size as f64 / longer_edge as f64;
    let new_width = (width as f64 * scale_factor).round() as u32;
    let new_height = (height as f64 * scale_factor).round() as u32;
    
    (new_width, new_height)
}