use serde::Deserialize;

/// GitHub release information from the GitHub API.
///
/// This struct represents the JSON response from GitHub's releases API endpoint.
/// It contains the essential information needed for the self-update process.
#[derive(Deserialize)]
pub struct GitHubRelease {
    /// The git tag name for this release (e.g., "v1.3.0").
    pub tag_name: String,
    /// List of downloadable assets (binaries) for this release.
    pub assets: Vec<GitHubAsset>,
}

/// A downloadable asset from a GitHub release.
///
/// This struct represents individual files attached to a GitHub release,
/// typically containing compiled binaries for different platforms.
#[derive(Deserialize)]
pub struct GitHubAsset {
    /// The filename of the asset (e.g., "image-optimizer-x86_64-apple-darwin").
    pub name: String,
    /// Direct download URL for the asset.
    pub browser_download_url: String,
}
