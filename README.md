# opencode-updater

![CI](https://github.com/CodingInCarhartts/opencode-updater/workflows/CI/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)
![Version](https://img.shields.io/badge/version-0.2.0-blue)

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
- **NEW:** Real-time download progress with visual indicators and ETA.
- **NEW:** Archive format fallback (zip → tar.gz) for maximum compatibility.
- **NEW:** System version detection for seamless migration.
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
- Download the `opencode-linux-x64.zip` asset (falls back to `opencode-linux-x64.tar.gz` if zip is unavailable).
- **NEW:** Display real-time download progress with visual bar and ETA.
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
opencode-updater --bin             # Interactive binary selection from release assets
```

### Advanced Features

#### Real-time Progress Display
All downloads now feature professional progress bars:
```
⠋ [00:03:12] [████████████████████████████████████████] 45.2MiB/45.2MiB (00s) Downloaded opencode-linux-x64.zip
```
- Visual progress bar with percentage
- Real-time download speed and ETA
- Handles both known and unknown file sizes

#### Force Updates
Sometimes you may need to force an update even when on the latest version:
```bash
opencode-updater --force
```
Use cases:
- Reinstalling a corrupted binary
- Testing the update process
- Overriding version detection issues

#### Interactive Binary Selection
For releases with multiple binary options:
```bash
opencode-updater --bin
```
This presents an interactive menu to select from available binaries in the release.

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
- **Progress Display:** Real-time download progress with visual indicators and ETA
- **Archive Fallback:** Automatic fallback from zip to tar.gz format for compatibility

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
- **NEW:** Enhanced error handling with detailed error types for better security awareness.
- **NEW:** System version detection prevents accidental overwrites of existing installations.

## Dependencies

- `ureq` (for HTTP requests).
- `clap` (for command-line argument parsing).
- `dialoguer` (for interactive prompts).
- `zip` (for extracting zip archives).
- `tar` (for extracting tar archives).
- `flate2` (for gzip decompression).
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

### Development Workflow

This project uses a modern development workflow with:

- **Smart CI/CD**: Automated formatting fixes that preserve commit messages
- **Issue Templates**: Standardized bug reports and feature requests
- **PR Template**: Guided pull request submissions
- **Comprehensive Testing**: Unit and integration tests with mocked dependencies
- **Code Quality**: Automated Clippy linting and Rustfmt formatting

#### Running Tests
```bash
cargo test                    # Run all tests
cargo test -- --nocapture     # Run with output
cargo test integration_tests # Run only integration tests
```

#### Code Quality Checks
```bash
cargo clippy -- -D warnings   # Lint with warnings as errors
cargo fmt --check             # Check formatting
cargo fmt                     # Auto-format code
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Technical Details

### VersionManager Architecture

The `VersionManager` struct handles all version-related operations:

```rust
pub struct VersionManager {
    storage_dir: PathBuf,    // ~/.local/share/opencode-updater/
    versions_dir: PathBuf,   // ~/.local/share/opencode-updater/versions/
    cache_dir: PathBuf,      // ~/.local/share/opencode-updater/cache/
}
```

#### Storage Structure
```
~/.local/share/opencode-updater/
├── versions/           # Stored versions with metadata
│   ├── 1.0.73/
│   │   ├── opencode   # Binary executable
│   │   └── metadata.json # Version information
│   └── 1.0.72/
├── cache/             # GitHub API cache (1-hour TTL)
│   └── releases.json  # Cached release data
└── current            # Symlink to active version
```

#### VersionInfo Structure
Each stored version includes:
- Version string and tag name
- Release date and installation timestamp
- Download URL and checksum
- Release notes and metadata
- Installation path

### Error Handling

The application uses custom error types for better error reporting:

```rust
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
```

### Archive Extraction

Supports multiple archive formats with automatic fallback:
1. **ZIP**: Primary format (`opencode-linux-x64.zip`)
2. **TAR.GZ**: Fallback format (`opencode-linux-x64.tar.gz`)

The extractor automatically detects executable files and preserves permissions.

### System Integration

- **Version Detection**: Automatically detects system-installed versions
- **Symlink Management**: Maintains `current` symlink for active version
- **Permission Handling**: Ensures binaries have executable permissions (755)
- **Sudo Integration**: Uses system sudo for privileged operations

## Contact

- GitHub: [https://github.com/CodingInCarhartts](https://github.com/CodingInCarhartts)
- Email: yumlabs.team@gmail.com