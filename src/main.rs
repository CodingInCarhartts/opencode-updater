//! opencode-updater: A utility to update the opencode binary to the latest GitHub release.
//! Purpose: The AUR package on Arch Linux updates slowly, and opencode's built-in upgrade
//! command was unreliable. This tool fetches and installs the latest Linux x64 binary directly.
//! Security Note: This downloads and installs executables with sudo—verify the GitHub source.
//! Integrity: Performs SHA-256 checksum verification against GitHub release checksums.

use clap::Parser;
use opencode_updater::{Args, run_update};
use ureq::Agent;

/// Main entry point: Parses arguments and runs the update process.
/// Requires sudo for installation. Panics on errors for simplicity.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Create HTTP client.
    let client = Agent::new();

    run_update(&args, &client, "https://api.github.com", None, false)
}
