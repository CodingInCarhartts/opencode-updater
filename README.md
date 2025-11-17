# opencode-updater

A simple Rust utility to update the `opencode` binary to the latest version from GitHub releases. This tool was created because the AUR package on Arch Linux didn't update quickly enough, and the built-in upgrade command in `opencode` wasn't working reliably.

## Purpose

- Downloads the latest `opencode` binary (Linux x64) from the official GitHub releases.
- Extracts and installs it to `/usr/bin/opencode` using `sudo`.
- Ensures the binary is executable.
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

## Usage

Simply run the binary. It will:
- Fetch the latest release info from GitHub.
- Download the `opencode-linux-x64.zip` asset.
- Extract the `opencode` binary to a temporary directory.
- Move it to `/usr/bin/opencode` (requires `sudo`).
- Make it executable.

Example output:
```
Updated opencode to latest version.
```

## Security Notes

- This tool downloads and installs binaries directly. Always verify the source (GitHub releases) and consider the risks of running unverified executables.
- Requires `sudo` for system-wide installation—use at your own risk.
- Performs SHA-256 checksum verification against GitHub release checksums when available.
- Checksums provide integrity protection but not authenticity; still trust GitHub as the source.

## Dependencies

- `reqwest` (for HTTP requests).
- `tokio` (for async runtime).
- `zip` (for extracting archives).
- `serde_json` (for parsing GitHub API responses).
- `tempfile` (for temporary directories).
- `sha2` (for SHA-256 checksum verification).

## Contributing

Feel free to open issues or pull requests on GitHub.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

- GitHub: [https://github.com/CodingInCarhartts](https://github.com/CodingInCarhartts)
- Email: yumlabs.team@gmail.com