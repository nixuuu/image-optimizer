use anyhow::Result;
use serde::Deserialize;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp"];

pub fn scan_images(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut image_files = Vec::new();
    
    let walker = if recursive {
        WalkDir::new(dir)
    } else {
        WalkDir::new(dir).max_depth(1)
    };

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension().and_then(OsStr::to_str) {
                if SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
                    image_files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    Ok(image_files)
}

pub fn ensure_output_dir(output_path: &Path, input_path: &Path, file_path: &Path) -> Result<PathBuf> {
    let relative_path = file_path.strip_prefix(input_path)?;
    let output_file_path = output_path.join(relative_path);
    
    if let Some(parent) = output_file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    Ok(output_file_path)
}

pub fn create_backup(file_path: &Path) -> Result<()> {
    let backup_path = file_path.with_extension(
        format!("{}.bak", 
            file_path.extension()
                .and_then(OsStr::to_str)
                .unwrap_or("")
        )
    );
    std::fs::copy(file_path, backup_path)?;
    Ok(())
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
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

pub fn calculate_resize_dimensions(width: u32, height: u32, max_size: u32) -> (u32, u32) {
    let longer_edge = width.max(height);
    
    if longer_edge <= max_size {
        return (width, height);
    }
    
    let scale_factor = max_size as f64 / longer_edge as f64;
    let new_width = (width as f64 * scale_factor).round() as u32;
    let new_height = (height as f64 * scale_factor).round() as u32;
    
    (new_width, new_height)
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_OWNER: &str = "nixuuu";
const REPO_NAME: &str = "image-optimizer-rs";

fn get_platform_target() -> Result<String> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    
    let target = match (os, arch) {
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") => "aarch64-unknown-linux-gnu",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        ("windows", "x86_64") => "x86_64-pc-windows-msvc",
        _ => return Err(anyhow::anyhow!("Unsupported platform: {}-{}", os, arch)),
    };
    
    Ok(target.to_string())
}

fn compare_versions(current: &str, latest: &str) -> Result<bool> {
    let current_clean = current.trim_start_matches('v');
    let latest_clean = latest.trim_start_matches('v');
    
    let parse_version = |v: &str| -> Result<Vec<u32>> {
        v.split('.')
            .map(|part| part.parse::<u32>().map_err(|e| anyhow::anyhow!("Invalid version format: {}", e)))
            .collect()
    };
    
    let current_parts = parse_version(current_clean)?;
    let latest_parts = parse_version(latest_clean)?;
    
    // Compare version parts (major.minor.patch)
    for (curr, latest) in current_parts.iter().zip(latest_parts.iter()) {
        if latest > curr {
            return Ok(true); // Update available
        } else if curr > latest {
            return Ok(false); // Current is newer
        }
    }
    
    // If all compared parts are equal, check if latest has more parts
    Ok(latest_parts.len() > current_parts.len())
}

fn get_current_executable() -> Result<PathBuf> {
    std::env::current_exe()
        .map_err(|e| anyhow::anyhow!("Failed to get current executable path: {}", e))
}

pub fn update_self() -> Result<()> {
    println!("🔍 Checking for updates...");
    println!("Current version: v{}", CURRENT_VERSION);
    
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", REPO_OWNER, REPO_NAME);
    
    let response = client
        .get(&url)
        .header("User-Agent", format!("{}/{}", REPO_NAME, CURRENT_VERSION))
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
    let binary_name = format!("image-optimizer-rs-{}", target);
    
    let asset = release.assets
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
    std::fs::write(&current_exe, binary_data)
        .map_err(|e| anyhow::anyhow!("Failed to write updated binary: {}", e))?;
    
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