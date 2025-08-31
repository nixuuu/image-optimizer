pub mod executable_manager;
pub mod github_release;
pub mod platform_detector;
pub mod self_updater;
pub mod version_comparator;

pub use self_updater::update_self;
