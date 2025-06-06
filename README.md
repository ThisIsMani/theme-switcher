# Theme Switcher

A macOS background service that monitors system theme changes (light/dark mode) and executes custom scripts in response.

## Features

- Real-time monitoring of macOS theme changes using native APIs
- Zero CPU usage when idle (event-driven)
- Execute custom shell scripts when switching to light/dark themes
- Execute Lua scripts with built-in API for theme handling
- Quiet mode for background operation
- Environment variables passed to scripts
- IPC server for integration with other applications (e.g., Neovim)

## Installation

```bash
cargo build --release
cp target/release/theme-switcher /usr/local/bin/
```

## Usage

### Basic usage (with logging)
```bash
theme-switcher
```

### Enable IPC server for Neovim integration
```bash
theme-switcher --ipc
```

### Run scripts on theme changes
```bash
theme-switcher --light-script ./light_theme.sh --dark-script ./dark_theme.sh
```

### Run in quiet mode (no output except errors)
```bash
theme-switcher --quiet --light-script ./light_theme.sh --dark-script ./dark_theme.sh
```

### Run a script on any theme change
```bash
theme-switcher --any-script ./theme_changed.sh
```

### Run Lua scripts
```bash
theme-switcher --lua-light ./light.lua --lua-dark ./dark.lua

# Or run a single Lua script on any change
theme-switcher --lua-any ./theme_change.lua
```

### Use a configuration file
```bash
# Use default config location: ~/.config/theme-switcher/config.toml
theme-switcher

# Or specify a custom config file
theme-switcher --config ~/my-theme-config.toml
```

## Script Environment Variables

Shell scripts receive the following environment variables:
- `THEME_SWITCHER_THEME`: Current theme (`light` or `dark`)
- `THEME_SWITCHER_THEME_UPPER`: Current theme in uppercase (`LIGHT` or `DARK`)

## Lua Script API

Lua scripts have access to global variables and a `theme_switcher` module:

### Global Variables
- `THEME`: Current theme string (`"light"` or `"dark"`)
- `THEME_UPPER`: Current theme in uppercase (`"LIGHT"` or `"DARK"`)
- `IS_DARK`: Boolean indicating if dark theme is active
- `IS_LIGHT`: Boolean indicating if light theme is active

### theme_switcher Module Functions
- `theme_switcher.execute(cmd)`: Execute a shell command and return results
  - Returns a table with: `stdout`, `stderr`, `success`
- `theme_switcher.log(msg)`: Log an informational message
- `theme_switcher.log_error(msg)`: Log an error message
- `theme_switcher.current_theme`: Current theme string
- `theme_switcher.is_dark`: Boolean for dark theme
- `theme_switcher.is_light`: Boolean for light theme

## Configuration File (TOML)

Theme-switcher supports TOML configuration files for managing multiple scripts:

```toml
[general]
quiet = false  # Run in quiet mode
# log_file = "/path/to/logfile.log"  # Optional log file

[scripts]
# Shell scripts - can specify multiple scripts per event
light = ["~/scripts/light1.sh", "~/scripts/light2.sh"]
dark = ["~/scripts/dark1.sh", "~/scripts/dark2.sh"]
any = ["~/scripts/any-change.sh"]

[lua_scripts]
# Lua scripts - can specify multiple scripts per event
light = ["~/scripts/light.lua"]
dark = ["~/scripts/dark.lua"]
any = ["~/scripts/theme-change.lua"]
```

## Example Scripts

See the `examples/` directory for:
- Shell scripts for basic theme changes
- Lua scripts for:
  - Changing VS Code themes
  - Updating terminal colors (iTerm2, Terminal.app)
  - Sending system notifications
  - Complex configuration file updates
- Integration scripts:
  - `zellij_theme.sh` / `zellij_theme.lua` - Zellij terminal multiplexer
  - `claude_code_theme.sh` / `claude_code_theme.lua` - Claude Code editor
- TOML configuration examples:
  - `theme-switcher.toml` - Full example with comments
  - `minimal-config.toml` - Simple configuration
  - `advanced-config.toml` - Complex multi-app setup
  - `nvim-config.lua` - Neovim plugin configuration

## Neovim Integration

A Neovim plugin is included in `nvim-theme-switcher/` that automatically syncs your editor theme with the system theme.

### Quick Setup

1. Start theme-switcher with IPC enabled:
   ```bash
   theme-switcher --ipc
   ```

2. Add the plugin to your Neovim config:
   ```lua
   require('theme-switcher').setup({
     schemes = {
       light = "tokyonight-day",
       dark = "tokyonight-night"
     }
   })
   ```

See `nvim-theme-switcher/README.md` for detailed setup instructions.

## Running as a Background Service

To run theme-switcher as a macOS LaunchAgent:

1. Create a plist file at `~/Library/LaunchAgents/com.yourdomain.theme-switcher.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.yourdomain.theme-switcher</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/theme-switcher</string>
        <string>--config</string>
        <string>/Users/yourusername/.config/theme-switcher/config.toml</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

2. Load the service:
```bash
launchctl load ~/Library/LaunchAgents/com.yourdomain.theme-switcher.plist
```

## Building from Source

```bash
git clone https://github.com/yourusername/theme-switcher
cd theme-switcher
cargo build --release
```

## License

MIT