use std::fs;
use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CLI tool for optimizing images"));
    assert!(stdout.contains("--input"));
    assert!(stdout.contains("--output"));
    assert!(stdout.contains("--jpeg-quality"));
}

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("image-optimizer"));
}

#[test]
fn test_invalid_input_path() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-i", "/nonexistent/path"])
        .output()
        .expect("Failed to execute command");

    // Should fail gracefully with proper error message
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("does not exist") || stderr.contains("No such file"));
}

#[test]
fn test_empty_directory() {
    let temp_dir = std::env::temp_dir().join("test_empty_dir");
    fs::create_dir_all(&temp_dir).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-i", temp_dir.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    fs::remove_dir_all(&temp_dir).unwrap();
}

#[cfg(test)]
mod helper_tests {
    use image_optimizer::file_ops::{calculate_resize_dimensions, format_bytes};

    #[test]
    fn test_byte_formatter_integration() {
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(0), "0 B");
    }

    #[test]
    fn test_size_calculator_integration() {
        assert_eq!(calculate_resize_dimensions(1000, 800, 600), (600, 480));
        assert_eq!(calculate_resize_dimensions(500, 300, 1000), (500, 300));
    }
}
