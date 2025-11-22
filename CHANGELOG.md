# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-11-21

### Added
- **Comprehensive Version Management System**: Complete version backup and rollback functionality with local storage
- **New CLI Commands**: `--rollback`, `--list-versions`, `--changelog`, `--compare` for advanced version control
- **GitHub API Integration**: Direct access to release notes, version comparison, and release metadata
- **Real-time Download Progress**: Professional progress bars with ETA, download speed, and visual feedback using `indicatif`
- **Interactive Binary Selection**: `--bin` flag for interactive selection of available binaries from GitHub releases
- **Semantic Version Support**: Full semantic version parsing, comparison, and validation
- **Configurable Version Retention**: `--keep-versions` flag to control local version storage (default: 2)
- **System Version Detection**: Fallback logic to detect system-installed binary versions
- **Archive Format Fallback**: Automatic fallback from zip to tar.gz when zip is unavailable
- **GitHub Templates**: Standardized issue report and feature request templates for better contributions
- **Enhanced Error Handling**: Custom `UpdaterError` types with detailed error messages
- **Comprehensive Test Suite**: Extensive unit and integration tests with mocked network calls
- **Smart CI Workflow**: Auto-formatting with intelligent commit message preservation
- **Package Metadata**: Enhanced Cargo.toml with description, keywords, and repository URL

### Changed
- **Architecture Evolution**: Transformed from simple download tool to full version management system
- **HTTP Client Migration**: Replaced async `reqwest`/`tokio` with sync `ureq` for simplified codebase
- **Storage Structure**: Implemented organized local storage with versions/, cache/, and current symlink
- **Progress Display**: All downloads now show real-time progress with visual indicators
- **Dependency Updates**: Updated to latest stable versions for security and performance
- **Documentation**: Comprehensive README with badges, table of contents, and detailed usage examples

### Fixed
- **Runtime Conflicts**: Resolved Tokio runtime conflicts in test environment
- **Code Quality**: Fixed all Clippy warnings and implemented consistent formatting
- **Test Reliability**: Improved integration tests with proper mocking and cleanup
- **Error Messages**: Enhanced error reporting with specific error types and context
- **Permission Handling**: Fixed executable permission issues in extracted archives
- **CI Reliability**: Improved GitHub Actions workflow with better error handling

## [0.1.0] - 2025-11-16

## [0.1.0] - 2025-11-16

### Added
- Initial release of opencode-updater.
- Fetches the latest opencode Linux x64 binary from GitHub releases.
- Performs SHA-256 checksum verification against release checksums (if available).
- Extracts and installs the binary to `/usr/bin/opencode` with sudo.
- Handles errors and provides warnings for missing checksums or failed downloads.