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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_supported_extensions() {
        assert!(SUPPORTED_EXTENSIONS.contains(&"jpg"));
        assert!(SUPPORTED_EXTENSIONS.contains(&"jpeg"));
        assert!(SUPPORTED_EXTENSIONS.contains(&"png"));
        assert!(SUPPORTED_EXTENSIONS.contains(&"webp"));
        assert!(!SUPPORTED_EXTENSIONS.contains(&"gif"));
        assert!(!SUPPORTED_EXTENSIONS.contains(&"txt"));
    }

    #[test]
    fn test_scan_single_file() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test.jpg");
        fs::write(&test_file, "fake jpg content").unwrap();

        let result = scan_images(&test_file, false);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], test_file);

        fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_scan_unsupported_file() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test.txt");
        fs::write(&test_file, "text content").unwrap();

        let result = scan_images(&test_file, false);
        assert_eq!(result.len(), 0);

        fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_scan_nonexistent_path() {
        let nonexistent = Path::new("/nonexistent/path");
        let result = scan_images(nonexistent, false);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_case_insensitive_extensions() {
        let temp_dir = std::env::temp_dir();
        let test_files = [
            ("test_upper.JPG", "jpg"),
            ("test_upper.JPEG", "jpeg"),
            ("test_upper.PNG", "png"),
            ("test_upper.WEBP", "webp"),
        ];

        for (filename, _) in &test_files {
            let test_file = temp_dir.join(filename);
            fs::write(&test_file, "fake content").unwrap();

            let result = scan_images(&test_file, false);
            assert_eq!(result.len(), 1, "Failed for file: {}", filename);
            assert_eq!(result[0], test_file);

            fs::remove_file(&test_file).unwrap();
        }
    }
}
