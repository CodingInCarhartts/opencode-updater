# Expand Platform Support

## Overview
Transform opencode-updater from a Linux-focused tool into a comprehensive cross-platform utility supporting Windows, macOS, and multiple Linux distributions with various architectures.

## Problem Statement
Currently, opencode-updater is hardcoded for Arch Linux with x64 architecture, using `/usr/bin/opencode` installation path and `sudo` commands. This limits usability to a small subset of potential users.

## Solution Overview
Implement platform detection, cross-platform installation logic, multi-architecture builds, and configuration system to support:
- **Windows**: x64 architecture with Windows-specific installation
- **macOS**: Intel (x64) and Apple Silicon (ARM64) support  
- **Linux**: Multiple distributions and architectures (x64, ARM64)
- **Installation Methods**: System-wide, user-local, and portable options

## Capabilities

### Platform Detection
- Auto-detect current platform and architecture
- Map platform to appropriate GitHub release assets
- Provide platform-specific default configurations

### Cross-Platform Installation
- Replace hardcoded `/usr/bin/opencode` with platform-appropriate paths
- Support multiple installation methods (system-wide, user-local, portable)
- Handle platform-specific file operations (symlinks, permissions)

### Multi-Architecture Builds
- Build and distribute binaries for all supported platforms/architectures
- Update CI/CD pipeline for matrix builds
- Generate platform-specific release assets

### Configuration System
- Allow users to customize installation preferences
- Support configuration files and CLI overrides
- Maintain backward compatibility with existing behavior

## Relationships Between Capabilities

1. **Platform Detection** → Foundation for all other capabilities
2. **Cross-Platform Installation** → Depends on platform detection
3. **Multi-Architecture Builds** → Parallel development with installation logic
4. **Configuration System** → Enhances all capabilities with user customization

## Implementation Phases

### Phase 1: Core Platform Infrastructure
- Platform detection module
- Basic cross-platform installation paths
- Replace hardcoded Linux-specific code

### Phase 2: Enhanced Installation Logic  
- Multiple installation methods
- Platform-specific file operations
- Configuration system foundation

### Phase 3: Build & Distribution
- Multi-architecture CI/CD pipeline
- Platform-specific release assets
- Cross-platform testing

### Phase 4: Documentation & Polish
- Platform-specific installation guides
- Package manager integration research
- User experience improvements

## Success Criteria
- Users can install and use opencode-updater on Windows, macOS, and Linux
- All existing functionality preserved for Linux users
- Automated builds for all supported platforms
- Comprehensive documentation for each platform
- Backward compatibility maintained