# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`theme-switcher` is a Rust-based background service that monitors macOS system theme changes (light/dark mode) in real-time using native Key-Value Observing (KVO).

## Commands

### Build
```bash
cargo build
cargo build --release
```

### Run
```bash
cargo run
cargo run -- --ipc  # Enable IPC server for Neovim integration
```

### Test
```bash
cargo test
```

### Check code (without building)
```bash
cargo check
```

### Format code
```bash
cargo fmt
```

### Lint with Clippy
```bash
cargo clippy
```

## Architecture

### Core Structure
- `src/lib.rs` - Public API with `Theme` enum and `ThemeMonitor` trait
- `src/bin/theme-switcher.rs` - Binary entry point (minimal, just calls lib)
- `src/platform/macos.rs` - macOS-specific implementation using Cocoa APIs
- `src/ipc.rs` - IPC server for external integrations (Unix domain socket)
- `src/async_runtime.rs` - Tokio runtime for IPC mode
- `src/handlers.rs` - Theme change handlers (scripts, logging, IPC)
- `src/lua_handler.rs` - Lua script execution with theme_switcher API

### Key Components

1. **ThemeMonitor Trait** - Platform-agnostic interface:
   - `start()` - Begin monitoring theme changes
   - `stop()` - Stop monitoring and clean up resources
   - `get_current_theme()` - Get current system theme

2. **MacOSThemeMonitor** - Native macOS implementation:
   - Uses KVO to observe `NSApp.effectiveAppearance`
   - Zero CPU usage when idle (event-driven)
   - Instant theme change detection
   - Custom Objective-C bridge class for KVO callbacks

3. **IPC Server** - Unix domain socket server:
   - Broadcasts theme changes to connected clients
   - Supports multiple simultaneous connections
   - Located at: `$XDG_RUNTIME_DIR/theme-switcher.sock` or `/tmp/theme-switcher.sock`
   - Protocol: Sends theme name (`light` or `dark`) followed by newline

4. **Neovim Plugin** - Located in `lua/theme-switcher/`:
   - Connects to IPC socket using `nc` (netcat)
   - Automatically syncs Neovim colorscheme with system theme
   - Configurable theme mappings and callbacks
   - Repository can be used directly as a Neovim plugin

### Dependencies
- `cocoa` - macOS Cocoa API bindings
- `objc` - Objective-C runtime bindings
- `objc_id` - Safe Objective-C object management
- `lazy_static` - Runtime class registration
- `tokio` - Async runtime for IPC server
- `clap` - Command line argument parsing
- `mlua` - Lua scripting support
- `serde` + `toml` - Configuration file support
- `dirs` - Platform-specific directory paths