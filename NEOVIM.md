# Neovim Integration

This repository includes a Neovim plugin that automatically syncs your editor theme with the system theme using the `theme-switcher` daemon.

## Requirements

- Neovim 0.7+
- `theme-switcher` daemon running with `--ipc` flag
- `nc` (netcat) command available

## Installation

### Using [lazy.nvim](https://github.com/folke/lazy.nvim)

```lua
{
  'ThisIsMani/theme-switcher',
  config = function()
    require('theme-switcher').setup({
      schemes = {
        light = "gruvbox-light",
        dark = "gruvbox-dark"
      }
    })
  end
}
```

### Using [packer.nvim](https://github.com/wbthomason/packer.nvim)

```lua
use {
  'ThisIsMani/theme-switcher',
  config = function()
    require('theme-switcher').setup({
      schemes = {
        light = "gruvbox-light", 
        dark = "gruvbox-dark"
      }
    })
  end
}
```

## Configuration

```lua
require('theme-switcher').setup({
  -- Theme schemes
  schemes = {
    light = "gruvbox-light",  -- Simple string format
    dark = {                  -- Or detailed format
      colorscheme = "gruvbox-dark",
      background = "dark",    -- Optional, defaults to theme name
      lightline = "gruvbox",  -- Lightline theme (if using lightline)
    }
  },
  
  -- Lualine themes (if using lualine)
  lualine = {
    light = "gruvbox_light",
    dark = "gruvbox_dark"
  },
  
  -- Lightline configuration (if using lightline)
  lightline_loaders = {
    -- Add paths to lightline theme files if needed
  },
  
  -- Callback function on theme change
  onchange = function(theme)
    -- Custom logic on theme change
    print("Theme changed to: " .. theme)
  end
})
```

## Usage

The plugin automatically connects to the `theme-switcher` daemon and syncs themes. No manual intervention needed!

### Commands

- `:lua require('theme-switcher').toggle()` - Toggle between light and dark themes
- `:lua require('theme-switcher').set_theme('light')` - Manually set theme
- `:lua require('theme-switcher').stop()` - Stop the plugin

### API

```lua
local theme_switcher = require('theme-switcher')

-- Get current theme
local current = theme_switcher.get_theme()

-- Set theme manually
theme_switcher.set_theme('dark')

-- Toggle theme
theme_switcher.toggle()
```

## Running the theme-switcher daemon

Make sure to run the `theme-switcher` daemon with the `--ipc` flag:

```bash
theme-switcher --ipc
```

Or add it to your config file:

```toml
# ~/.config/theme-switcher/config.toml
[general]
ipc = true
```

## Preventing Theme Flash on Startup

By default, Neovim may briefly show its default colorscheme before the plugin loads. To prevent this:

### Option 1: Use Early Initialization (Recommended)

Add this to the **very beginning** of your `init.lua`:

```lua
-- Put this BEFORE any colorscheme commands
require('theme-switcher.early-init').setup({
  schemes = {
    light = "gruvbox-light",
    dark = "gruvbox-dark"
  },
  default = "dark"  -- Fallback if detection fails
})

-- Then setup the main plugin normally later
require('theme-switcher').setup({
  schemes = {
    light = "gruvbox-light",
    dark = "gruvbox-dark"
  }
})
```

### Option 2: Automatic Early Detection

The plugin automatically includes early detection in `plugin/theme-switcher.lua` which runs before `init.lua`. This provides basic flash prevention without configuration.

### Option 3: Manual Background Setting

If you prefer manual control, set the background early in your config:

```lua
-- At the very start of init.lua
vim.opt.background = "dark"  -- or detect it yourself
```

## Troubleshooting

1. **Plugin can't connect**: Make sure `theme-switcher` is running with `--ipc` flag
2. **Theme not changing**: Check if the colorscheme names are correct and installed
3. **Socket not found**: The plugin looks for the socket in standard locations. Check if the daemon is running
4. **Theme flash on startup**: Use the early initialization method described above

## License

MIT