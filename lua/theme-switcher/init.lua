-- Neovim Theme Switcher Plugin
-- Connects to theme-switcher daemon via Unix socket

local M = {}

-- State management
local state = {
  initialized = false,
  connected = false,
  socket_path = nil,
  job_id = nil,
  config = {},
  current_theme = nil,
}

-- Utility function to trim whitespace
local function trim(s)
  return s:match'^()%s*$' and '' or s:match'^%s*(.*%S)'
end

-- Default configuration
local default_config = {
  schemes = {
    light = "default",
    dark = "default"
  },
  lightline_loaders = {},
  lualine = {
    light = nil,
    dark = nil,
  },
  onchange = nil,
}

-- Find socket path
local function find_socket_path()
  local runtime_dir = vim.env.XDG_RUNTIME_DIR or ""
  local home_dir = vim.env.HOME or ""
  local temp_dir = "/tmp"
  
  local possible_paths = {
    runtime_dir .. "/theme-switcher.sock",
    home_dir .. "/.local/run/theme-switcher.sock",
    temp_dir .. "/theme-switcher.sock",
  }
  
  for _, path in ipairs(possible_paths) do
    if vim.fn.filereadable(path) == 1 then
      return path
    end
  end
  
  return nil
end

-- Apply theme configuration
local function apply_theme(theme)
  local config = state.config
  local scheme = config.schemes[theme] or {}
  
  -- Handle string shorthand
  if type(scheme) == "string" then
    scheme = { colorscheme = scheme }
  end
  
  local colorscheme = scheme.colorscheme
  local background = scheme.background or theme
  
  -- Set background
  vim.opt.background = background
  
  -- Set colorscheme
  if colorscheme and colorscheme ~= "default" then
    local ok, err = pcall(vim.cmd, "colorscheme " .. colorscheme)
    if not ok then
      vim.notify("Failed to set colorscheme: " .. err, vim.log.levels.ERROR)
    end
  end
  
  -- Update lightline if present
  if vim.g.loaded_lightline == 1 then
    local lightline_theme = scheme.lightline
    if lightline_theme then
      vim.g.lightline = vim.g.lightline or {}
      vim.g.lightline.colorscheme = lightline_theme
      
      -- Reload lightline
      vim.cmd([[
        call lightline#init()
        call lightline#colorscheme()
        call lightline#update()
      ]])
    end
  end
  
  -- Update lualine if present
  local ok, lualine = pcall(require, 'lualine')
  if ok and config.lualine[theme] then
    lualine.setup({ options = { theme = config.lualine[theme] } })
  end
  
  -- Call user callback
  if config.onchange then
    config.onchange(theme)
  end
  
  state.current_theme = theme
end

-- Handle incoming data from socket
local function on_socket_data(data)
  for _, line in ipairs(data) do
    local theme = trim(line)
    if theme == "light" or theme == "dark" then
      vim.schedule(function()
        apply_theme(theme)
      end)
    end
  end
end

-- Connect to theme-switcher socket
local function connect()
  local socket_path = find_socket_path()
  if not socket_path then
    vim.notify("theme-switcher socket not found. Is the daemon running with --ipc?", vim.log.levels.WARN)
    return false
  end
  
  state.socket_path = socket_path
  
  -- Use netcat to connect to Unix socket
  local cmd = string.format("nc -U %s", socket_path)
  
  state.job_id = vim.fn.jobstart(cmd, {
    on_stdout = function(_, data, _)
      on_socket_data(data)
    end,
    on_exit = function(_, exit_code, _)
      state.connected = false
      state.job_id = nil
      if exit_code ~= 0 then
        vim.notify("theme-switcher connection lost", vim.log.levels.WARN)
      end
    end,
  })
  
  if state.job_id > 0 then
    state.connected = true
    return true
  else
    vim.notify("Failed to connect to theme-switcher", vim.log.levels.ERROR)
    return false
  end
end

-- Disconnect from socket
local function disconnect()
  if state.job_id then
    vim.fn.jobstop(state.job_id)
    state.job_id = nil
    state.connected = false
  end
end

-- Stop the plugin
function M.stop()
  disconnect()
  state.initialized = false
end

-- Configure the plugin
function M.configure(config)
  state.config = vim.tbl_deep_extend("force", default_config, config or {})
end

-- Initialize and start the plugin
function M.setup(config)
  M.configure(config)
  
  -- Apply initial theme from early detection to prevent flash
  local initial_theme = vim.g.theme_switcher_initial_theme
  if initial_theme and (initial_theme == "light" or initial_theme == "dark") then
    -- Apply theme immediately without scheduling
    apply_theme(initial_theme)
  end
  
  -- Connect to socket
  if connect() then
    state.initialized = true
    
    -- Set up autocmd to clean up on exit
    vim.api.nvim_create_autocmd("VimLeave", {
      pattern = "*",
      callback = function()
        M.stop()
      end,
    })
  end
end

-- Manual theme update (for testing)
function M.set_theme(theme)
  if theme == "light" or theme == "dark" then
    apply_theme(theme)
  else
    vim.notify("Theme must be 'light' or 'dark'", vim.log.levels.ERROR)
  end
end

-- Get current theme
function M.get_theme()
  return state.current_theme
end

-- Toggle theme
function M.toggle()
  local current = state.current_theme or "dark"
  local new_theme = current == "light" and "dark" or "light"
  M.set_theme(new_theme)
end

return M