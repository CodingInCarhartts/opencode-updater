# opencode-updater

![CI](https://github.com/CodingInCarhartts/opencode-updater/workflows/CI/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)
![Version](https://img.shields.io/badge/version-0.1.0-blue)

A powerful Rust utility to update the `opencode` binary with advanced version management capabilities. This tool was created because the AUR package on Arch Linux didn't update quickly enough, and the built-in upgrade command in `opencode` wasn't working reliably.

## Table of Contents

- [Purpose](#purpose)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [New Features](#new-features)
- [Security Notes](#security-notes)
- [Dependencies](#dependencies)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Purpose

- Downloads the latest `opencode` binary (Linux x64) from the official GitHub releases.
- Extracts and installs it to `/usr/bin/opencode` using `sudo`.
- Ensures the binary is executable.
- Supports interactive selection of binaries via the `--bin` flag.
- **NEW:** Version management with automatic backup of previous versions.
- **NEW:** Rollback to any previously installed version.
- **NEW:** View release notes and compare versions.
- **NEW:** Local caching of versions for quick operations.
- **NEW:** Configurable version retention (default: keeps 2 most recent versions).
- Provides a faster alternative to waiting for AUR updates or relying on broken upgrade commands.

## Prerequisites

- Rust (for building from source).
- `sudo` access (required for installing to `/usr/bin`).
- Internet connection (to fetch from GitHub).

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/CodingInCarhartts/opencode-updater.git
   cd opencode-updater
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the updater:
   ```bash
   ./target/release/opencode-updater
   ```

Optional:
   Install the updater:
   ```bash
   cd opencode-updater && cargo install --path .
   opencode-updater
   ```

## Usage

### Basic Update
Simply run the binary. It will:
- Fetch the latest release info from GitHub.
- Download the `opencode-linux-x64.zip` asset.
- Extract the `opencode` binary to a temporary directory.
- Move it to `/usr/bin/opencode` (requires `sudo`).
- Make it executable.
- **NEW:** Backup current version before updating.

Use the `--bin` flag for interactive selection of available binaries from the release.

### Version Management

#### List Versions
```bash
opencode-updater --list-versions
```
Shows all installed versions and available updates.

#### Rollback to Previous Version
```bash
opencode-updater --rollback 1.0.72
```
Rollback to any previously installed version.

#### View Release Notes
```bash
opencode-updater --changelog latest
opencode-updater --changelog 1.0.73
```
View release notes for specific or latest version.

#### Compare Versions
```bash
opencode-updater --compare 1.0.72 1.0.73
```
See what changed between two versions.

#### Configuration Options
```bash
opencode-updater --keep-versions 3  # Keep only 3 recent versions (default: 2)
opencode-updater --force           # Force update even if on latest
```

### Example Output
```
📦 opencode Versions

✅ Current: 1.0.73 (installed: 2025-11-18 17:10)

📁 Installed Versions:
  → 1.0.73 (2025-11-18)
  → 1.0.72 (2025-11-18)

🌐 Available Updates:
  📦 v1.0.73 (2025-11-18)
  📦 v1.0.72 (2025-11-18)
```

## New Features

### Version Management & Rollback
- **Automatic Backup:** Before each update, the current version is automatically backed up to `~/.local/share/opencode-updater/versions/`
- **Quick Rollback:** Instantly rollback to any previously installed version with `--rollback <version>`
- **Version History:** View all installed versions with installation dates
- **Storage Management:** Configurable version retention with `--keep-versions <count>` (default: 2)

### Release Information
- **Release Notes:** View detailed changelog for any version with `--changelog <version>`
- **Version Comparison:** Compare two versions to see what changed with `--compare <from> <to>`
- **GitHub Integration:** Direct access to GitHub release data with local caching
- **Offline Support:** Cached release information available when network is unavailable

### Storage Location
All version data is stored in:
```
~/.local/share/opencode-updater/
├── versions/           # Stored versions with metadata
├── cache/             # GitHub API cache
└── current            # Symlink to active version
```

## Security Notes

- This tool downloads and installs binaries directly. Always verify the source (GitHub releases) and consider the risks of running unverified executables.
- Requires `sudo` for system-wide installation—use at your own risk.
- Performs SHA-256 checksum verification against GitHub release checksums when available.
- Checksums provide integrity protection but not authenticity; still trust GitHub as the source.

## Dependencies

- `ureq` (for HTTP requests).
- `clap` (for command-line argument parsing).
- `dialoguer` (for interactive prompts).
- `zip` (for extracting archives).
- `serde_json` (for parsing GitHub API responses).
- `serde` (for serialization/deserialization).
- `tempfile` (for temporary directories).
- `sha2` (for SHA-256 checksum verification).
- `chrono` (for date/time handling).
- `semver` (for version comparison).
- `dirs` (for finding user data directories).
- `indicatif` (for progress bars).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

- GitHub: [https://github.com/CodingInCarhartts](https://github.com/CodingInCarhartts)
- Email: yumlabs.team@gmail.com