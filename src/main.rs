// opencode-updater: A utility to update the opencode binary to the latest GitHub release.
// Purpose: The AUR package on Arch Linux updates slowly, and opencode's built-in upgrade
// command was unreliable. This tool fetches and installs the latest Linux x64 binary directly.
// Security Note: This downloads and installs executables with sudo—verify the GitHub source.

use reqwest::Client;
use std::io::Cursor;
use std::process::Command;
use zip::ZipArchive;

/// Main entry point: Fetches the latest opencode release, downloads the binary,
/// extracts it, and installs it to /usr/bin/opencode.
/// Requires sudo for installation. Panics on errors for simplicity.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Fetch the latest release information from the GitHub API.
    // Uses a user agent to identify the request (good practice for APIs).
    let client = Client::builder().user_agent("opencode-updater/0.1.0").build()?;
    let release_url = "https://api.github.com/repos/sst/opencode/releases/latest";
    let response = client.get(release_url).send().await?;
    if !response.status().is_success() {
        // Handle HTTP errors by printing the status and body, then exiting.
        let status = response.status();
        let body = response.text().await?;
        eprintln!("HTTP error: {} {}", status, body);
        return Err("HTTP error".into());
    }
    // Parse the JSON response into a serde Value for easy access.
    let release: serde_json::Value = response.json().await?;

    // Step 2: Locate the 'opencode-linux-x64.zip' asset in the release.
    // Assumes the asset exists and is named exactly this; panics if not found.
    let assets = release["assets"].as_array().unwrap();
    let asset = assets.iter().find(|a| a["name"] == "opencode-linux-x64.zip").unwrap();
    let download_url = asset["browser_download_url"].as_str().unwrap();

    // Step 3: Download the ZIP file containing the binary.
    let zip_response = client.get(download_url).send().await?;
    let zip_bytes = zip_response.bytes().await?;
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
    Command::new("sudo").arg("mv").arg(&binary_path).arg("/usr/bin/opencode").status()?;

    // Step 7: Ensure the binary is executable.
    Command::new("sudo").arg("chmod").arg("+x").arg("/usr/bin/opencode").status()?;

    // Success: Print confirmation message.
    println!("Updated opencode to latest version.");
    Ok(())
}
