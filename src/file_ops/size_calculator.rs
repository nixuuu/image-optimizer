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