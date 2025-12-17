# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**WinShell** (project name: `vibe_winshell`) is a comprehensive system monitoring tool built in Rust that provides a unified GUI interface for monitoring processes, file handles, and network connections on Windows and Linux systems. It serves as an alternative to Task Manager with enhanced features like CSV export, real-time graphs, and cross-platform support.

**Note**: The Cargo package name is `vibe_winshell` but the application is referred to as "WinShell" in documentation.

## Architecture & Code Structure

**GUI Framework**: egui/eframe (immediate mode GUI)
**Language**: Rust (100% safe code)
**Pattern**: Component-based architecture with auto-generated modules

The codebase uses SplitRS tool to split monolithic code into organized modules under `src/ws/`. The main structure includes:
- `processmanagerapp_type.rs` - Main app state struct
- `processmanagerapp_traits.rs` - Trait implementations (App trait)
- `processmanagerapp_*_group.rs` - Feature-specific modules (40+ files)
- `types.rs` - Data structures and type definitions
- `functions.rs` - Application entry and setup

## Common Development Commands

```bash
# Development build and run
cargo build
cargo run

# Optimized release build
cargo build --release
cargo run --release

# Code quality and checking
cargo check          # Quick syntax/type checking
cargo clippy          # Lint checking
cargo fmt            # Format code
cargo test           # Run tests (if any exist)

# Dependency management
cargo update         # Update dependencies
cargo tree           # Show dependency tree

# Build maintenance
cargo clean          # Clean build artifacts

# The application binary will be at:
# Linux: ./target/release/winshell
# Windows: ./target/release/winshell.exe
```

## Development Workflow

**Code Generation**: The module structure is auto-generated using SplitRS. When adding features:
1. Work in the main application code
2. Regenerate modules using SplitRS
3. Run the appropriate fix script from `scripts/fix_fn2pub*.py` to fix visibility issues

**Build Scripts**:
- `scripts/fix_fn2pub*.py` - Fix function visibility errors after code splitting (numbers refer to different module files)
- `create_icon.ps1` - Generate application icons (Windows PowerShell script)

**Icon Generation** (Windows only):
```powershell
# Run from project root
.\create_icon.ps1
```

**After Code Splitting**:
Run fix scripts to make functions public across modules:
```bash
# Apply fixes to all modules
for script in scripts/fix_fn2pub*.py; do python "$script"; done
```

## Key Data Structures

- `ProcessInfo` - Process details (PID, memory, CPU, parent PID, etc.)
- `FileHandle` - Open file information with process context
- `NetworkConnection` - Active network connections with metadata
- `ProcessManagerApp` - Central application state manager

## Platform-Specific Considerations

- **Windows**: Full implementation with native API integration via `windows` crate. Requires Windows SDK for resource compilation.
- **Linux**: Uses `procfs` crate for file and network monitoring. Requires read access to `/proc` filesystem.
- **macOS**: Basic process monitoring only (file/network features not implemented)

**Linux Setup**:
- Ensure `/proc` filesystem is mounted (standard on most distributions)
- May require elevated permissions for some process information

**Windows Setup**:
- Windows SDK automatically installed with Rust toolchain
- Resource compilation handled by `build.rs` and `winres` crate
- Icon embedded via Windows resource compilation

**Cross-Compilation**:
```bash
# Cross-compile from Linux to Windows
cargo build --target=x86_64-pc-windows-gnu

# Cross-compile from Windows to Linux
cargo build --target=x86_64-unknown-linux-gnu
```

## Adding New Features

1. Add to appropriate `processmanagerapp_*_group.rs` file based on feature type
2. Update type definitions in `types.rs` if needed
3. Follow existing naming conventions (snake_case for functions, PascalCase for types)
4. Consider cross-platform implications - use conditional compilation for platform-specific code
5. Module files follow pattern: `processmanagerapp_<feature>_group.rs`

## UI/UX Patterns

- Tabbed interface: Processes, Files, Network views
- Real-time updates with configurable refresh intervals
- Search/filter functionality across all views
- CSV export capability for all data types
- Dark/Light theme support
- Color-coded CPU usage and connection states

## Performance Characteristics

- Memory usage: ~5-10 MB RAM
- CPU usage: <1% idle, ~2-5% during refresh
- Release builds are heavily optimized with LTO enabled
- Single codegen unit for maximum optimization

## Testing

No formal test suite is present. Manual testing via `cargo run` is the primary method. Test functionality on both Windows and Linux when adding new features.

**Manual Testing Guidelines**:
- Test process monitoring (basic functionality should work on all platforms)
- Test file handle monitoring (Linux only, Windows shows placeholders)
- Test network connection monitoring (Linux only, Windows shows placeholders)
- Verify CSV export produces valid files
- Test theme switching (dark/light modes)
- Test auto-refresh with different intervals
- Test process termination with various process types

**Debugging**:
```bash
# Run with extra logging (if implemented)
RUST_LOG=debug cargo run

# Use debugger
rust-gdb target/debug/winshell  # Linux
rust-lldb target/debug/winshell # macOS/Windows
```

## Troubleshooting

**Common Issues**:
- **Permission denied**: May need elevated privileges for process termination or system information
- **Build failures on Windows**: Ensure Windows SDK is installed with Rust toolchain
- **Module visibility errors**: Run the appropriate `fix_fn2pub*.py` script after code changes
- **Missing file/network data on Windows**: These features are currently placeholders, full implementation is Linux-only
- **egui window not appearing**: Check that graphics drivers are up to date

**Performance Issues**:
- Reduce refresh interval if CPU usage is high
- Close application when monitoring large numbers of processes for extended periods
- Use release build for better performance

## Dependencies Architecture

**Core Dependencies**:
- `eframe`/`egui` (0.29) - Immediate mode GUI framework
- `sysinfo` (0.32) - Cross-platform system information
- `egui_plot` (0.29) - Real-time graphing
- `chrono` (0.4) - Time handling and formatting
- `csv` (1.3) - Data export functionality
- `dirs` (5.0) - Directory utilities

**Platform-Specific**:
- `procfs` (0.16) - Linux /proc filesystem access
- `windows` (0.58) - Windows API bindings for system calls

## Build Configuration

The project uses optimized release builds:
- `opt-level = 3` - Maximum optimization
- `lto = true` - Link Time Optimization
- `codegen-units = 1` - Single codegen unit for better optimization
- Windows resources compiled via `build.rs` and `winres`