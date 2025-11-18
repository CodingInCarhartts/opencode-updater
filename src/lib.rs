use chrono::{DateTime, Utc};
use clap::Parser;
use dialoguer::{Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use ureq::Agent;

/// Custom error types for the updater
#[derive(Debug)]
pub enum UpdaterError {
    VersionNotFound(String),
    NetworkError(String),
    StorageError(String),
    PermissionError(String),
    ChecksumMismatch(String, String),
    InvalidVersionFormat(String),
    RollbackFailed(String),
    GitHubApiError(String),
}

impl std::fmt::Display for UpdaterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdaterError::VersionNotFound(v) => write!(f, "Version '{}' not found", v),
            UpdaterError::NetworkError(e) => write!(f, "Network error: {}", e),
            UpdaterError::StorageError(e) => write!(f, "Storage error: {}", e),
            UpdaterError::PermissionError(e) => write!(f, "Permission error: {}", e),
            UpdaterError::ChecksumMismatch(expected, actual) => {
                write!(
                    f,
                    "Checksum mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            UpdaterError::InvalidVersionFormat(v) => write!(f, "Invalid version format: {}", v),
            UpdaterError::RollbackFailed(e) => write!(f, "Rollback failed: {}", e),
            UpdaterError::GitHubApiError(e) => write!(f, "GitHub API error: {}", e),
        }
    }
}

impl std::error::Error for UpdaterError {}

/// Version information stored in metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionInfo {
    pub version: String,
    pub tag_name: String,
    pub release_date: DateTime<Utc>,
    pub download_url: String,
    pub checksum: String,
    pub installed_at: DateTime<Utc>,
    pub install_path: PathBuf,
    pub release_notes: String,
}

/// Manages version storage and operations
pub struct VersionManager {
    storage_dir: PathBuf,
    versions_dir: PathBuf,
    cache_dir: PathBuf,
}

impl VersionManager {
    /// Initialize version manager with default storage directory
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let data_dir = dirs::data_dir()
            .ok_or("Could not find data directory")?
            .join("opencode-updater");

        let versions_dir = data_dir.join("versions");
        let cache_dir = data_dir.join("cache");

        // Create directories if they don't exist
        std::fs::create_dir_all(&versions_dir)?;
        std::fs::create_dir_all(&cache_dir)?;

