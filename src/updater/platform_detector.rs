use anyhow::Result;

/// Detects the current platform target for binary downloads
///
/// # Errors
/// Returns an error if the current platform is unsupported
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
