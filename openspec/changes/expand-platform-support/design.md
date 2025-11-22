# Cross-Platform Architecture Design

## Current Architecture Analysis

### Existing Components
```
src/
├── main.rs          # CLI entry point, hardcoded Linux assumptions
└── lib.rs           # Core logic with Unix-specific code paths
```

### Current Limitations
1. **Hardcoded Paths**: `/usr/bin/opencode` throughout codebase
2. **Unix-Specific Operations**: `std::os::unix::fs::symlink`, `sudo` commands
3. **Single Architecture**: Only targets `x86_64-unknown-linux-gnu`
4. **Linux Asset Detection**: Hardcoded `opencode-linux-x64.zip` patterns
5. **System Installation**: Only supports sudo-based system-wide installation

## Proposed Architecture

### New Module Structure
```
src/
├── main.rs              # CLI entry point (updated)
├── lib.rs               # Core logic (refactored)
├── platform.rs          # Platform detection and configuration
├── installer.rs         # Cross-platform installation logic
└── config.rs            # Configuration management
```

### Core Abstractions

#### Platform Module
```rust
pub enum Platform {
    Linux { arch: Architecture },
    MacOS { arch: Architecture },
    Windows { arch: Architecture },
}

pub enum Architecture {
    X64,
    Arm64,
}

pub struct InstallPaths {
    pub system_binary: PathBuf,
    pub user_binary: PathBuf,
    pub storage_dir: PathBuf,
    pub config_dir: PathBuf,
}

impl Platform {
    pub fn detect() -> Self;
    pub fn asset_patterns(&self) -> Vec<&'static str>;
    pub fn default_paths(&self) -> InstallPaths;
    pub fn supports_system_install(&self) -> bool;
}
```

#### Installer Module
```rust
pub enum InstallMethod {
    SystemWide,  // sudo/UAC required
    UserLocal,   // no admin required
    Portable,    // extract to directory
}

pub struct Installer {
    platform: Platform,
    method: InstallMethod,
    custom_path: Option<PathBuf>,
}

impl Installer {
    pub fn new(platform: Platform, method: InstallMethod) -> Self;
    pub fn install(&self, source: &Path, target: &Path) -> Result<(), UpdaterError>;
    pub fn set_executable(&self, path: &Path) -> Result<(), UpdaterError>;
    pub fn create_symlink(&self, target: &Path, link: &Path) -> Result<(), UpdaterError>;
}
```

#### Configuration Module
```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub installation: InstallationConfig,
    pub platform: PlatformConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstallationConfig {
    pub method: InstallMethod,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlatformConfig {
    pub auto_detect: bool,
    pub force_architecture: Option<Architecture>,
}
```

## Platform-Specific Considerations

### Windows
- **Executable Handling**: No chmod needed, .exe extension
- **Symlinks**: Requires developer mode or admin privileges
- **Paths**: Use `%APPDATA%` or `%LOCALAPPDATA%` for user data
- **Installation**: Add to PATH or create shortcut
- **Asset Pattern**: `opencode-windows-x64.zip`

### macOS
- **Executable Handling**: chmod +x required
- **Symlinks**: Standard Unix symlinks work
- **Paths**: Use `/usr/local/bin` for system, `~/.local/bin` for user
- **Installation**: Standard Unix installation with homebrew compatibility
- **Asset Patterns**: `opencode-macos-x64.zip`, `opencode-macos-arm64.zip`

### Linux
- **Executable Handling**: chmod +x required
- **Symlinks**: Standard Unix symlinks work
- **Paths**: `/usr/bin` for system, `~/.local/bin` for user
- **Installation**: sudo for system, user permissions for local
- **Asset Patterns**: `opencode-linux-x64.zip`, `opencode-linux-arm64.zip`

## Migration Strategy

### Phase 1: Foundation
1. Create `platform.rs` with detection logic
2. Add `installer.rs` with cross-platform abstractions
3. Update `lib.rs` to use new abstractions
4. Maintain backward compatibility

### Phase 2: Integration
1. Update `main.rs` to use platform detection
2. Replace hardcoded paths with platform-specific logic
3. Add configuration system
4. Update CLI arguments

### Phase 3: Enhancement
1. Add multi-architecture support
2. Implement advanced installation methods
3. Add platform-specific optimizations
4. Comprehensive testing

## Backward Compatibility

### Existing Linux Behavior
- Default installation method remains system-wide with sudo
- Default path remains `/usr/bin/opencode` on Linux
- All existing CLI arguments continue to work
- Configuration file optional - defaults preserve current behavior

### Migration Path
- Existing users see no change in behavior
- New users get enhanced cross-platform experience
- Configuration can be added incrementally
- Platform detection is automatic and transparent

## Testing Strategy

### Unit Tests
- Platform detection logic
- Path resolution for each platform
- Installation method logic
- Configuration parsing

### Integration Tests
- End-to-end installation on each platform
- Asset detection and download
- Rollback functionality
- Configuration overrides

### CI/CD Testing
- Matrix builds for all platforms
- Automated testing on each OS
- Release asset verification
- Cross-platform compatibility checks

## Security Considerations

### Privilege Escalation
- Clear indication when admin privileges required
- Fallback to user-local installation when admin unavailable
- Secure temporary file handling across platforms

### Path Security
- Validate custom installation paths
- Prevent path traversal attacks
- Secure permission handling

### Download Security
- Maintain existing SHA-256 verification
- Platform-specific asset validation
- Secure temporary directory usage