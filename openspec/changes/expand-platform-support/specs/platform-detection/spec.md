# Platform Detection Specification

## ADDED Requirements

### Requirement: Auto-Detect Current Platform and Architecture
#### Scenario:
When a user runs `opencode-updater` on any supported platform (Windows, macOS, Linux), the application automatically detects the current operating system and CPU architecture without requiring manual configuration.

#### Acceptance Criteria:
- Detects Windows, macOS, and Linux operating systems
- Identifies x64 and ARM64 architectures
- Returns Platform enum with appropriate variant
- Handles detection errors gracefully with fallback to Linux x64

### Requirement: Map Platform to GitHub Release Asset Patterns
#### Scenario:
When the application needs to download the appropriate opencode binary, it uses the detected platform to determine which GitHub release assets to search for, supporting multiple archive formats.

#### Acceptance Criteria:
- Returns ordered list of asset name patterns for each platform
- Windows: `["opencode-windows-x64.zip"]`
- macOS Intel: `["opencode-macos-x64.zip"]`
- macOS Apple Silicon: `["opencode-macos-arm64.zip"]`
- Linux x64: `["opencode-linux-x64.zip", "opencode-linux-x64.tar.gz"]`
- Linux ARM64: `["opencode-linux-arm64.zip", "opencode-linux-arm64.tar.gz"]`

### Requirement: Provide Platform-Specific Default Installation Paths
#### Scenario:
When the application needs to determine where to install the opencode binary, it provides platform-appropriate default paths for both system-wide and user-local installation methods.

#### Acceptance Criteria:
- Windows system: `C:\Program Files\opencode\opencode.exe`
- Windows user: `%APPDATA%\opencode\opencode.exe`
- macOS system: `/usr/local/bin/opencode`
- macOS user: `~/.local/bin/opencode`
- Linux system: `/usr/bin/opencode`
- Linux user: `~/.local/bin/opencode`

### Requirement: Determine System Installation Support
#### Scenario:
When the application needs to know whether the current platform supports system-wide installation requiring elevated privileges, it provides this information to guide installation method selection.

#### Acceptance Criteria:
- Returns true for all platforms (all support system installation)
- Windows requires UAC elevation
- macOS requires sudo
- Linux requires sudo
- Used to determine if admin privileges should be requested

## MODIFIED Requirements

### Requirement: Asset Detection Logic
#### Scenario:
When searching for the appropriate binary asset in GitHub releases, the application uses platform-specific patterns instead of hardcoded Linux-only patterns.

#### Acceptance Criteria:
- Replace hardcoded `opencode-linux-x64.zip` with dynamic patterns
- Use platform detection to determine search patterns
- Maintain fallback behavior for multiple archive formats
- Preserve existing error handling for missing assets

## Implementation Details

### Platform Detection Algorithm
```rust
impl Platform {
    pub fn detect() -> Self {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        
        let platform = match os {
            "windows" => Platform::Windows { arch: Architecture::from_str(arch) },
            "macos" => Platform::MacOS { arch: Architecture::from_str(arch) },
            "linux" => Platform::Linux { arch: Architecture::from_str(arch) },
            _ => Platform::Linux { arch: Architecture::X64 }, // fallback
        };
        
        platform
    }
}
```

### Asset Pattern Mapping
```rust
impl Platform {
    pub fn asset_patterns(&self) -> Vec<&'static str> {
        match self {
            Platform::Windows { arch: Architecture::X64 } => {
                vec!["opencode-windows-x64.zip"]
            },
            Platform::MacOS { arch: Architecture::X64 } => {
                vec!["opencode-macos-x64.zip"]
            },
            Platform::MacOS { arch: Architecture::Arm64 } => {
                vec!["opencode-macos-arm64.zip"]
            },
            Platform::Linux { arch: Architecture::X64 } => {
                vec!["opencode-linux-x64.zip", "opencode-linux-x64.tar.gz"]
            },
            Platform::Linux { arch: Architecture::Arm64 } => {
                vec!["opencode-linux-arm64.zip", "opencode-linux-arm64.tar.gz"]
            },
        }
    }
}
```

### Path Resolution
```rust
impl Platform {
    pub fn default_paths(&self) -> InstallPaths {
        match self {
            Platform::Windows { .. } => InstallPaths {
                system_binary: PathBuf::from(r"C:\Program Files\opencode\opencode.exe"),
                user_binary: dirs::data_dir().unwrap().join("opencode").join("opencode.exe"),
                storage_dir: dirs::data_dir().unwrap().join("opencode-updater"),
                config_dir: dirs::config_dir().unwrap().join("opencode-updater"),
            },
            Platform::MacOS { .. } | Platform::Linux { .. } => InstallPaths {
                system_binary: PathBuf::from("/usr/local/bin/opencode"),
                user_binary: dirs::home_dir().unwrap().join(".local/bin/opencode"),
                storage_dir: dirs::home_dir().unwrap().join(".local/share/opencode-updater"),
                config_dir: dirs::home_dir().unwrap().join(".config/opencode-updater"),
            },
        }
    }
}
```

## Testing Requirements

### Unit Tests
- Test platform detection on all supported OS/architecture combinations
- Verify asset pattern mapping for each platform
- Validate path resolution returns valid paths for each platform
- Test error handling for unsupported platforms

### Integration Tests
- Mock different platform environments and verify behavior
- Test asset detection with platform-specific patterns
- Verify installation path selection works correctly

### Cross-Platform Tests
- Run tests on Windows, macOS, and Linux
- Verify platform detection works on actual hardware
- Test with different architectures (x64, ARM64)