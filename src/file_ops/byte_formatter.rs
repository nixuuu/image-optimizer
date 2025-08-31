/// Formats a byte count into a human-readable string with appropriate units.
///
/// This function converts raw byte counts into a more readable format using
/// standard binary units (B, KB, MB, GB). It automatically selects the most
/// appropriate unit and formats the output with appropriate decimal precision.
///
/// # Arguments
///
/// * `bytes` - The number of bytes to format
///
/// # Returns
///
/// A formatted string representation of the byte count with units.
/// - Bytes are shown as whole numbers (e.g., "512 B")
/// - Larger units are shown with one decimal place (e.g., "1.5 KB")
///
/// # Examples
///
/// ```rust
/// use image_optimizer::file_ops::format_bytes;
///
/// assert_eq!(format_bytes(512), "512 B");
/// assert_eq!(format_bytes(1024), "1.0 KB");
/// assert_eq!(format_bytes(1536), "1.5 KB");
/// assert_eq!(format_bytes(1048576), "1.0 MB");
/// ```
#[must_use]
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    #[allow(clippy::cast_precision_loss)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1023), "1023 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.0 GB");
        assert_eq!(format_bytes(2147483648), "2.0 GB");
    }

    #[test]
    fn test_format_bytes_edge_cases() {
        assert_eq!(format_bytes(u64::MAX), "17179869184.0 GB");
        assert_eq!(format_bytes(1), "1 B");
        assert_eq!(format_bytes(1025), "1.0 KB");
    }
}
