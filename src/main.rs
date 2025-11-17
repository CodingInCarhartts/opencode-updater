// opencode-updater: A utility to update the opencode binary to the latest GitHub release.
// Purpose: The AUR package on Arch Linux updates slowly, and opencode's built-in upgrade
// command was unreliable. This tool fetches and installs the latest Linux x64 binary directly.
// Security Note: This downloads and installs executables with sudo—verify the GitHub source.
// Integrity: Performs SHA-256 checksum verification against GitHub release checksums.

use reqwest::Client;
use sha2::{Digest, Sha256};
use std::io::Cursor;
use std::process::Command;
use zip::ZipArchive;

/// Calculates the SHA-256 hash of the given bytes.
fn calculate_sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Verifies if the SHA-256 hash of bytes matches the expected hash.
fn verify_checksum(bytes: &[u8], expected: &str) -> bool {
    calculate_sha256(bytes) == expected
}

/// Finds an asset by name in the list of assets.
fn find_asset<'a>(assets: &'a [serde_json::Value], name: &str) -> Option<&'a serde_json::Value> {
    assets.iter().find(|a| a["name"] == name)
}

/// Fetches the latest release information from GitHub API.
async fn fetch_release(
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

/// Main entry point: Fetches the latest opencode release, downloads the binary,
/// extracts it, and installs it to /usr/bin/opencode.
/// Requires sudo for installation. Panics on errors for simplicity.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Fetch the latest release information from the GitHub API.
    // Uses a user agent to identify the request (good practice for APIs).
    let client = Client::builder()
        .user_agent("opencode-updater/0.1.0")
        .build()?;
    let release = fetch_release(&client, "https://api.github.com").await?;

    // Step 2: Locate the 'opencode-linux-x64.zip' asset and its checksum in the release.
    // Assumes the asset exists and is named exactly this; panics if not found.
    let assets = release["assets"].as_array().unwrap();
    let asset = find_asset(assets, "opencode-linux-x64.zip").unwrap();
    let download_url = asset["browser_download_url"].as_str().unwrap();

    // Step 2.1: Locate the checksum file for the ZIP asset.
    // Looks for 'opencode-linux-x64.zip.sha256' in the release assets.
    let checksum_asset = find_asset(assets, "opencode-linux-x64.zip.sha256");
    let expected_checksum = match checksum_asset {
        Some(asset) => {
            let checksum_url = asset["browser_download_url"].as_str().unwrap();
            let checksum_response = client.get(checksum_url).send().await?;
            if !checksum_response.status().is_success() {
                eprintln!(
                    "Warning: Failed to download checksum file, proceeding without verification"
                );
                None
            } else {
                let checksum_text = checksum_response.text().await?;
                Some(checksum_text.trim().to_string())
            }
        }
        None => {
            eprintln!(
                "Warning: No checksum file found in release, proceeding without verification"
            );
            None
        }
    };

    // Step 3: Download the ZIP file containing the binary.
    let zip_response = client.get(download_url).send().await?;
    let zip_bytes = zip_response.bytes().await?;

    // Step 3.1: Verify checksum if available.
    // Computes SHA-256 of downloaded ZIP and compares against expected checksum.
    if let Some(expected) = expected_checksum {
        if !verify_checksum(&zip_bytes, &expected) {
            return Err(format!(
                "Checksum mismatch: expected {}, got {}",
                expected,
                calculate_sha256(&zip_bytes)
            )
            .into());
        }
        println!("Checksum verification passed.");
    } else {
        println!("Warning: No checksum verification performed.");
    }

    let cursor = Cursor::new(zip_bytes);

    // Step 4: Extract ZIP to a temporary directory.
    // Uses tempfile for automatic cleanup on drop.
    let mut archive = ZipArchive::new(cursor)?;
    let temp_dir = tempfile::tempdir()?;
    archive.extract(&temp_dir)?;

    // Step 5: Locate the 'opencode' binary file within the extracted archive.
    // Assumes exactly one file named 'opencode' exists; panics if not.
    let mut binary_path = None;
    for entry in std::fs::read_dir(&temp_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap() == "opencode" {
            binary_path = Some(path);
            break;
        }
    }

    let binary_path = binary_path.ok_or("Binary 'opencode' not found in zip")?;
    if !binary_path.exists() {
        return Err("Binary not found".into());
    }

    // Step 6: Move binary to /usr/bin/opencode (system-wide installation).
    // Requires sudo privileges. Overwrites any existing file.
    Command::new("sudo")
        .arg("mv")
        .arg(&binary_path)
        .arg("/usr/bin/opencode")
        .status()?;

    // Step 7: Ensure the binary is executable.
    Command::new("sudo")
        .arg("chmod")
        .arg("+x")
        .arg("/usr/bin/opencode")
        .status()?;

    // Success: Print confirmation message.
    println!("Updated opencode to latest version.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sha256() {
        let data = b"hello world";
        let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert_eq!(calculate_sha256(data), expected);
    }

    #[test]
    fn test_verify_checksum() {
        let data = b"test data";
        let hash = calculate_sha256(data);
        assert!(verify_checksum(data, &hash));
        assert!(!verify_checksum(data, "invalid_hash"));
    }

    #[test]
    fn test_find_asset() {
        let assets = vec![
            serde_json::json!({"name": "opencode-linux-x64.zip", "browser_download_url": "url1"}),
            serde_json::json!({"name": "other.zip", "browser_download_url": "url2"}),
        ];
        let found = find_asset(&assets, "opencode-linux-x64.zip");
        assert!(found.is_some());
        assert_eq!(found.unwrap()["browser_download_url"], "url1");

        let not_found = find_asset(&assets, "missing.zip");
        assert!(not_found.is_none());
    }
}
