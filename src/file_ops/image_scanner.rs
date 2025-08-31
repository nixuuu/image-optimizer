use std::ffi::OsStr;
use std::path::PathBuf;
use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp"];

pub fn scan_images(path: &std::path::Path, recursive: bool) -> Vec<PathBuf> {
    let mut image_files = Vec::new();

    if path.is_file() {
        if let Some(extension) = path.extension().and_then(OsStr::to_str)
            && SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str())
        {
            image_files.push(path.to_path_buf());
        }
        return image_files;
    }

    let walker = if recursive {
        WalkDir::new(path)
    } else {
        WalkDir::new(path).max_depth(1)
    };

    for entry in walker.into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file()
            && let Some(extension) = entry.path().extension().and_then(OsStr::to_str)
            && SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str())
        {
            image_files.push(entry.path().to_path_buf());
        }
    }

    image_files
}
