# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Issue and PR templates in `.github/` for standardized bug reports, feature requests, and pull requests

### Changed
- Enhanced README.md with badges (CI, license, Rust version, project version), table of contents, and updated contributing link
- Updated .gitignore for better file exclusions

### Fixed
- Multiple updates to GitHub Actions CI workflow for improved reliability
- Code formatting with `rustfmt`

## [0.1.0] - 2025-11-16

### Added
- Initial release of opencode-updater.
- Fetches the latest opencode Linux x64 binary from GitHub releases.
- Performs SHA-256 checksum verification against release checksums (if available).
- Extracts and installs the binary to `/usr/bin/opencode` with sudo.
- Handles errors and provides warnings for missing checksums or failed downloads.