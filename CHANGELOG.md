# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Issue and PR templates in `.github/` for standardized bug reports, feature requests, and pull requests
- `--bin` flag for interactive selection of available binaries from GitHub releases
- Package metadata in Cargo.toml: description, keywords, and repository URL for better discoverability
- Comprehensive documentation comments (crate-level and function/struct docs) for improved code readability and API usability
- Unit test for `find_executable_binary` function to verify binary detection in extracted archives
- Integration test (`test_run_update`) for the main update process, including mocked network calls (GitHub API, checksums, downloads) and temp directory file operations for end-to-end validation

### Changed
- Enhanced README.md with badges (CI, license, Rust version, project version), table of contents, and updated contributing link
- Updated .gitignore for better file exclusions
- Updated dependencies to latest versions: `dialoguer` v0.12.0, `zip` v6.0.0, and transitive updates for improved security and performance
- Replaced asynchronous HTTP client (`reqwest` with `tokio`) with synchronous client (`ureq`) to resolve Tokio runtime conflicts in tests and simplify the codebase
- Updated dependencies: added `ureq` with JSON features, removed `reqwest` and `tokio` for sync operations

### Fixed
- Multiple updates to GitHub Actions CI workflow for improved reliability
- Code formatting with `rustfmt`
- Clippy warnings for collapsible `if` statements and needless borrows in `src/main.rs` and `src/lib.rs`
- Compilation and runtime issues in integration tests (zip file permissions, async runtime conflicts)

## [0.1.0] - 2025-11-16

### Added
- Initial release of opencode-updater.
- Fetches the latest opencode Linux x64 binary from GitHub releases.
- Performs SHA-256 checksum verification against release checksums (if available).
- Extracts and installs the binary to `/usr/bin/opencode` with sudo.
- Handles errors and provides warnings for missing checksums or failed downloads.