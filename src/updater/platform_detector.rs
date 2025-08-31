use anyhow::Result;

/// Detects the current platform target for binary downloads.
///
/// This function determines the Rust target triple for the current platform,
/// which is used to identify the correct binary asset to download from GitHub releases.
/// The detection is based on the OS and architecture constants provided by Rust's
/// standard library.
///
/// # Returns
///
/// Returns the Rust target triple as a `String` for the current platform.
///
/// # Errors
///
/// Returns an error if the current platform combination is not supported
/// for automatic updates.
///
/// # Supported Platforms
///
/// - **Linux**: `x86_64`, aarch64
/// - **macOS**: `x86_64` (Intel), aarch64 (Apple Silicon)
/// - **Windows**: `x86_64`
///
/// # Examples
///
/// ```rust
/// use image_optimizer::updater::get_platform_target;
///
/// # fn example() -> anyhow::Result<()> {
/// let target = get_platform_target()?;
/// // On Apple Silicon Mac: "aarch64-apple-darwin"
/// // On Intel Mac: "x86_64-apple-darwin"  
/// // On Linux x64: "x86_64-unknown-linux-gnu"
/// # Ok(())
/// # }
/// ```
pub fn get_platform_target() -> Result<String> {
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
