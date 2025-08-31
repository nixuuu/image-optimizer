use anyhow::Result;

/// Compares two semantic version strings to determine if an update is available.
///
/// This function implements semantic version comparison following the semver specification.
/// It automatically strips 'v' prefixes and compares major.minor.patch version numbers.
/// The comparison determines whether the latest version is newer than the current version.
///
/// # Arguments
///
/// * `current` - The current version string (e.g., "1.2.1" or "v1.2.1")
/// * `latest` - The latest available version string (e.g., "1.2.2" or "v1.2.2")
///
/// # Returns
///
/// Returns `true` if the latest version is newer than the current version,
/// `false` if the current version is newer or equal to the latest version.
///
/// # Errors
///
/// Returns an error if either version string cannot be parsed as a valid
/// semantic version (must contain numeric parts separated by dots).
///
/// # Examples
///
/// ```rust
/// use image_optimizer::updater::version_comparator::compare_versions;
///
/// # fn example() -> anyhow::Result<()> {
/// // Update available
/// assert!(compare_versions("1.0.0", "1.0.1")?);
/// assert!(compare_versions("v1.0.0", "v1.1.0")?);
///
/// // No update needed
/// assert!(!compare_versions("1.0.1", "1.0.0")?);
/// assert!(!compare_versions("1.0.0", "1.0.0")?);
/// # Ok(())
/// # }
/// ```
pub fn compare_versions(current: &str, latest: &str) -> Result<bool> {
    let current_clean = current.trim_start_matches('v');
    let latest_clean = latest.trim_start_matches('v');

    let parse_version = |v: &str| -> Result<Vec<u32>> {
        v.split('.')
            .map(|part| {
                part.parse::<u32>()
                    .map_err(|e| anyhow::anyhow!("Invalid version format: {}", e))
            })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(compare_versions("1.0.0", "1.0.1").unwrap());
        assert!(compare_versions("1.0.0", "1.1.0").unwrap());
        assert!(compare_versions("1.0.0", "2.0.0").unwrap());
        assert!(!compare_versions("1.0.1", "1.0.0").unwrap());
        assert!(!compare_versions("1.1.0", "1.0.0").unwrap());
        assert!(!compare_versions("2.0.0", "1.0.0").unwrap());
        assert!(!compare_versions("1.0.0", "1.0.0").unwrap());
    }

    #[test]
    fn test_version_with_v_prefix() {
        assert!(compare_versions("v1.0.0", "v1.0.1").unwrap());
        assert!(compare_versions("1.0.0", "v1.0.1").unwrap());
        assert!(compare_versions("v1.0.0", "1.0.1").unwrap());
    }

    #[test]
    fn test_different_version_lengths() {
        assert!(compare_versions("1.0", "1.0.1").unwrap());
        assert!(!compare_versions("1.0.1", "1.0").unwrap());
        assert!(!compare_versions("1.0", "1.0").unwrap());
    }

    #[test]
    fn test_invalid_version_format() {
        assert!(compare_versions("invalid", "1.0.0").is_err());
        assert!(compare_versions("1.0.0", "invalid").is_err());
        assert!(compare_versions("1.x.0", "1.0.0").is_err());
    }

    #[test]
    fn test_real_world_versions() {
        assert!(compare_versions("1.2.1", "1.2.2").unwrap());
        assert!(compare_versions("0.9.0", "1.0.0").unwrap());
        assert!(!compare_versions("2.0.0", "1.9.9").unwrap());
    }
}
