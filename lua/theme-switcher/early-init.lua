-- Early initialization module for theme-switcher
-- Use this in your init.lua BEFORE any colorscheme commands to prevent flash
-- Example: require('theme-switcher.early-init').setup()

local M = {}

function M.setup(opts)
  opts = opts or {}
  
  -- Get system theme as early as possible
  local function get_theme()
    -- Try socket first
    local runtime_dir = vim.env.XDG_RUNTIME_DIR or ""
    local home_dir = vim.env.HOME or ""
    
    local sockets = {
      runtime_dir .. "/theme-switcher.sock",
      home_dir .. "/.local/run/theme-switcher.sock",
      "/tmp/theme-switcher.sock",
    }
    
    for _, socket in ipairs(sockets) do
      if vim.fn.filereadable(socket) == 1 then
        local cmd = string.format("timeout 0.05 nc -U %s < /dev/null 2>/dev/null | head -n1", socket)
        local theme = vim.fn.system(cmd):match("^%s*(.-)%s*$")
        if theme == "light" or theme == "dark" then
          return theme
        end
      end
    end
    
    -- macOS fallback
    if vim.fn.has("mac") == 1 then
      local output = vim.fn.system("defaults read -g AppleInterfaceStyle 2>/dev/null")
      return (vim.v.shell_error == 0 and output:match("Dark")) and "dark" or "light"
    end
    
    return opts.default or "dark"
  end
  
  local theme = get_theme()
  vim.opt.background = theme
  
  -- Apply colorscheme if provided
  if opts.schemes and opts.schemes[theme] then
    local scheme = opts.schemes[theme]
    if type(scheme) == "string" then
      vim.cmd.colorscheme(scheme)
    elseif type(scheme) == "table" and scheme.colorscheme then
      vim.cmd.colorscheme(scheme.colorscheme)
    end
  end
  
  -- Store for later use
  vim.g.theme_switcher_initial_theme = theme
  
  return theme
end

return M