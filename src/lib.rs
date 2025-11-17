use reqwest::Client;
use sha2::{Digest, Sha256};

/// Calculates the SHA-256 hash of the given bytes.
pub fn calculate_sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Verifies if the SHA-256 hash of bytes matches the expected hash.
pub fn verify_checksum(bytes: &[u8], expected: &str) -> bool {
    calculate_sha256(bytes) == expected
}

/// Finds an asset by name in the list of assets.
pub fn find_asset<'a>(
    assets: &'a [serde_json::Value],
    name: &str,
) -> Option<&'a serde_json::Value> {
    assets.iter().find(|a| a["name"] == name)
}

/// Fetches the latest release information from GitHub API.
pub async fn fetch_release(
    client: &Client,
    base_url: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let release_url = format!("{}/repos/sst/opencode/releases/latest", base_url);
    let response = client.get(release_url).send().await?;
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        eprintln!("HTTP error: {} {}", status, body);
        return Err("HTTP error".into());
    }
    let release: serde_json::Value = response.json().await?;
    Ok(release)
}
