use anyhow::Result;

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
