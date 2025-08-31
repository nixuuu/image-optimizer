//! Self-update functionality.
//!
//! This module provides automatic update capabilities for the image optimizer tool.
//! It can check for new releases on GitHub, download the appropriate binary for the
//! current platform, and perform in-place updates with backup creation.
//!
//! ## Components
//!
//! - **Self updater**: Main update orchestration and GitHub API interaction
//! - **GitHub release**: Data structures for GitHub API responses
//! - **Platform detector**: Determines the correct binary target for the current platform
//! - **Version comparator**: Semantic version comparison logic
//! - **Executable manager**: Current executable path management

pub mod executable_manager;
pub mod github_release;
pub mod platform_detector;
#[allow(clippy::module_name_repetitions)]
pub mod self_updater;
pub mod version_comparator;

pub use self_updater::update_self;
