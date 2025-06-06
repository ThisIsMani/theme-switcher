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

### Dependencies
- `cocoa` - macOS Cocoa API bindings
- `objc` - Objective-C runtime bindings
- `objc_id` - Safe Objective-C object management
- `lazy_static` - Runtime class registration