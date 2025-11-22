# Project Context

## Purpose
A Rust CLI utility that updates the `opencode` binary to the latest GitHub release. Created to solve slow AUR package updates on Arch Linux and unreliable built-in upgrade commands. The tool provides advanced version management with automatic backups, rollback capabilities, and real-time download progress.

## Tech Stack
- **Language**: Rust (edition 2024)
- **CLI Framework**: clap 4.0 with derive features
- **HTTP Client**: ureq 3.1 with JSON support
- **Archive Handling**: zip 6.0, tar 0.4, flate2 1.0
- **Serialization**: serde 1.0 with derive features, serde_json 1.0
- **Security**: sha2 0.10 for checksum verification
- **UI/UX**: dialoguer 0.12 for interactive prompts, indicatif 0.18 for progress bars
- **File System**: tempfile 3.10, dirs 6.0 for cross-platform paths
- **Version Management**: semver 1.0, chrono 0.4 with serde features
- **Testing**: mockito 1.2 for HTTP mocking

## Project Conventions

### Code Style
- **Formatting**: Uses `rustfmt` with default settings
- **Linting**: `clippy` with warnings as errors (`-D warnings`)
- **Naming**: Rust conventions - snake_case for functions/variables, PascalCase for types
- **Error Handling**: Custom `UpdaterError` enum with detailed error types
- **Documentation**: Module-level docs explaining purpose and security considerations
- **Comments**: Minimal inline comments, focus on self-documenting code

### Architecture Patterns
- **Modular Design**: Separate modules for version management, downloading, and CLI handling
- **VersionManager**: Central struct for all version storage and operations
- **Error Types**: Custom enum with specific error variants for better error reporting
- **Configuration**: Command-line arguments using clap derive macros
- **Storage**: Structured local storage in `~/.local/share/opencode-updater/`
- **Caching**: GitHub API responses cached locally with 1-hour TTL

### Testing Strategy
- **Unit Tests**: Comprehensive tests for core functions in `tests/integration_tests.rs`
- **Mocked Network**: Uses `mockito` for HTTP request mocking
- **Integration Tests**: End-to-end testing of update process with fake archives
- **Test Coverage**: Tests for SHA-256 calculation, version parsing, archive extraction
- **CI Testing**: Automated testing on GitHub Actions for all PRs and pushes

### Git Workflow
- **Branching**: Main branch for releases, feature branches for development
- **Commits**: Conventional commit messages (not strictly enforced but encouraged)
- **CI/CD**: Smart formatting fixes that preserve commit messages
- **Releases**: Automated release builds with tar.gz and zip artifacts
- **PR Process**: Uses PR template for structured submissions

## Domain Context
- **Target Users**: Arch Linux users and others needing reliable opencode updates
- **Installation Target**: System-wide installation to `/usr/bin/opencode`
- **Privilege Requirements**: Requires sudo for system binary installation
- **Security Model**: Downloads from official GitHub releases, verifies SHA-256 checksums
- **Version Storage**: Local version management with automatic backup before updates
- **Fallback Behavior**: Supports both zip and tar.gz archive formats with automatic fallback

## Important Constraints
- **Security**: Downloads and installs executables with elevated privileges
- **Platform**: Linux-focused (specifically targets Linux x64 binaries)
- **Network**: Requires internet connection for GitHub API access
- **Permissions**: Must be run with sudo for installation operations
- **Storage**: Uses user data directory for version storage (~/.local/share/)
- **Dependencies**: Minimal external dependencies, all from crates.io

## External Dependencies
- **GitHub API**: https://api.github.com/repos/sst/opencode/releases
- **GitHub Releases**: Downloads binary assets from official releases
- **System Integration**: Uses system `sudo` command for privileged operations
- **File System**: Standard Unix file permissions and symlink management
