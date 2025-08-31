use serde::Deserialize;

#[derive(Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
}