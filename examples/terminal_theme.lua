-- Example: Change terminal theme based on system theme
-- This example works with iTerm2 on macOS

local function change_iterm_theme(theme_name)
    -- iTerm2 AppleScript to change the color preset
    local script = string.format([[
        tell application "iTerm2"
            tell current session of current window
                set color preset to "%s"
            end tell
        end tell
    ]], theme_name)
    
    local result = theme_switcher.execute("osascript -e '" .. script .. "'")
    
    if result.success then
        theme_switcher.log("iTerm2 theme changed to: " .. theme_name)
    else
        theme_switcher.log_error("Failed to change iTerm2 theme: " .. result.stderr)
    end
end

local function change_terminal_app_theme(theme_name)
    -- macOS Terminal.app AppleScript
    local script = string.format([[
        tell application "Terminal"
            set current settings of first window to settings set "%s"
        end tell
    ]], theme_name)
    
    local result = theme_switcher.execute("osascript -e '" .. script .. "'")
    
    if result.success then
        theme_switcher.log("Terminal.app theme changed to: " .. theme_name)
    else
        -- Terminal.app might not be running
        if result.stderr:find("is not running") then
            theme_switcher.log("Terminal.app is not running")
        else
            theme_switcher.log_error("Failed to change Terminal.app theme: " .. result.stderr)
        end
    end
end

-- Change themes based on system theme
if IS_DARK then
    -- Dark themes
    change_iterm_theme("Solarized Dark")
    change_terminal_app_theme("Pro")
else
    -- Light themes
    change_iterm_theme("Solarized Light")
    change_terminal_app_theme("Basic")
end

-- You can also update other terminal emulators
-- Example for Alacritty (updates config file)
local alacritty_config = os.getenv("HOME") .. "/.config/alacritty/alacritty.yml"
if IS_DARK then
    theme_switcher.execute("sed -i '' 's/colors: .*/colors: *dark/' " .. alacritty_config)
else
    theme_switcher.execute("sed -i '' 's/colors: .*/colors: *light/' " .. alacritty_config)
end