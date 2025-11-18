use clap::Parser;
use dialoguer::{Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use ureq::Agent;

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

/// Downloads a file with real-time progress display.
///
/// This function downloads a file from the given URL while displaying a progress bar
/// that shows the download progress, speed, and estimated time remaining.
///
/// # Arguments
///
/// * `client` - The HTTP client to use for the request
/// * `url` - The URL to download from
/// * `filename` - The filename to display in the progress bar
///
/// # Returns
///
/// Returns a `Vec<u8>` containing the downloaded file data.
///
/// # Errors
///
/// Returns an error if the HTTP request fails or if there's an I/O error during download.
pub fn download_with_progress(
    client: &Agent,
    url: &str,
    filename: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Make the request to get headers first
    let response = client.get(url).call()?;

    // Get content length for progress bar
    let content_length = response
        .header("Content-Length")
        .and_then(|len| len.parse::<u64>().ok())
        .unwrap_or(0);

    // Create progress bar
    let progress = ProgressBar::new(content_length);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    progress.set_message(format!("Downloading {}", filename));

    // Start the download with progress tracking
    let mut reader = response.into_reader();
    let mut buffer = Vec::new();
    let mut chunk = [0; 8192]; // 8KB chunks

    loop {
        let bytes_read = reader.read(&mut chunk)?;
        if bytes_read == 0 {
            break;
        }

        buffer.extend_from_slice(&chunk[..bytes_read]);
        progress.inc(bytes_read as u64);
    }

    progress.finish_with_message(format!("Downloaded {}", filename));
    Ok(buffer)
}

/// Finds an asset by name in the list of assets.
pub fn find_asset<'a>(
    assets: &'a [serde_json::Value],
    name: &str,
) -> Option<&'a serde_json::Value> {
    assets.iter().find(|a| a["name"] == name)
}

/// Fetches the latest release information from GitHub API.
pub fn fetch_release(
    client: &Agent,
    base_url: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let release_url = format!("{}/repos/sst/opencode/releases/latest", base_url);
    let response = client.get(&release_url).call()?;
    if !response.status() == 200 {
        let status = response.status();
        let body = response.into_string()?;
        eprintln!("HTTP error: {} {}", status, body);
        return Err("HTTP error".into());
    }
    let release: serde_json::Value = response.into_json()?;
    Ok(release)
}

/// Finds the executable binary in the given directory.
pub fn find_executable_binary(temp_dir: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(temp_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && path
                .metadata()
                .is_ok_and(|m| m.permissions().mode() & 0o111 != 0)
        {
            return Ok(path);
        }
    }
    Err("No executable binary found".into())
}

/// Command line arguments for the opencode updater.
#[derive(Parser)]
#[command(name = "opencode-updater")]
#[command(about = "Update opencode to the latest version")]
pub struct Args {
    /// Enable interactive binary selection instead of using the default asset.
    #[arg(long)]
    pub bin: bool,
}

/// Runs the update process: Fetches the latest opencode release, downloads the binary,
/// extracts it, and optionally installs it.
/// If skip_install is true, skips the installation steps.
/// If asset_override is Some, uses the provided asset name and URL instead of selecting.
pub fn run_update(
    args: &Args,
    client: &Agent,
    base_url: &str,
    asset_override: Option<(String, String)>,
    skip_install: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let release = fetch_release(client, base_url)?;

    // Step 2: Select the asset to download.
    let assets = release["assets"].as_array().unwrap();
    let (asset_name, download_url) = if let Some((name, url)) = asset_override {
        (name, url)
    } else if args.bin {
        // Interactive mode
        let binary_assets: Vec<_> = assets
            .iter()
            .filter(|a| {
                let name = a["name"].as_str().unwrap();
                name.ends_with(".zip") || name.ends_with(".tar.gz") || name.contains("linux")
            })
            .collect();
        if binary_assets.is_empty() {
            return Err("No binary assets found in release".into());
        }
        let options: Vec<String> = binary_assets
            .iter()
            .map(|a| a["name"].as_str().unwrap().to_string())
            .collect();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a binary to install")
            .default(0)
            .items(&options)
            .interact()
            .unwrap();
        let selected_asset = &binary_assets[selection];
        let asset_name = selected_asset["name"].as_str().unwrap().to_string();
        let download_url = selected_asset["browser_download_url"]
            .as_str()
            .unwrap()
            .to_string();
        (asset_name, download_url)
    } else {
        let asset = find_asset(assets, "opencode-linux-x64.zip")
            .ok_or("Default asset 'opencode-linux-x64.zip' not found")?;
        let asset_name = "opencode-linux-x64.zip".to_string();
        let download_url = asset["browser_download_url"].as_str().unwrap().to_string();
        (asset_name, download_url)
    };

    // Step 2.1: Locate the checksum file for the selected asset (if available).
    let checksum_name = format!("{}.sha256", asset_name);
    let checksum_asset = find_asset(assets, &checksum_name);
    let expected_checksum = match checksum_asset {
        Some(asset) => {
            let checksum_url = asset["browser_download_url"].as_str().unwrap();
            let checksum_response = client.get(checksum_url).call()?;
            if !checksum_response.status() == 200 {
                None
            } else {
                let checksum_text = checksum_response.into_string()?;
                Some(checksum_text.trim().to_string())
            }
        }
        None => None,
    };

    // Step 3: Download the selected archive with progress display.
    let zip_bytes = download_with_progress(client, &download_url, &asset_name)?;

    // Step 3.1: Verify checksum if available.
    if let Some(expected) = expected_checksum
        && !verify_checksum(&zip_bytes, &expected)
    {
        return Err(format!(
            "Checksum mismatch: expected {}, got {}",
            expected,
            calculate_sha256(&zip_bytes)
        )
        .into());
    }

    let cursor = std::io::Cursor::new(zip_bytes);

    // Step 4: Extract the archive to a temporary directory.
    let mut archive = zip::ZipArchive::new(cursor)?;
    let temp_dir = tempfile::tempdir()?;
    archive.extract(&temp_dir)?;

    // Step 5: Locate the executable binary within the extracted files.
    let binary_path = find_executable_binary(temp_dir.path())?;

    if !skip_install {
        // Step 6: Install the binary to /usr/bin/opencode (system-wide).
        Command::new("sudo")
            .arg("mv")
            .arg(&binary_path)
            .arg("/usr/bin/opencode")
            .status()?;

        // Step 7: Ensure the installed binary has execute permissions.
        Command::new("sudo")
            .arg("chmod")
            .arg("+x")
            .arg("/usr/bin/opencode")
            .status()?;

        println!("Updated opencode to latest version.");
    }
    Ok(())
}
