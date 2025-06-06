-- Early initialization for theme-switcher
-- This runs before init.lua to prevent theme flash

local function get_system_theme()
  -- Try to read from the socket synchronously
  local runtime_dir = vim.env.XDG_RUNTIME_DIR or ""
  local home_dir = vim.env.HOME or ""
  local temp_dir = "/tmp"
  
  local possible_paths = {
    runtime_dir .. "/theme-switcher.sock",
    home_dir .. "/.local/run/theme-switcher.sock",
    temp_dir .. "/theme-switcher.sock",
  }
  
  for _, socket_path in ipairs(possible_paths) do
    if vim.fn.filereadable(socket_path) == 1 then
      -- Try to get current theme from socket using timeout
      local output = vim.fn.system(string.format("timeout 0.1 nc -U %s < /dev/null 2>/dev/null | head -n1", socket_path))
      if vim.v.shell_error == 0 and output ~= "" then
        local theme = output:match("^%s*(.-)%s*$") -- trim whitespace
        if theme == "light" or theme == "dark" then
          return theme
        end
      end
    end
  end
  
  -- Fallback: try to detect from system (macOS specific)
  if vim.fn.has("mac") == 1 then
    local cmd = "defaults read -g AppleInterfaceStyle 2>/dev/null"
    local output = vim.fn.system(cmd)
    if vim.v.shell_error == 0 and output:match("Dark") then
      return "dark"
    else
      return "light"
    end
  end
  
  -- Default to dark if we can't determine
  return "dark"
end

-- Apply early theme to prevent flash
local theme = get_system_theme()
vim.opt.background = theme

-- Store the detected theme for the main plugin
vim.g.theme_switcher_initial_theme = theme