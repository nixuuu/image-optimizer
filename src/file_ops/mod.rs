pub mod backup_manager;
pub mod image_scanner;
pub mod output_manager;
pub mod size_calculator;
pub mod byte_formatter;

pub use backup_manager::create_backup;
pub use image_scanner::scan_images;
pub use output_manager::ensure_output_dir;
pub use size_calculator::calculate_resize_dimensions;
pub use byte_formatter::format_bytes;