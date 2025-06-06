# nvim-theme-switcher

A Neovim plugin that automatically syncs your editor theme with the system theme using the `theme-switcher` daemon.

## Requirements

- Neovim 0.7+
- `theme-switcher` daemon running with `--ipc` flag
- `nc` (netcat) command available

## Installation

### Using [lazy.nvim](https://github.com/folke/lazy.nvim)

```lua
{
  'your-username/nvim-theme-switcher',
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
  'your-username/nvim-theme-switcher',
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

## Troubleshooting

1. **Plugin can't connect**: Make sure `theme-switcher` is running with `--ipc` flag
2. **Theme not changing**: Check if the colorscheme names are correct and installed
3. **Socket not found**: The plugin looks for the socket in standard locations. Check if the daemon is running

## License

MIT