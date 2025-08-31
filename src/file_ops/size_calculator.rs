/// Calculates new image dimensions while preserving aspect ratio.
///
/// This function determines the optimal dimensions for resizing an image to fit within
/// a maximum size constraint while maintaining the original aspect ratio. The resize
/// is based on the longer edge of the image.
///
/// # Arguments
///
/// * `width` - Original image width in pixels
/// * `height` - Original image height in pixels  
/// * `max_size` - Maximum allowed size for the longer edge in pixels
///
/// # Returns
///
/// A tuple `(new_width, new_height)` containing the calculated dimensions.
/// If no resizing is needed (longer edge ≤ max_size), returns the original dimensions.
///
/// # Algorithm
///
/// 1. Determines the longer edge between width and height
/// 2. If longer edge ≤ max_size, no resizing is needed
/// 3. Otherwise, calculates a scaling factor to fit the longer edge to max_size
/// 4. Applies the scaling factor to both dimensions and rounds to nearest integer
///
/// # Examples
///
/// ```rust
/// use image_optimizer::file_ops::calculate_resize_dimensions;
///
/// // No resize needed
/// assert_eq!(calculate_resize_dimensions(800, 600, 1000), (800, 600));
///
/// // Width is longer edge, needs resize
/// assert_eq!(calculate_resize_dimensions(1200, 800, 600), (600, 400));
///
/// // Height is longer edge, needs resize  
/// assert_eq!(calculate_resize_dimensions(800, 1200, 600), (400, 600));
///
/// // Square image resize
/// assert_eq!(calculate_resize_dimensions(1000, 1000, 500), (500, 500));
/// ```
#[must_use]
pub fn calculate_resize_dimensions(width: u32, height: u32, max_size: u32) -> (u32, u32) {
    let longer_edge = width.max(height);

    if longer_edge <= max_size {
        return (width, height);
    }

    let scale_factor = f64::from(max_size) / f64::from(longer_edge);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let new_width = (f64::from(width) * scale_factor).round() as u32;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let new_height = (f64::from(height) * scale_factor).round() as u32;

    (new_width, new_height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_resize_needed() {
        assert_eq!(calculate_resize_dimensions(800, 600, 1000), (800, 600));
        assert_eq!(calculate_resize_dimensions(100, 100, 200), (100, 100));
        assert_eq!(calculate_resize_dimensions(500, 300, 500), (500, 300));
    }

    #[test]
    fn test_width_longer_resize() {
        assert_eq!(calculate_resize_dimensions(1200, 800, 600), (600, 400));
        assert_eq!(calculate_resize_dimensions(2000, 1000, 800), (800, 400));
    }

    #[test]
    fn test_height_longer_resize() {
        assert_eq!(calculate_resize_dimensions(800, 1200, 600), (400, 600));
        assert_eq!(calculate_resize_dimensions(1000, 2000, 800), (400, 800));
    }

    #[test]
    fn test_square_image_resize() {
        assert_eq!(calculate_resize_dimensions(1000, 1000, 500), (500, 500));
        assert_eq!(calculate_resize_dimensions(2000, 2000, 800), (800, 800));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(calculate_resize_dimensions(1, 1, 100), (1, 1));
        assert_eq!(calculate_resize_dimensions(u32::MAX, 100, 50), (50, 0));
    }

    #[test]
    fn test_rounding() {
        assert_eq!(calculate_resize_dimensions(1333, 1000, 800), (800, 600));
        assert_eq!(calculate_resize_dimensions(1001, 1000, 800), (800, 799));
    }
}