        Ok(Self {
            storage_dir: data_dir,
            versions_dir,
            cache_dir,
        })
    }

    /// Get storage directory path
    pub fn storage_dir(&self) -> &Path {
        &self.storage_dir
    }

    /// Get versions directory path
    pub fn versions_dir(&self) -> &Path {
        &self.versions_dir
    }

    /// Get cache directory path
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Get list of installed versions
    pub fn list_installed_versions(&self) -> Result<Vec<VersionInfo>, Box<dyn std::error::Error>> {
        let mut versions = Vec::new();

        if !self.versions_dir.exists() {
            return Ok(versions);
        }

        for entry in std::fs::read_dir(&self.versions_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let metadata_file = path.join("metadata.json");
                if metadata_file.exists() {
                    let content = std::fs::read_to_string(metadata_file)?;
                    let version_info: VersionInfo = serde_json::from_str(&content)?;
                    versions.push(version_info);
                }
            }
        }

        // Sort by installation date (newest first)
        versions.sort_by(|a, b| b.installed_at.cmp(&a.installed_at));
        Ok(versions)
    }

    /// Get current active version
    pub fn get_current_version(&self) -> Result<Option<VersionInfo>, Box<dyn std::error::Error>> {
        // First check if we have a local storage symlink
        let current_link = self.storage_dir.join("current");

        if current_link.exists() {
            let target = std::fs::read_link(&current_link)?;
            let metadata_file = target.join("metadata.json");

            if metadata_file.exists() {
                let content = std::fs::read_to_string(metadata_file)?;
                let version_info: VersionInfo = serde_json::from_str(&content)?;
                return Ok(Some(version_info));
            }
        }

        // Fallback: Check if system binary exists and try to detect its version
        let system_binary = Path::new("/usr/bin/opencode");
        if system_binary.exists() {
            // Try to get version from the binary
            if let Some(version_info) = self.detect_system_version()? {
                return Ok(Some(version_info));
            }
        }

        Ok(None)
    }

    /// Detect version of system-installed binary
    fn detect_system_version(&self) -> Result<Option<VersionInfo>, Box<dyn std::error::Error>> {
        let output = Command::new("opencode").arg("--version").output().ok();

        if let Some(output) = output {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                let version = version_str.trim().trim_start_matches('v');

                // Create version info for detected system binary
                let version_info = VersionInfo {
                    version: version.to_string(),
                    tag_name: format!("v{}", version),
                    release_date: Utc::now(), // Unknown, use current time
                    download_url: String::new(),
                    checksum: String::new(),
                    installed_at: Utc::now(), // Unknown, use current time
                    install_path: PathBuf::from("/usr/bin/opencode"),
                    release_notes: "Currently installed version (release notes unknown)"
                        .to_string(),
                };

                return Ok(Some(version_info));
            }
        }

        Ok(None)
    }

    /// Save version with metadata
    pub fn save_version(
        &self,
        version: &VersionInfo,
        binary_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let version_dir = self.versions_dir.join(&version.version);
        std::fs::create_dir_all(&version_dir)?;

        // Copy binary to version directory
        let version_binary = version_dir.join("opencode");
        std::fs::copy(binary_path, &version_binary)?;

        // Make it executable
        let mut perms = std::fs::metadata(&version_binary)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&version_binary, perms)?;

        // Save metadata
        let metadata_file = version_dir.join("metadata.json");
        let metadata_json = serde_json::to_string_pretty(version)?;
        std::fs::write(metadata_file, metadata_json)?;

        Ok(())
    }

    /// Rollback to specific version
    pub fn rollback_to(&self, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        let version_dir = self.versions_dir.join(version);
        let version_binary = version_dir.join("opencode");
        let metadata_file = version_dir.join("metadata.json");

        if !version_binary.exists() {
            return Err(UpdaterError::VersionNotFound(version.to_string()).into());
        }

        // Read metadata to verify
        let _version_info: VersionInfo =
            serde_json::from_str(&std::fs::read_to_string(metadata_file)?)?;

        // Install the binary to system location
        Command::new("sudo")
            .arg("cp")
            .arg(&version_binary)
            .arg("/usr/bin/opencode")
            .status()?;

        // Ensure executable permissions
        Command::new("sudo")
            .arg("chmod")
            .arg("+x")
            .arg("/usr/bin/opencode")
            .status()?;

        // Update current symlink
        let current_link = self.storage_dir.join("current");
        if current_link.exists() {
            std::fs::remove_file(&current_link)?;
        }
        std::os::unix::fs::symlink(&version_dir, &current_link)?;

        println!("Successfully rolled back to version {}", version);
        Ok(())
    }

    /// Clean up old versions (keep only N most recent, default: 2)
    pub fn cleanup_old_versions(
        &self,
        keep_count: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut versions = self.list_installed_versions()?;

        if versions.len() <= keep_count {
            return Ok(());
        }

        // Remove versions beyond keep_count (excluding current version)
        let current_version = self.get_current_version()?;
        versions.retain(|v| {
            current_version
                .as_ref()
                .map_or(true, |curr| curr.version != v.version)
        });

        versions.sort_by(|a, b| b.installed_at.cmp(&a.installed_at));

        for version in versions.iter().skip(keep_count) {
            let version_dir = self.versions_dir.join(&version.version);
            std::fs::remove_dir_all(&version_dir)?;
            println!("Removed old version: {}", version.version);
        }

        Ok(())
    }

    /// Backup current version before updating
    pub fn backup_current_version(
        &self,
    ) -> Result<Option<VersionInfo>, Box<dyn std::error::Error>> {
        // Check if current binary exists
        if !Path::new("/usr/bin/opencode").exists() {
            return Ok(None);
        }

        // Try to get version info from running binary
        let output = Command::new("opencode").arg("--version").output().ok();

        if let Some(output) = output {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                let version = version_str.trim().trim_start_matches('v');

                // Create version info
                let version_info = VersionInfo {
                    version: version.to_string(),
                    tag_name: format!("v{}", version),
                    release_date: Utc::now(),
                    download_url: String::new(),
                    checksum: String::new(),
                    installed_at: Utc::now(),
                    install_path: PathBuf::from("/usr/bin/opencode"),
                    release_notes: "Current installation".to_string(),
                };

                // Save current binary
                let current_binary = Path::new("/usr/bin/opencode");
                self.save_version(&version_info, current_binary)?;

                return Ok(Some(version_info));
            }
        }

        Ok(None)
    }
}

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

/// Fetch all releases (not just latest) from GitHub API.
pub fn fetch_all_releases(
    client: &Agent,
    base_url: &str,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let releases_url = format!("{}/repos/sst/opencode/releases", base_url);
    let response = client.get(&releases_url).call()?;
    if !response.status() == 200 {
        let status = response.status();
        let body = response.into_string()?;
        eprintln!("HTTP error: {} {}", status, body);
        return Err("HTTP error".into());
    }
    let releases: Vec<serde_json::Value> = response.into_json()?;
    Ok(releases)
}

