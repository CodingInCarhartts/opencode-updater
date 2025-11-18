//! opencode-updater: A utility to update the opencode binary to the latest GitHub release.
//! Purpose: The AUR package on Arch Linux updates slowly, and opencode's built-in upgrade
//! command was unreliable. This tool fetches and installs the latest Linux x64 binary directly.
//! Security Note: This downloads and installs executables with sudo—verify the GitHub source.
//! Integrity: Performs SHA-256 checksum verification against GitHub release checksums.

use clap::Parser;
use opencode_updater::{
    Args, VersionManager, cache_releases, display_version_comparison, fetch_all_releases,
    fetch_release_by_tag, format_release_notes, load_cached_releases, run_update,
};
use ureq::Agent;

/// Main entry point: Parses arguments and runs the appropriate command.
/// Requires sudo for installation. Panics on errors for simplicity.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize version manager
    let version_manager = VersionManager::new()?;

    // Create HTTP client
    let client = Agent::new();

    // Handle different commands
    if let Some(version) = &args.rollback {
        return handle_rollback(&version_manager, version);
    }

    if args.list_versions {
        return handle_list_versions(&version_manager, &client);
    }

    if let Some(version) = &args.changelog {
        return handle_changelog(&client, version);
    }

    if let Some(versions) = &args.compare {
        return handle_compare(&client, versions);
    }

    // Default: update to latest
    run_update(&args, &client, "https://api.github.com", None, false)
}

/// Handle rollback command
fn handle_rollback(
    version_manager: &VersionManager,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    version_manager.rollback_to(version)
}

/// Handle list versions command
fn handle_list_versions(
    version_manager: &VersionManager,
    client: &ureq::Agent,
) -> Result<(), Box<dyn std::error::Error>> {
    let installed = version_manager.list_installed_versions()?;
    let current = version_manager.get_current_version()?;

    // Try to load cached releases first
    let available = match load_cached_releases(version_manager.cache_dir())? {
        Some(releases) => releases,
        None => {
            // Fetch from GitHub if cache is empty or expired
            match fetch_all_releases(client, "https://api.github.com") {
                Ok(releases) => {
                    // Cache the releases
                    let _ = cache_releases(&releases, version_manager.cache_dir());
                    releases
                }
                Err(_) => {
                    // If network fails, continue with empty list
                    vec![]
                }
            }
        }
    };

    let output = format_version_list(&installed, &available, current.as_ref());
    println!("{}", output);

    Ok(())
}

/// Handle changelog command
fn handle_changelog(client: &ureq::Agent, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let release = if version == "latest" || version.is_empty() {
        // Fetch latest release
        opencode_updater::fetch_release(client, "https://api.github.com")?
    } else {
        // Fetch specific release by tag
        let tag = if !version.starts_with('v') {
            format!("v{}", version)
        } else {
            version.to_string()
        };
        fetch_release_by_tag(client, "https://api.github.com", &tag)?
    };

    let changelog = format_release_notes(&release)?;
    println!("{}", changelog);

    Ok(())
}

/// Handle compare command
fn handle_compare(
    client: &ureq::Agent,
    versions: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if versions.len() != 2 {
        return Err("Compare requires exactly two version arguments".into());
    }

    let from_tag = if !versions[0].starts_with('v') {
        format!("v{}", versions[0])
    } else {
        versions[0].clone()
    };

    let to_tag = if !versions[1].starts_with('v') {
        format!("v{}", versions[1])
    } else {
        versions[1].clone()
    };

    let from_release = fetch_release_by_tag(client, "https://api.github.com", &from_tag)?;
    let to_release = fetch_release_by_tag(client, "https://api.github.com", &to_tag)?;

    let comparison = display_version_comparison(&from_release, &to_release)?;
    println!("{}", comparison);

    Ok(())
}

/// Format version list for display
fn format_version_list(
    installed: &[opencode_updater::VersionInfo],
    available: &[serde_json::Value],
    current: Option<&opencode_updater::VersionInfo>,
) -> String {
    let mut output = String::new();

    output.push_str("📦 opencode Versions\n\n");

    // Current version
    if let Some(curr) = current {
        output.push_str(&format!(
            "✅ Current: {} (installed: {})\n\n",
            curr.version,
            curr.installed_at.format("%Y-%m-%d %H:%M")
        ));
    } else {
        output.push_str("❌ No version currently installed\n\n");
    }

    // Installed versions
    if !installed.is_empty() {
        output.push_str("📁 Installed Versions:\n");
        for version in installed {
            let marker = if current
                .as_ref()
                .is_some_and(|c| c.version == version.version)
            {
                "→"
            } else {
                " "
            };
            output.push_str(&format!(
                "  {} {} ({})\n",
                marker,
                version.version,
                version.installed_at.format("%Y-%m-%d")
            ));
        }
    } else {
        output.push_str("📁 No versions installed locally\n");
    }

    // Available updates
    if !available.is_empty() {
        output.push_str("\n🌐 Available Updates:\n");
        for release in available.iter().take(5) {
            let tag = release["tag_name"].as_str().unwrap_or("Unknown");
            let date = release["published_at"].as_str().unwrap_or("Unknown");
            output.push_str(&format!("  📦 {} ({})\n", tag, &date[..10]));
        }
    } else {
        output.push_str("\n🌐 No version information available (network required)\n");
    }

    output
}
