# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-11-16

### Added
- Initial release of opencode-updater.
- Fetches the latest opencode Linux x64 binary from GitHub releases.
- Performs SHA-256 checksum verification against release checksums (if available).
- Extracts and installs the binary to `/usr/bin/opencode` with sudo.
- Handles errors and provides warnings for missing checksums or failed downloads.