/// Fetch specific release by tag from GitHub API.
pub fn fetch_release_by_tag(
    client: &Agent,
    base_url: &str,
    tag: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let release_url = format!("{}/repos/sst/opencode/releases/tags/{}", base_url, tag);
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

/// Format release notes for display.
pub fn format_release_notes(
    release: &serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let tag_name = release["tag_name"].as_str().unwrap_or("Unknown");
    let name = release["name"].as_str().unwrap_or(tag_name);
    let published_at = release["published_at"].as_str().unwrap_or("Unknown date");
    let body = release["body"]
        .as_str()
        .unwrap_or("No release notes available.");

    let formatted = format!(
        "📦 Release: {} ({})\n📅 Published: {}\n\n{}",
        name, tag_name, published_at, body
    );

    Ok(formatted)
}

/// Cache releases locally to avoid API rate limits.
pub fn cache_releases(
    releases: &[serde_json::Value],
    cache_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let cache_file = cache_dir.join("releases.json");
    let json = serde_json::to_string_pretty(releases)?;
    std::fs::write(cache_file, json)?;
    Ok(())
}

/// Load cached releases if available and recent (< 1 hour).
pub fn load_cached_releases(
    cache_dir: &Path,
) -> Result<Option<Vec<serde_json::Value>>, Box<dyn std::error::Error>> {
    let cache_file = cache_dir.join("releases.json");
    if !cache_file.exists() {
        return Ok(None);
    }

    let metadata = std::fs::metadata(&cache_file)?;
    let modified = metadata.modified()?;
    let age = std::time::SystemTime::now().duration_since(modified)?;

    if age > std::time::Duration::from_secs(3600) {
        return Ok(None); // Cache expired
    }

    let content = std::fs::read_to_string(cache_file)?;
    let releases: Vec<serde_json::Value> = serde_json::from_str(&content)?;
    Ok(Some(releases))
}

/// Parse semantic version string for comparison.
pub fn parse_version(version: &str) -> Result<(u64, u64, u64), Box<dyn std::error::Error>> {
    let clean = version.trim_start_matches('v');
    let parts: Vec<&str> = clean.split('.').collect();

    if parts.len() != 3 {
        return Err("Invalid version format".into());
    }

    let major = parts[0].parse()?;
    let minor = parts[1].parse()?;
    let patch = parts[2].parse()?;

    Ok((major, minor, patch))
}

/// Compare two versions (-1, 0, 1).
pub fn compare_versions(v1: &str, v2: &str) -> Result<i8, Box<dyn std::error::Error>> {
    let (major1, minor1, patch1) = parse_version(v1)?;
    let (major2, minor2, patch2) = parse_version(v2)?;

    if major1 != major2 {
        return Ok(major1.cmp(&major2) as i8);
    }
    if minor1 != minor2 {
        return Ok(minor1.cmp(&minor2) as i8);
    }
    Ok(patch1.cmp(&patch2) as i8)
}

/// Display comparison between two versions.
pub fn display_version_comparison(
    from_release: &serde_json::Value,
    to_release: &serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let from_tag = from_release["tag_name"].as_str().unwrap_or("Unknown");
    let to_tag = to_release["tag_name"].as_str().unwrap_or("Unknown");
    let from_date = from_release["published_at"].as_str().unwrap_or("Unknown");
    let to_date = to_release["published_at"].as_str().unwrap_or("Unknown");

    let comparison = format!(
        "🔄 Version Comparison\n\n\
         📦 From: {} (published: {})\n\
         📦 To: {} (published: {})\n\n\
         📝 Changes in {}:\n\
         {}",
        from_tag,
        from_date,
        to_tag,
        to_date,
        to_tag,
        to_release["body"]
            .as_str()
            .unwrap_or("No release notes available.")
    );

    Ok(comparison)
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

    /// Rollback to a previous version
    #[arg(long, value_name = "VERSION")]
    pub rollback: Option<String>,

    /// List all installed and available versions
    #[arg(long)]
    pub list_versions: bool,

    /// Show changelog for a specific version (or latest if not specified)
    #[arg(long, value_name = "VERSION")]
    pub changelog: Option<String>,

    /// Compare two versions to see differences
    #[arg(long, value_names = ["FROM", "TO"])]
    pub compare: Option<Vec<String>>,

    /// Maximum number of versions to keep locally (default: 2)
    #[arg(long, default_value = "2")]
    pub keep_versions: usize,

    /// Force update even if already on latest version
    #[arg(long)]
    pub force: bool,
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
    // Initialize version manager
    let version_manager = VersionManager::new()?;

    // Backup current version before updating
    if !skip_install {
        if let Some(backup_info) = version_manager.backup_current_version()? {
            println!("Backed up current version: {}", backup_info.version);
        }
    }

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
    if let Some(ref expected) = expected_checksum
        && !verify_checksum(&zip_bytes, expected)
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
        // Create version info for the new version
        let version = release["tag_name"].as_str().unwrap_or("unknown");
        let version_clean = version.trim_start_matches('v');

        let version_info = VersionInfo {
            version: version_clean.to_string(),
            tag_name: version.to_string(),
            release_date: release["published_at"]
                .as_str()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now),
            download_url: download_url.clone(),
            checksum: expected_checksum.unwrap_or_default(),
            installed_at: Utc::now(),
            install_path: PathBuf::from("/usr/bin/opencode"),
            release_notes: release["body"]
                .as_str()
                .unwrap_or("No release notes available.")
                .to_string(),
        };

        // Save the new version to storage
        version_manager.save_version(&version_info, &binary_path)?;

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

        // Update current symlink
        let current_link = version_manager.storage_dir.join("current");
        let version_dir = version_manager.versions_dir.join(&version_info.version);
        if current_link.exists() {
            std::fs::remove_file(&current_link)?;
        }
        std::os::unix::fs::symlink(&version_dir, &current_link)?;

        // Clean up old versions
        version_manager.cleanup_old_versions(args.keep_versions)?;

        println!("Updated opencode to version {}.", version_clean);
    }
    Ok(())
}
