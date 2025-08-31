pub mod github_release;
pub mod platform_detector;
pub mod version_comparator;
pub mod executable_manager;
pub mod self_updater;

pub use self_updater::update_self;