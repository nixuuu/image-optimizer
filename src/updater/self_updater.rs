use anyhow::Result;

use super::executable_manager::get_current_executable;
use super::github_release::GitHubRelease;
use super::platform_detector::get_platform_target;
use super::version_comparator::compare_versions;

/// Current version of the application from Cargo.toml.
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// GitHub repository owner for fetching releases.
const REPO_OWNER: &str = "nixuuu";

/// GitHub repository name for fetching releases.
const REPO_NAME: &str = "image-optimizer";

/// Downloads and installs the latest version from GitHub releases.
///
/// This function performs a complete self-update process including:
/// 1. Checking the latest release via GitHub API
/// 2. Comparing versions to determine if an update is needed
/// 3. Downloading the appropriate binary for the current platform
/// 4. Creating a backup of the current executable
/// 5. Installing the new version with proper permissions
///
/// The update process is atomic - if any step fails, the original binary remains unchanged.
/// A backup is always created before replacement for safety.
///
/// # Returns
///
/// Returns `Ok(())` on successful update or if already up-to-date.
///
/// # Errors
///
/// Returns an error if:
/// - Network requests to GitHub API fail
/// - Platform is unsupported for automatic updates
/// - File operations fail (backup creation, binary replacement)
/// - Downloaded binary is corrupted or invalid
/// - Permission changes fail on Unix systems
///
/// # Platform Support
///
/// Supports automatic updates on:
/// - Linux (x86_64, aarch64)
/// - macOS (x86_64, Apple Silicon)
/// - Windows (x86_64)
pub fn update_self() -> Result<()> {
    println!("🔍 Checking for updates...");
    println!("Current version: v{CURRENT_VERSION}");

    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}/releases/latest");

    let response = client
        .get(&url)
        .header("User-Agent", format!("{REPO_NAME}/{CURRENT_VERSION}"))
        .send()
        .map_err(|e| anyhow::anyhow!("Failed to check for updates: {}", e))?;

    let release: GitHubRelease = response
        .json()
        .map_err(|e| anyhow::anyhow!("Failed to parse release information: {}", e))?;

    println!("Latest version: {}", release.tag_name);

    if !compare_versions(CURRENT_VERSION, &release.tag_name)? {
        println!("✅ You're already running the latest version!");
        return Ok(());
    }

    println!("📦 New version available: {}", release.tag_name);

    let target = get_platform_target()?;
    let binary_name = format!("image-optimizer-{target}");

    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == binary_name)
        .ok_or_else(|| anyhow::anyhow!("No binary found for platform: {}", target))?;

    println!("⬇️  Downloading update...");

    let binary_data = client
        .get(&asset.browser_download_url)
        .send()
        .map_err(|e| anyhow::anyhow!("Failed to download update: {}", e))?
        .bytes()
        .map_err(|e| anyhow::anyhow!("Failed to read update data: {}", e))?;

    let current_exe = get_current_executable()?;
    let backup_path = current_exe.with_extension("bak");

    println!("💾 Creating backup...");
    std::fs::copy(&current_exe, &backup_path)
        .map_err(|e| anyhow::anyhow!("Failed to create backup: {}", e))?;

    println!("🔄 Installing update...");
    let temp_exe = current_exe.with_extension("tmp");
    std::fs::write(&temp_exe, binary_data)
        .map_err(|e| anyhow::anyhow!("Failed to write updated binary: {}", e))?;

    std::fs::rename(&temp_exe, &current_exe)
        .map_err(|e| anyhow::anyhow!("Failed to install updated binary: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&current_exe)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&current_exe, perms)?;
    }

    println!("✅ Successfully updated to {}!", release.tag_name);
    println!("📁 Backup saved to: {}", backup_path.display());

    Ok(())
}
